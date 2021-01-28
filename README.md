# Bow Sequence Generator

## Getting Started
1. Install Python
2. Clone the repo
3. Run `cargo build --release
4. Run the binary in Command Prompt (Windows) or the terminal (MacOS/Linux) with the proper arguments
    1. Windows Example:  `target/release/bowseq.exe 0 1`
    2. MacOs/Linux Example: `./target/release/bowseq 0 1`

## Bash Style Documentation and Arguments
```
Bow Sequence Generator 0.1.1

Usage:  bowseq <alpha_term> <beta_term> [options]

Options:
    -count <count>  Amount of rows you would like generated (defaults to 10)
    -type <string>  Representation of the numbers in the sequence (int, float, alphabeta)
    -rowFormat      Log the sequence with row formatting in rows.txt

    -minmax         Find the maximum and minimum values in the sequence
    -find <num>     Return the indexes where a specific value appears
    -mean           Return the average value of the entire sequence
```

## RAM Usage
Since the amount of terms generated is doubled for each additional row, a small amount of rows leads a large amount of RAM being used.  Here are some examples:
* 27 Rows : 4 GB
* 28 Rows : 8 GB
* 29 Rows : 16 GB
* 30 Rows : 32 GB
* etc...

These are all based on the use of 32-bit integers.  64-bit floats are used, so double the usage when making alpha and/or beta a decimal.
