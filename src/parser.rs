use crate::cube::Move;
use anyhow::{anyhow, Error};

const DELIMITER: char = ' ';

pub fn parse(moves: &str) -> Result<Vec<Move>, Error> {
    if moves.is_empty() {
        return Err(anyhow!("empty input"));
    }
    moves
        .trim()
        .split(DELIMITER)
        .filter(|s| !s.is_empty())
        .map(|s| match s {
            "U" | "u" => Ok(Move::U),
            "D" | "d" => Ok(Move::D),
            "F" | "f" => Ok(Move::F),
            "B" | "b" => Ok(Move::B),
            "L" | "l" => Ok(Move::L),
            "R" | "r" => Ok(Move::R),
            "M" | "m" => Ok(Move::M),
            "E" | "e" => Ok(Move::E),
            "S" | "s" => Ok(Move::S),
            "X" | "x" | "Y" | "y" | "Z" | "z" => Err(anyhow!("attempted to rotate cube (unsupported)")),
            other => Err(anyhow!("not a valid WCA move: {other}"))
        })
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_valid_single_moves() {
        for move_str in ["U", "u", "D", "d", "F", "f", "B", "b", "L", "l", "R", "r", "M", "m", "E", "e", "S", "s"] {
            let result = parse(move_str);
            assert!(result.is_ok(), "failed to parse: {move_str}");
        }
    }

    #[test]
    fn test_multiple_moves() {
        let result = parse("U R F");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), vec![Move::U, Move::R, Move::F]);
    }

    #[test]
    fn test_empty_input() {
        let result = parse("");
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_move() {
        let result = parse("X");
        assert!(result.is_err());
    }

    #[test]
    fn test_cube_rotation_error() {
        for move_str in ["X", "x", "Y", "y", "Z", "z"] {
            let result = parse(move_str);
            assert!(result.is_err());
            let err = result.unwrap_err().to_string();
            assert!(err.contains("rotate cube"), "wrong error for {move_str}: {err}");
        }
    }

    #[test]
    fn test_mixed_valid_invalid() {
        let result = parse("U A F");
        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(err.contains("not a valid WCA move"));
    }
}
