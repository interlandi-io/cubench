use anyhow::{Error, anyhow};
use std::fmt::{Display, Formatter};

const KNUTH: u64 = 0x9E37_79B9_7F4A_7C15;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Color {
    W,
    Y,
    G,
    B,
    O,
    R,
}

impl Color {
    pub fn index(self) -> usize {
        match self {
            Self::W => 0,
            Self::Y => 1,
            Self::G => 2,
            Self::B => 3,
            Self::O => 4,
            Self::R => 5,
        }
    }
}

pub type Face = [[Color; 3]; 3];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Clockwise,
    Prime,
    Double,
}

impl Direction {
    fn turns(self) -> usize {
        match self {
            Self::Clockwise => 1,
            Self::Prime => 3,
            Self::Double => 2,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CubeState {
    pub u: Face,
    pub d: Face,
    pub f: Face,
    pub b: Face,
    pub l: Face,
    pub r: Face,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Cube {
    pub state: CubeState,
}

pub fn create_face(color: Color) -> Face {
    [[color; 3]; 3]
}

pub fn rotate_face_matrix_cw(face: Face) -> Face {
    let mut rotated = face;

    for (i_row, row) in face.iter().enumerate() {
        for (i_col, color) in row.iter().enumerate() {
            // TODO :Remember to change this once this becomes nxn agnostic
            rotated[i_col][2 - i_row] = *color;
        }
    }

    rotated
}

fn get_row(face: &Face, row: usize) -> [Color; 3] {
    face[row]
}

fn set_row(face: &mut Face, row: usize, values: [Color; 3]) {
    face[row] = values;
}

fn get_col(face: &Face, col: usize) -> [Color; 3] {
    [face[0][col], face[1][col], face[2][col]]
}

fn set_col(face: &mut Face, col: usize, values: [Color; 3]) {
    for row in 0..3 {
        face[row][col] = values[row];
    }
}

fn reverse_vec3_color(values: [Color; 3]) -> [Color; 3] {
    [values[2], values[1], values[0]]
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Move {
    // Rotations
    U(Direction),
    D(Direction),
    F(Direction),
    B(Direction),
    L(Direction),
    R(Direction),
    // Slices
    M(Direction),
    E(Direction),
    S(Direction),
}

impl Cube {
    pub const FACELET_DISPLAY_ORDER: &'static str =
        "UUUUUUUUURRRRRRRRRFFFFFFFFFDDDDDDDDDLLLLLLLLLBBBBBBBBB";

    pub fn new_solved() -> Self {
        Self {
            state: CubeState {
                u: create_face(Color::W),
                d: create_face(Color::Y),
                f: create_face(Color::G),
                b: create_face(Color::B),
                l: create_face(Color::O),
                r: create_face(Color::R),
            },
        }
    }

    pub fn r#move(&mut self, r#move: &Move) {
        let dir = match r#move {
            Move::U(d) => d,
            Move::D(d) => d,
            Move::F(d) => d,
            Move::B(d) => d,
            Move::L(d) => d,
            Move::R(d) => d,
            Move::M(d) => d,
            Move::E(d) => d,
            Move::S(d) => d,
        };

        for _ in 0..dir.turns() {
            match r#move {
                Move::U(_) => self.rotate_u_cw(),
                Move::D(_) => self.rotate_d_cw(),
                Move::F(_) => self.rotate_f_cw(),
                Move::B(_) => self.rotate_b_cw(),
                Move::L(_) => self.rotate_l_cw(),
                Move::R(_) => self.rotate_r_cw(),
                Move::M(_) => self.slice_m_cw(),
                Move::E(_) => self.slice_e_cw(),
                Move::S(_) => self.slice_s_cw(),
            }
        }
    }

    pub fn move_batch(&mut self, moves: &[Move]) {
        for r#move in moves {
            self.r#move(r#move);
        }
    }

    pub fn is_solved(&self) -> bool {
        [
            &self.state.u,
            &self.state.d,
            &self.state.f,
            &self.state.b,
            &self.state.l,
            &self.state.r,
        ]
        .into_iter()
        .all(|face| face_is_uniform(face))
    }

    pub fn scramble(&mut self, n: usize, seed: Option<u64>) -> Vec<Move> {
        let mut moves = Vec::new();
        let mut rng = seed.unwrap_or(KNUTH).max(1);

        for _ in 0..n {
            let direction = match xorshiftstar(&mut rng) % 3 {
                0 => Direction::Clockwise,
                1 => Direction::Prime,
                _ => Direction::Double,
            };

            let r#move = match xorshiftstar(&mut rng) % 6 {
                0 => Move::U(direction),
                1 => Move::D(direction),
                2 => Move::F(direction),
                3 => Move::B(direction),
                4 => Move::L(direction),
                _ => Move::R(direction),
            };

            moves.push(r#move);
            self.r#move(&r#move);
        }

        moves
    }

    // Each rotation can be represented as a clockwise matrix rotation about (1, 1),
    // followed by a blitting of the rows/cols accordingly.
    // For example, a standard U can be represented as a rotation of the u matrix about its center,
    // then copying the top row on each face to its counterpart 90deg clockwise.

    fn rotate_u_cw(&mut self) {
        self.state.u = rotate_face_matrix_cw(self.state.u);

        let front = get_row(&self.state.f, 0);
        let right = get_row(&self.state.r, 0);
        let back = get_row(&self.state.b, 0);
        let left = get_row(&self.state.l, 0);

        set_row(&mut self.state.f, 0, right);
        set_row(&mut self.state.l, 0, front);
        set_row(&mut self.state.b, 0, left);
        set_row(&mut self.state.r, 0, back);
    }

    fn rotate_d_cw(&mut self) {
        self.state.d = rotate_face_matrix_cw(self.state.d);

        let front = get_row(&self.state.f, 2);
        let left = get_row(&self.state.l, 2);
        let back = get_row(&self.state.b, 2);
        let right = get_row(&self.state.r, 2);

        set_row(&mut self.state.l, 2, back);
        set_row(&mut self.state.b, 2, right);
        set_row(&mut self.state.r, 2, front);
        set_row(&mut self.state.f, 2, left);
    }

    fn rotate_f_cw(&mut self) {
        self.state.f = rotate_face_matrix_cw(self.state.f);

        let up = get_row(&self.state.u, 2);
        let left = get_col(&self.state.l, 2);
        let down = get_row(&self.state.d, 0);
        let right = get_col(&self.state.r, 0);

        set_col(&mut self.state.r, 0, up);
        set_row(&mut self.state.d, 0, reverse_vec3_color(right));
        set_col(&mut self.state.l, 2, down);
        set_row(&mut self.state.u, 2, reverse_vec3_color(left));
    }

    fn rotate_b_cw(&mut self) {
        self.state.b = rotate_face_matrix_cw(self.state.b);

        let up = get_row(&self.state.u, 0);
        let right = get_col(&self.state.r, 2);
        let down = get_row(&self.state.d, 2);
        let left = get_col(&self.state.l, 0);

        set_col(&mut self.state.r, 2, down);
        set_row(&mut self.state.d, 2, reverse_vec3_color(left));
        set_col(&mut self.state.l, 0, up);
        set_row(&mut self.state.u, 0, reverse_vec3_color(right));
    }

    fn rotate_l_cw(&mut self) {
        self.state.l = rotate_face_matrix_cw(self.state.l);

        let up = get_col(&self.state.u, 0);
        let back = get_col(&self.state.b, 2);
        let down = get_col(&self.state.d, 0);
        let front = get_col(&self.state.f, 0);

        set_col(&mut self.state.f, 0, up);
        set_col(&mut self.state.d, 0, front);
        set_col(&mut self.state.b, 2, reverse_vec3_color(down));
        set_col(&mut self.state.u, 0, reverse_vec3_color(back));
    }

    fn rotate_r_cw(&mut self) {
        self.state.r = rotate_face_matrix_cw(self.state.r);

        let up = get_col(&self.state.u, 2);
        let front = get_col(&self.state.f, 2);
        let down = get_col(&self.state.d, 2);
        let back = get_col(&self.state.b, 0);

        set_col(&mut self.state.f, 2, up);
        set_col(&mut self.state.d, 2, front);
        set_col(&mut self.state.b, 0, reverse_vec3_color(down));
        set_col(&mut self.state.u, 2, reverse_vec3_color(back));
    }

    fn slice_m_cw(&mut self) {
        let up = get_col(&self.state.u, 1);
        let front = get_col(&self.state.f, 1);
        let down = get_col(&self.state.d, 1);
        let back = get_col(&self.state.b, 1);

        set_col(&mut self.state.b, 1, reverse_vec3_color(up));
        set_col(&mut self.state.d, 1, reverse_vec3_color(back));
        set_col(&mut self.state.f, 1, down);
        set_col(&mut self.state.u, 1, front);
    }

    fn slice_e_cw(&mut self) {
        let front = get_row(&self.state.f, 1);
        let right = get_row(&self.state.r, 1);
        let back = get_row(&self.state.b, 1);
        let left = get_row(&self.state.l, 1);

        set_row(&mut self.state.l, 1, back);
        set_row(&mut self.state.b, 1, right);
        set_row(&mut self.state.r, 1, front);
        set_row(&mut self.state.f, 1, left);
    }

    fn slice_s_cw(&mut self) {
        let up = get_row(&self.state.u, 2);
        let right = get_col(&self.state.r, 1);
        let down = get_row(&self.state.d, 0);
        let left = get_col(&self.state.l, 1);

        set_col(&mut self.state.r, 1, up);
        set_row(&mut self.state.d, 0, reverse_vec3_color(right));
        set_col(&mut self.state.l, 1, down);
        set_row(&mut self.state.u, 2, reverse_vec3_color(left));
    }
}

impl Display for Cube {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let faces = [
            &self.state.u,
            &self.state.r,
            &self.state.f,
            &self.state.d,
            &self.state.l,
            &self.state.b,
        ];
        for face in faces {
            for row in face {
                for color in row {
                    f.write_str(&color.to_string())?;
                }
            }
        }

