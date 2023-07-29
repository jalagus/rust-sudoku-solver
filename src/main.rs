use std::collections::HashSet;
use std::io::{self, BufRead};

use structopt::StructOpt;
use rand::seq::SliceRandom;

struct Sudoku {
    items: [[u32; 9]; 9]
}

#[derive(StructOpt)]
struct Options {
    #[structopt(short = "r", long = "randomize")]
    /// Randomize the tree traversal order such that in case of multiple solutions returns a random one
    randomize: bool,

    #[structopt(short = "c", long = "count")]
    /// Count the possible solutions
    count: bool

}

impl std::fmt::Display for Sudoku {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut out = String::new();
        for (i, row) in self.items.iter().enumerate() {
            for i in 0..9 {
                out.push_str(format!(" {}", row[i]).as_str());
                if (i + 1) % 3 == 0 && i < 8 {
                    out.push_str(" |");
                }
            }
            out.push_str("\n");
            if (i + 1) % 3 == 0 && i < 8 {
                out.push_str("-----------------------\n");
            }
        }
        write!(f, "{}", out)
    }
}

fn flatten(block: &[[u32; 3]; 3]) -> [u32; 9] {
    let mut flattened_array: [u32; 9] = [0; 9];

    let mut index = 0;
    for row in block {
        for &element in row {
            flattened_array[index] = element;
            index += 1;
        }
    }

    flattened_array
}

fn check_row(row: &[u32; 9]) -> bool {
    let mut seen = HashSet::new();
    for item in row {
        if seen.contains(item) {
            return false;
        }
        if *item != 0 {
            seen.insert(item);
        }
    }
    return true
}

fn check_block(block: &[[u32; 3]; 3]) -> bool {
    check_row(&flatten(block))
}

fn check_column(col: &[u32; 9]) -> bool {
    check_row(col)
}

fn check_solution(sudoku: &Sudoku) -> bool {
    for row in sudoku.items {
        if !check_row(&row) {
            return false;
        }
    }
    for col_i in 0..9 {
        let col: [u32; 9] = sudoku.items
            .iter()
            .map(|row| row[col_i])
            .collect::<Vec<_>>().try_into().unwrap();
        if !check_column(&col) {
            return false;
        }
    }
    for block_i in 0..9 {
        let (i_dis, j_dis) = match block_i {
            0 => (0, 0),
            1 => (0, 3),
            2 => (0, 6),
            3 => (3, 0),
            4 => (3, 3),
            5 => (3, 6),
            6 => (6, 0),
            7 => (6, 3),
            8 => (6, 6),
            _ => (100, 100)
        };

        let mut values = [[0_u32; 3]; 3];
        for i in 0..3 {
            for j in 0..3 {
                values[i][j] = sudoku.items[i + i_dis][j + j_dis];
            }
        }
        if !check_block(&values) {
            return false;
        }
    }

    true
}

fn find_empty_position(sudoku: &Sudoku) -> Option<(usize, usize)> {
    for (i, row) in sudoku.items.iter().enumerate() {
        for (j, _) in row.iter().enumerate() {
            if sudoku.items[i][j] == 0 {
                return Some((i, j))
            }
        }
    }
    return None
}

fn dfs(sudoku: Sudoku, randomize: bool) -> Option<Sudoku> {
    let (start_i, start_j) = match find_empty_position(&sudoku) {
        Some(x) => x,
        None => return Some(Sudoku { items: sudoku.items.clone() })
    };
    
    let mut guesses: Vec<u32> = (1..10).collect();

    if randomize {
        guesses.shuffle(&mut rand::thread_rng());
    }

    for guess in guesses.iter() {
        let mut new_items = sudoku.items.clone();
        new_items[start_i][start_j] = *guess;
        let new_solution = Sudoku { items: new_items };
        if check_solution(&new_solution) {
            let res = dfs(new_solution, randomize);
            if res.is_some() {
                return Some(res.unwrap());
            }
        }
    }
    
    None
}

fn find_all_dfs(sudoku: Sudoku) -> u32 {
    let (start_i, start_j) = match find_empty_position(&sudoku) {
        Some(x) => x,
        None => return 1
    };

    let mut n_solution = 0;

    for guess in 1..10 {
        let mut new_items = sudoku.items.clone();
        new_items[start_i][start_j] = guess;
        let new_solution = Sudoku { items: new_items };
        if check_solution(&new_solution) {
            n_solution += find_all_dfs(new_solution);
        }
    }

    n_solution
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
    let sudoku = Sudoku {
        items: sudoku_items
    };

    println!("Initial sudoku:\n{}", sudoku);

    if options.count {
        println!("Counting all possible solutions.");
        println!("Number of solutions: {}", find_all_dfs(sudoku));
    }
    else {
        let result = dfs(sudoku, options.randomize);
        match result {
            Some(x) => println!("Solution:\n{}", x),
            None => println!("No solution")
        }
    }
}
