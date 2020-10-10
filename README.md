# Bow Sequence Generator

## Getting Started
1. Install Rust
2. Install Python
3. Clone the repo
4. Run `cargo run` in the root of the repo

## RAM Usage
Since the amount of terms generated is doubled for each additional row, a small amount of rows leads a large amount of RAM being used.  Here are some examples of how much memory is used for a specified amount of rows:
* 26 Rows : 2 GB
* 27 Rows : 4 GB
* 28 Rows : 8 GB
* 29 Rows : 16 GB
