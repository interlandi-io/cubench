use crate::{cube::Cube, parser::parse};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct CubeShim(Cube);

#[wasm_bindgen]
impl CubeShim {
    #[wasm_bindgen(constructor)]
    pub fn new_solved() -> Self {
        console_error_panic_hook::set_once();
        CubeShim(Cube::new_solved())
    }

    pub fn apply_moves(&mut self, moves: &str) -> Result<(), JsError> {
        let moves_dec = parse(moves).map_err(|e| JsError::new(e.to_string().as_str()))?;
        self.0.move_batch(&moves_dec);

        Ok(())
    }

    pub fn look(&self) -> String {
        self.0.to_string()
    }

    pub fn is_solved(&self) -> bool {
        self.0.is_solved()
    }

    pub fn scramble(&mut self, seed: Option<u64>) {
        self.0.scramble(25, seed);
    }
}
