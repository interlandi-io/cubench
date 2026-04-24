use std::time::{SystemTime, UNIX_EPOCH};

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
    U,
    D,
    F,
    B,
    L,
    R,
    // Slices
    M,
    E,
    S,
}

impl Cube {
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

    pub fn u(&mut self, direction: Direction) -> &mut Self {
        self.apply_move(Move::U, direction)
    }

    pub fn d(&mut self, direction: Direction) -> &mut Self {
        self.apply_move(Move::D, direction)
    }

    pub fn f(&mut self, direction: Direction) -> &mut Self {
        self.apply_move(Move::F, direction)
    }

    pub fn b(&mut self, direction: Direction) -> &mut Self {
        self.apply_move(Move::B, direction)
    }

    pub fn l(&mut self, direction: Direction) -> &mut Self {
        self.apply_move(Move::L, direction)
    }

    pub fn r(&mut self, direction: Direction) -> &mut Self {
        self.apply_move(Move::R, direction)
    }

    pub fn m(&mut self, direction: Direction) -> &mut Self {
        self.apply_move(Move::M, direction)
    }

    pub fn e(&mut self, direction: Direction) -> &mut Self {
        self.apply_move(Move::E, direction)
    }

    pub fn s(&mut self, direction: Direction) -> &mut Self {
        self.apply_move(Move::S, direction)
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

    pub fn scramble(&mut self, n: usize) -> &mut Self {
        let seed = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|duration| duration.as_nanos() as u64)
            .unwrap_or(KNUTH);
        self.scramble_with_seed(n, seed)
    }

    pub fn scramble_with_seed(&mut self, n: usize, seed: u64) -> &mut Self {
        let mut rng = seed.max(1);

        for _ in 0..n {
            let face = match xorshiftstar(&mut rng) % 6 {
                0 => Move::U,
                1 => Move::D,
                2 => Move::F,
                3 => Move::B,
                4 => Move::L,
                _ => Move::R,
            };

            let direction = match xorshiftstar(&mut rng) % 3 {
                0 => Direction::Clockwise,
                1 => Direction::Prime,
                _ => Direction::Double,
            };

            self.apply_move(face, direction);
        }

        self
    }

    fn apply_move(&mut self, movement: Move, direction: Direction) -> &mut Self {
        for _ in 0..direction.turns() {
            self.apply_move_cw(movement);
        }
        self
    }

    fn apply_move_cw(&mut self, movement: Move) {
        match movement {
            Move::U => self.rotate_u_cw(),
            Move::D => self.rotate_d_cw(),
            Move::F => self.rotate_f_cw(),
            Move::B => self.rotate_b_cw(),
            Move::L => self.rotate_l_cw(),
            Move::R => self.rotate_r_cw(),
            Move::M => self.slice_m_cw(),
            Move::E => self.slice_e_cw(),
            Move::S => self.slice_s_cw(),
        }
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
    use super::{Color, Cube, Direction, create_face, rotate_face_matrix_cw};

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
        cube.u(Direction::Clockwise);
        assert!(!cube.is_solved());
    }

    #[test]
    fn move_followed_by_inverse_restores_cube() {
        let mut cube = Cube::new_solved();
        let original = cube.clone();

        cube.f(Direction::Clockwise)
            .f(Direction::Prime);

        assert_eq!(cube, original);
    }

    #[test]
    fn double_move_twice_restores_cube() {
        let mut cube = Cube::new_solved();
        let original = cube.clone();

        cube.r(Direction::Double).r(Direction::Double);

        assert_eq!(cube, original);
    }

    #[test]
    fn four_clockwise_turns_restore_each_face_move() {
        for apply in [
            Cube::u,
            Cube::d,
            Cube::f,
            Cube::b,
            Cube::l,
            Cube::r,
        ] {
            let mut cube = Cube::new_solved();
            let original = cube.clone();

            apply(&mut cube, Direction::Clockwise);
            apply(&mut cube, Direction::Clockwise);
            apply(&mut cube, Direction::Clockwise);
            apply(&mut cube, Direction::Clockwise);

            assert_eq!(cube, original);
        }
    }

    #[test]
    fn chainable_rotation_methods_match_separate_calls() {
        let mut chained = Cube::new_solved();
        let mut separate = Cube::new_solved();

        chained
            .u(Direction::Clockwise)
            .r(Direction::Prime)
            .f(Direction::Double);

        separate.u(Direction::Clockwise);
        separate.r(Direction::Prime);
        separate.f(Direction::Double);

        assert_eq!(chained, separate);
    }

    #[test]
    fn scramble_zero_keeps_cube_solved() {
        let mut cube = Cube::new_solved();
        cube.scramble_with_seed(0, 1234);
        assert!(cube.is_solved());
    }

    #[test]
    fn scramble_preserves_color_counts() {
        let mut cube = Cube::new_solved();
        let before = color_counts(&cube);

        cube.scramble_with_seed(64, 1234);

        assert_eq!(before, color_counts(&cube));
    }

    #[test]
    fn four_slices_restores_faces() {
        for apply in [
            Cube::m,
            Cube::e,
            Cube::s,
        ] {
            let mut cube = Cube::new_solved();
            let original = cube.clone();

            apply(&mut cube, Direction::Clockwise);
            apply(&mut cube, Direction::Clockwise);
            apply(&mut cube, Direction::Clockwise);
            apply(&mut cube, Direction::Clockwise);

            assert_eq!(cube, original);
        }
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
