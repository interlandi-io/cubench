use crate::cube::{Move, Direction};
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
            "U" | "u" => Ok(Move::U(Direction::Clockwise)),
            "U'" | "u'" => Ok(Move::U(Direction::Prime)),
            "U2" | "u2" => Ok(Move::U(Direction::Double)),
            "D" | "d" => Ok(Move::D(Direction::Clockwise)),
            "D'" | "d'" => Ok(Move::D(Direction::Prime)),
            "D2" | "d2" => Ok(Move::D(Direction::Double)),
            "F" | "f" => Ok(Move::F(Direction::Clockwise)),
            "F'" | "f'" => Ok(Move::F(Direction::Prime)),
            "F2" | "f2" => Ok(Move::F(Direction::Double)),
            "B" | "b" => Ok(Move::B(Direction::Clockwise)),
            "B'" | "b'" => Ok(Move::B(Direction::Prime)),
            "B2" | "b2" => Ok(Move::B(Direction::Double)),
            "L" | "l" => Ok(Move::L(Direction::Clockwise)),
            "L'" | "l'" => Ok(Move::L(Direction::Prime)),
            "L2" | "l2" => Ok(Move::L(Direction::Double)),
            "R" | "r" => Ok(Move::R(Direction::Clockwise)),
            "R'" | "r'" => Ok(Move::R(Direction::Prime)),
            "R2" | "r2" => Ok(Move::R(Direction::Double)),
            "M" | "m" => Ok(Move::M(Direction::Clockwise)),
            "M'" | "m'" => Ok(Move::M(Direction::Prime)),
            "M2" | "m2" => Ok(Move::M(Direction::Double)),
            "E" | "e" => Ok(Move::E(Direction::Clockwise)),
            "E'" | "e'" => Ok(Move::E(Direction::Prime)),
            "E2" | "e2" => Ok(Move::E(Direction::Double)),
            "S" | "s" => Ok(Move::S(Direction::Clockwise)),
            "S'" | "s'" => Ok(Move::S(Direction::Prime)),
            "S2" | "s2" => Ok(Move::S(Direction::Double)),
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
        assert_eq!(result.unwrap(), vec![Move::U(Direction::Clockwise), Move::R(Direction::Clockwise), Move::F(Direction::Clockwise)]);
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
