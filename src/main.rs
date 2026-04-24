use cubench::cube::Cube;
use cubench::parser::parse;
use anyhow::Error;
use std::io;

fn main() -> Result<(), Error> {
    let mut cube = Cube::new_solved();
    cube.scramble(128);
    let mut solution_buf = String::new();

    println!("{}", cube.to_string());

    println!("Enter solution");
    io::stdin().read_line(&mut solution_buf)?;

    let solution_moves = parse(&solution_buf)?;

    for r#move in solution_moves {
        cube.r#move(r#move);
    }

    println!("Cube state: {cube}");
    println!("Solved: {}", cube.is_solved());

    Ok(())
}
