use std::io::{self, BufRead};
use structopt::StructOpt;

use crate::sudoku::Solvable;

mod sudoku;

#[derive(StructOpt)]
struct Options {
    #[structopt(short = "r", long = "randomize")]
    /// Randomize the tree traversal order such that in case of multiple solutions returns a random one
    randomize: bool,

    #[structopt(short = "c", long = "count")]
    /// Count the possible solutions
    count: bool

}

fn parse_user_input(input: String) -> Option<[[u32; 9]; 9]> {
    let mut all_vals = vec![];
    for row in input.split("\n") {
        for item in row.split(",") {
            all_vals.push(item.trim().parse::<u32>());
        }
    }
    let asd: Vec<u32> = all_vals.iter()
        .filter(|x| x.is_ok())
        .map(|x| x.clone().unwrap())
        .collect();

    let mut matrix = [[0u32; 9]; 9];
    if asd.len() == 9*9 {
        for (i, item) in asd.iter().enumerate() {
            matrix[i / 9][i % 9] = *item;
        }
    }
    else {
        return None;
    }
    Some(matrix)
}

fn main() {
    let options = Options::from_args();

    let mut user_input = String::new();
    let reader = Box::new(io::BufReader::new(io::stdin()));

    for line in reader.lines() {
        if let Ok(line) = line {
            user_input.push_str(&line);
            user_input.push_str("\n");
        }
    }

    let sudoku_items = match parse_user_input(user_input) {
        Some(x) => {
            println!("Loaded succefully from stdin.");
            x
        },
        None => {
            println!("Failed to load from stdin using default empty sudoku.");
            [
                [0,0,0, 0,0,0, 0,0,0],
                [0,0,0, 0,0,0, 0,0,0],
                [0,0,0, 0,0,0, 0,0,0],

                [0,0,0, 0,0,0, 0,0,0],
                [0,0,0, 0,0,0, 0,0,0],
                [0,0,0, 0,0,0, 0,0,0],

                [0,0,0, 0,0,0, 0,0,0],
                [0,0,0, 0,0,0, 0,0,0],
                [0,0,0, 0,0,0, 0,0,0],
            ]
        }
    };
    let sudoku = sudoku::Sudoku9x9 {
        items: sudoku_items
    };

    println!("Initial sudoku:\n{}", sudoku);

    if options.count {
        println!("Counting all possible solutions.");
        println!("Number of solutions: {}", sudoku.count_solutions());
    }
    else {
        let result = match options.randomize {
            true => sudoku.random_solution(),
            false => sudoku.solve()
        };
        match result {
            Some(x) => println!("Solution:\n{}", x),
            None => println!("No solution")
        }
    }
}
