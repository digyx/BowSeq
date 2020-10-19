# Bow Sequence Generator

## Getting Started
1. Install Python
2. Clone the repo
3. Run the binary in Command Prompt (Windows) or the terminal (MacOS/Linux) with the proper arguments
    1. Windows Example:  `BowSeq.exe 0 1`
    2. MacOs/Linux Example: `./BowSeq 0 1`

## Bash Style Documentation and Arguments
```
Bow Sequence Generator 0.1.0

Usage:  BowSeq alpha_term beta_term [options]

Options:
    -count <count>  Amount of rows you would like generated
    -type <string>  Representation of the numbers in the sequence (float, alphabeta)
    -minmax         Find the maximum and minimum values in the sequence
    -find <float>   Return the indexes where a specific value appears
    -rowFormat      Log the sequence with row formatting in rows.txt
```

## RAM Usage
Since the amount of terms generated is doubled for each additional row, a small amount of rows leads a large amount of RAM being used.  Here are some examples of how much memory is used for a specified amount of rows:
* 26 Rows : 2 GB
* 27 Rows : 4 GB
* 28 Rows : 8 GB
* 29 Rows : 16 GB
