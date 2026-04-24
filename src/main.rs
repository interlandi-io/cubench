use cubench::cube::Cube;
use cubench::parser::parse;
use anyhow::Error;
use std::io;

fn main() -> Result<(), Error> {
    let mut cube = Cube::new_solved();
    let mut solution_buf = String::new();
    let scramble = cube.scramble(25)
        .iter()
        .map(|m| m.to_string())
        .collect::<Vec<String>>() // TODO: need 
        .join(" ");

    println!("State: {}", cube.to_string());
    println!("Scramble: {scramble}");

    println!("Enter solution:");
    io::stdin().read_line(&mut solution_buf)?;

    let solution_moves = parse(&solution_buf)?;

    for r#move in solution_moves {
        cube.r#move(r#move);
    }

    println!("Cube state: {cube}");
    println!("Solved: {}", cube.is_solved());

    Ok(())
}
