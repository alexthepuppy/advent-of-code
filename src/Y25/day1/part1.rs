mod common;
use crate::common::*;

pub fn main() {
    let lines = INPUT_FILE.lines().collect();
    let result = solve(50, lines, ClickCountMode::EndOfOperation);
    println!("Zero crossings: {}", result)
}