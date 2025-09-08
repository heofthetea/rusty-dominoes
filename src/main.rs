// okay rust is beyonhd stupid when it comes to project filesystem management I have accepted it and will not question it again

use crate::generate::choose_random;
use crate::solve::solve;
use crate::solve_claude::solve_claude;
use std::io;

mod generate;
mod solve;
mod solve_claude;
mod types;

fn main() {
    let k = get_num();
    let dominoes = choose_random(k as usize);
    println!("{:?}", dominoes);
    println!("{:?}", solve(&dominoes));
    println!("{:?} (claude's solution)", solve_claude(&dominoes))
}

fn get_num() -> u32 {
    println!("Enter number of dominoes:");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    return input.trim().parse().expect("Not a number");
}
