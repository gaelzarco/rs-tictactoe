use tabled::builder::Builder;
use tabled::settings::Style;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    clear_screen();
    const TOTAL_ROWS: usize = 3;
    const TOTAL_COLS: usize = 3;

    let file = File::open("./src/logo.txt").expect("Error opening file");
    let reader = BufReader::new(file);
    for line in reader.lines() {
        println!("{}", line.expect("Could not read line"));
    }

    println!("Welcome to tic tac toe!");
    println!("This is a demo application built in Rust to demonstrate the po");

    let mut board = create_board(TOTAL_ROWS, TOTAL_COLS);

    println!("{:?}", board);
}

fn print_board() {

}

fn create_board(total_rows: usize, total_cols: usize) -> Vec<Vec<char>> {
    let mut matrix: Vec<Vec<char>> = Vec::new();
    for _ in 0..total_rows {
        let row: Vec<char> = vec![' '; total_cols];
        matrix.push(row)
    }
    return matrix
}

fn clear_screen() {
    print!("\x1B[2j\x1B[1;1H");
}