        Ok(())
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Color::W => "W",
            Color::Y => "Y",
            Color::G => "G",
            Color::B => "B",
            Color::O => "O",
            Color::R => "R",
        };
        f.write_str(s)?;

        Ok(())
    }
}

impl Display for Move {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Move::U(Direction::Clockwise) => "U",
            Move::U(Direction::Prime) => "U'",
            Move::U(Direction::Double) => "U2",
            Move::D(Direction::Clockwise) => "D",
            Move::D(Direction::Prime) => "D'",
            Move::D(Direction::Double) => "D2",
            Move::F(Direction::Clockwise) => "F",
            Move::F(Direction::Prime) => "F'",
            Move::F(Direction::Double) => "F2",
            Move::B(Direction::Clockwise) => "B",
            Move::B(Direction::Prime) => "B'",
            Move::B(Direction::Double) => "B2",
            Move::L(Direction::Clockwise) => "L",
            Move::L(Direction::Prime) => "L'",
            Move::L(Direction::Double) => "L2",
            Move::R(Direction::Clockwise) => "R",
            Move::R(Direction::Prime) => "R'",
            Move::R(Direction::Double) => "R2",
            Move::M(Direction::Clockwise) => "M",
            Move::M(Direction::Prime) => "M'",
            Move::M(Direction::Double) => "M2",
            Move::E(Direction::Clockwise) => "E",
            Move::E(Direction::Prime) => "E'",
            Move::E(Direction::Double) => "E2",
            Move::S(Direction::Clockwise) => "S",
            Move::S(Direction::Prime) => "S'",
            Move::S(Direction::Double) => "S2",
        };
        f.write_str(s)?;

        Ok(())
    }
}

