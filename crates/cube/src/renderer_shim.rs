use crate::renderer::{Renderer};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct RendererShim(Renderer);

#[wasm_bindgen]
impl RendererShim {
    /// js doesn't support async constructors itself,
    /// so annotating this with `#[wasm_bindgen(constructor)]`
    /// would be invalid.
    /// See https://github.com/wasm-bindgen/wasm-bindgen/issues/3976
    pub async fn create() -> Self {
        console_error_panic_hook::set_once();
        RendererShim(Renderer::new().await)
    }

    pub async fn draw_once(&mut self) {
        self.0.draw_once().await
    }
    
    /// The rendered image will be stale unless you call `Renderer::draw_once` first.
    pub fn write_once_to_buf(&self) -> Vec<u8> {
        self.0.write_once_to_buf()
    }
}
