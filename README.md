# log stats

Prints number of messages with given `type` and their total size in bytes on disk, white characters and EOLs are counted.

Message is considered as a file's line with arbitrary json object, example:
```
{"type":"B","foo":"bar","items":["one","two"]}
```

If deserialization error occours then line is skipped and counted as *not classified*. Skipped lines can be printed by passing `show-skipped` flag.

## Prerequisites

* rustup (https://www.rust-lang.org/tools/install)


## Build

`cargo build --release`

## Run

`cargo run -- --help`

## Things to consider

* sorting results in table in alphabetical order
* failing fast if there is an error durring message processing