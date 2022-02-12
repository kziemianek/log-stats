use std::{collections::HashMap, fmt::Display};

use serde::Deserialize;

/// Represents log message
#[derive(Deserialize, Debug)]
pub struct Message<'a> {
    // this is mandatory, what should we do if it's missing ?
    #[serde(alias = "type")]
    pub message_type: &'a str,
}

/// Represents stats of messages
#[derive(Debug, Default)]
pub struct MessageStats {
    messages_count: u128,
    messages_total_size: u128,
}

impl MessageStats {
    fn add_single(&mut self, size: u128) {
        self.messages_count += 1;
        self.messages_total_size += size;
    }
}

/// Represents stats of messages with given type and not classified ones
#[derive(Default)]
pub struct AggregatedTypeStats {
    pub stats: HashMap<String, MessageStats>,
    pub not_classified: MessageStats,
}

impl AggregatedTypeStats {
    pub fn add_single_message_stats(&mut self, message_type: &str, size: u128) {
        // as specified, we can trim `message_type` str
        let trimmed_type = message_type.trim();
        if let Some(stats) = self.stats.get_mut(trimmed_type) {
            stats.add_single(size);
        } else {
            self.stats
                .insert(trimmed_type.to_owned(), MessageStats::default());
        }
    }

    pub fn add_not_classified(&mut self, size: u128) {
        self.not_classified.add_single(size)
    }
}

impl Display for AggregatedTypeStats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "{0: <10} | {1: <10} | {2: <10} ",
            "type", "count", "total size"
        )?;
        for (message_type, stats) in self.stats.iter() {
            writeln!(
                f,
                "{0: <10} | {1: <10} | {2: <10}",
                message_type, stats.messages_count, stats.messages_total_size
            )?;
        }
        if self.not_classified.messages_count > 0 {
            write!(
                f,
                "*** There were {} not counted messages with {} total size in bytes ***",
                self.not_classified.messages_count, self.not_classified.messages_total_size
            )?;
        }
        write!(f, "")
    }
}
