mod stats;

use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use clap::Parser;

use serde_json::Error;
use stats::{AggregatedTypeStats, Message};

/// Tool to show stats from log file
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long)]
    file: String,

    #[clap(short, long)]
    show_skipped: bool,
}

fn main() {
    let args = Args::parse();
    let file = File::open(args.file).expect("File could not be opened.");
    println!(
        "{}",
        calculate_stats(&file, args.show_skipped)
            .expect("There was a problem during stats calculation...")
    );
}

fn calculate_stats(file: &File, show_skipped: bool) -> Result<AggregatedTypeStats, Error> {
    let mut reader = BufReader::new(file);
    let line = &mut String::new();
    let mut stats = AggregatedTypeStats::default();
    let stats_ref = &mut stats;
    let mut line_num: u128 = 0;
    while let Ok(size) = reader.read_line(line) {
        line_num += 1;
        if size == 0 {
            break;
        }
        match serde_json::from_str(line) {
            Ok::<Message, Error>(deserialized) => {
                stats_ref.add_single_message_stats(deserialized.message_type, size as u128);
            }
            Err(err) => {
                if show_skipped {
                    println!("---");
                    println!("Skipping line #{} with content \"{}\" because of deserialization issue: {}", line_num, line, err);
                    println!("---");
                }
                stats_ref.add_not_classified(size as u128);
            }
        }
        line.clear();
    }
    Ok(stats)
}

#[cfg(test)]
mod tests {
    use std::fs::File;

    use crate::calculate_stats;

    #[test]
    fn test_calculate_stats_ok() {
        let file = File::open("samples/log.file").expect("File could not be opened.");
        calculate_stats(&file, false).expect("Stats should be calculated correctly");
    }
}