impl TryFrom<&str> for Move {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "u" => Ok(Move::U(Direction::Clockwise)),
            "u'" => Ok(Move::U(Direction::Prime)),
            "u2" => Ok(Move::U(Direction::Double)),
            "d" => Ok(Move::D(Direction::Clockwise)),
            "d'" => Ok(Move::D(Direction::Prime)),
            "d2" => Ok(Move::D(Direction::Double)),
            "f" => Ok(Move::F(Direction::Clockwise)),
            "f'" => Ok(Move::F(Direction::Prime)),
            "f2" => Ok(Move::F(Direction::Double)),
            "b" => Ok(Move::B(Direction::Clockwise)),
            "b'" => Ok(Move::B(Direction::Prime)),
            "b2" => Ok(Move::B(Direction::Double)),
            "l" => Ok(Move::L(Direction::Clockwise)),
            "l'" => Ok(Move::L(Direction::Prime)),
            "l2" => Ok(Move::L(Direction::Double)),
            "r" => Ok(Move::R(Direction::Clockwise)),
            "r'" => Ok(Move::R(Direction::Prime)),
            "r2" => Ok(Move::R(Direction::Double)),
            "m" => Ok(Move::M(Direction::Clockwise)),
            "m'" => Ok(Move::M(Direction::Prime)),
            "m2" => Ok(Move::M(Direction::Double)),
            "e" => Ok(Move::E(Direction::Clockwise)),
            "e'" => Ok(Move::E(Direction::Prime)),
            "e2" => Ok(Move::E(Direction::Double)),
            "s" => Ok(Move::S(Direction::Clockwise)),
            "s'" => Ok(Move::S(Direction::Prime)),
            "s2" => Ok(Move::S(Direction::Double)),
            "x" | "y" | "z" => Err(anyhow!("attempted to rotate cube (unsupported)")),
            other => Err(anyhow!("not a valid WCA move: {other}")),
        }
    }
}

fn face_is_uniform(face: &Face) -> bool {
    let anchor = face[0][0];
    face.iter().flatten().all(|&color| color == anchor)
}

