use cubench::renderer::{Renderer, DrawResult};
use kiss3d::prelude::*;

#[kiss3d::main]
async fn main() -> Result<(), anyhow::Error> {
    let mut renderer = Renderer::new().await;
    let rot = Quat::from_axis_angle(Vec3::Y, 0.014);

    renderer.draw_loop(|scene| {
        scene.cube.rotate(rot);
        DrawResult::Continue
    }).await;
}
