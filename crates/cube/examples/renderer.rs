use cubench::renderer::Renderer;
use kiss3d::prelude::*;

#[kiss3d::main]
async fn main() -> Result<(), anyhow::Error> {
    let mut window = Window::new("").await;
    let mut camera = OrbitCamera3d::new(
        Vec3 {
            x: 15.0,
            y: 15.0,
            z: -15.0,
        },
        Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
    );
    let mut scene = SceneNode3d::empty();
    scene
        .add_light(Light::point(100.0))
        .set_position(Vec3::new(0.0, 10.0, 0.0));

    let mut cube = scene.add_group();
    for i in 0..=2 {
        for j in 0..=2 {
            for k in 0..=2 {
                if i == 1 && j == 1 && k == 1 {
                    continue;
                }
                let x = (1 - i) as f32 * 1.1;
                let y = (1 - j) as f32 * 1.1;
                let z = (1 - k) as f32 * 1.1;
                cube.add_cube(1.0, 1.0, 1.0).translate(Vec3 { x, y, z });
            }
        }
    }

    let rot = Quat::from_axis_angle(Vec3::Y, 0.014);

    while window.render_3d(&mut scene, &mut camera).await {
        cube.rotate(rot);
    }
}
