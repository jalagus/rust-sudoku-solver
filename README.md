# Sudoku solver in Rust

Reads a sudoku from stdin and outputs solution to stdout if solution is found. All 9 * 9 = 81 elements need to be provided or the parsing will fail and default to an empty sudoku. This will as a consequence provide a possible solution to an empty sudoku which can be used to create new possible sudokus.

Using the empty template provided, the program can be run using:
```
cat sudoku_template.txt | sudoku-solver
```