fn xorshiftstar(state: &mut u64) -> u64 {
    let mut x = *state;
    x ^= x >> 12;
    x ^= x << 25;
    x ^= x >> 27;
    *state = x;
    x.wrapping_mul(0x2545_F491_4F6C_DD1D)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_face_fills_all_positions() {
        assert_eq!(create_face(Color::G), [[Color::G; 3]; 3]);
    }

    #[test]
    fn rotate_face_matrix_cw_rotates_correctly() {
        let face = [
            [Color::W, Color::Y, Color::G],
            [Color::B, Color::O, Color::R],
            [Color::Y, Color::G, Color::B],
        ];

        let rotated = rotate_face_matrix_cw(face);

        assert_eq!(
            rotated,
            [
                [Color::Y, Color::B, Color::W],
                [Color::G, Color::O, Color::Y],
                [Color::B, Color::R, Color::G],
            ]
        );
    }

    #[test]
    fn four_face_rotations_restore_original_face() {
        let face = [
            [Color::W, Color::Y, Color::G],
            [Color::B, Color::O, Color::R],
            [Color::Y, Color::G, Color::B],
        ];

        let rotated = rotate_face_matrix_cw(rotate_face_matrix_cw(rotate_face_matrix_cw(
            rotate_face_matrix_cw(face),
        )));

        assert_eq!(rotated, face);
    }

    #[test]
    fn fresh_cube_is_solved() {
        let cube = Cube::new_solved();
        assert!(cube.is_solved());
    }

    #[test]
    fn single_move_makes_cube_unsolved() {
        let mut cube = Cube::new_solved();
        cube.r#move(&Move::U(Direction::Clockwise));
        assert!(!cube.is_solved());
    }

    #[test]
    fn move_followed_by_inverse_restores_cube() {
        let mut cube = Cube::new_solved();
        let original = cube.clone();

        cube.move_batch(&[Move::F(Direction::Clockwise), Move::F(Direction::Prime)]);

        assert_eq!(cube, original);
    }

    #[test]
    fn double_move_twice_restores_cube() {
        let mut cube = Cube::new_solved();
        let original = cube.clone();

        cube.move_batch(&[Move::R(Direction::Double), Move::R(Direction::Double)]);

        assert_eq!(cube, original);
    }

    #[test]
    fn four_clockwise_turns_restore_each_face_move() {
        for r#move in [
            Move::U(Direction::Clockwise),
            Move::D(Direction::Clockwise),
            Move::F(Direction::Clockwise),
            Move::B(Direction::Clockwise),
            Move::L(Direction::Clockwise),
            Move::R(Direction::Clockwise),
        ] {
            let mut cube = Cube::new_solved();
            let original = cube.clone();

            cube.move_batch(&[r#move, r#move, r#move, r#move]);

            assert_eq!(cube, original);
        }
    }

    #[test]
    fn batch_moves_match_separate_calls() {
        let mut batch = Cube::new_solved();
        let mut separate = Cube::new_solved();

        batch.move_batch(&[
            Move::U(Direction::Clockwise),
            Move::R(Direction::Prime),
            Move::F(Direction::Double),
        ]);

        separate.r#move(&Move::U(Direction::Clockwise));
        separate.r#move(&Move::R(Direction::Prime));
        separate.r#move(&Move::F(Direction::Double));

        assert_eq!(batch, separate);
    }

    #[test]
    fn scramble_zero_keeps_cube_solved() {
        let mut cube = Cube::new_solved();
        cube.scramble(0, None);
        assert!(cube.is_solved());
    }

    #[test]
    fn scramble_preserves_color_counts() {
        let mut cube = Cube::new_solved();
        let before = color_counts(&cube);

        cube.scramble(64, None);

        assert_eq!(before, color_counts(&cube));
    }

    #[test]
    fn four_slices_restores_faces() {
        for r#move in [
            Move::M(Direction::Clockwise),
            Move::E(Direction::Clockwise),
            Move::S(Direction::Clockwise),
        ] {
            let mut cube = Cube::new_solved();
            let original = cube.clone();

            cube.move_batch(&[r#move, r#move, r#move, r#move]);

            assert_eq!(cube, original);
        }
    }

    #[test]
    fn to_string_works() {
        let cube = Cube::new_solved();
        let should = Cube::FACELET_DISPLAY_ORDER
            .replace("U", "W")
            .replace("R", "R")
            .replace("F", "G")
            .replace("D", "Y")
            .replace("L", "O")
            .replace("B", "B");
        assert_eq!(cube.to_string(), should);
    }

    fn color_counts(cube: &Cube) -> [usize; 6] {
        let mut counts = [0; 6];

        for face in [
            &cube.state.u,
            &cube.state.d,
            &cube.state.f,
            &cube.state.b,
            &cube.state.l,
            &cube.state.r,
        ] {
            for color in face.iter().flatten() {
                counts[color.index()] += 1;
            }
        }

        counts
    }
}
