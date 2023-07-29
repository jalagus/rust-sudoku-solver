# Sudoku solver in Rust

Reads a sudoku from stdin and outputs solution to stdout if solution is found. All 9 * 9 = 81 elements need to be provided or the parsing will fail and default to an empty sudoku. This will as a consequence provide a possible solution to an empty sudoku which can be used to generate new sudokus.

## How to run

Using the empty template provided, the program can be used to generate new sudokus by running
```
sudoku-solver -r < sudoku_template.txt
```

The `-r` flag indicates that we want to randomize the tree traversal which as a consequence will lead to multitude of solutions being generated (if the template sudoku has multiple possible solutions).