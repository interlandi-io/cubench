pub use image::{ImageBuffer, Rgb};
use kiss3d::prelude::*;
use std::sync::Arc;

pub struct Renderer {
    window: Window,
    scene: Scene,
}

#[allow(unused)]
pub struct Scene {
    pub camera: OrbitCamera3d,
    pub root: SceneNode3d,
    pub light: SceneNode3d,
    pub cube: SceneNode3d,
    pub cubelets: Vec<Cubelet>,
}

pub struct Cubelet {
    pub node: SceneNode3d,
    pub texture: Arc<Texture>,
}

pub enum DrawResult {
    Continue,
    Break,
}

impl Renderer {
    pub async fn new() -> Self {
        let window = Window::new("").await;
        let camera = OrbitCamera3d::new(
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
        let mut root = SceneNode3d::empty();

        let mut cube = root.add_group();
        let light = root
            .add_light(Light::point(100.0))
            .set_position(Vec3::new(0.0, 10.0, 0.0));

        let mut cubelets = Vec::with_capacity(26);
        for i in 0..=2 {
            for j in 0..=2 {
                for k in 0..=2 {
                    if i == 1 && j == 1 && k == 1 {
                        continue;
                    }
                    let x = (1 - i) as f32 * 1.1;
                    let y = (1 - j) as f32 * 1.1;
                    let z = (1 - k) as f32 * 1.1;
                    let node = cube.add_cube(1.0, 1.0, 1.0).translate(Vec3 { x, y, z });
                    cubelets.push(Cubelet {
                        node,
                        texture: Texture::new_default(),
                    });
                }
            }
        }

        let scene = Scene {
            camera,
            root,
            light,
            cube,
            cubelets,
        };

        Self { window, scene }
    }

    pub async fn draw_once(&mut self) {
        self.window
            .render_3d(&mut self.scene.root, &mut self.scene.camera)
            .await;
    }

    pub async fn draw_loop(&mut self, mut f: impl FnMut(&mut Scene) -> DrawResult) {
        while self
            .window
            .render_3d(&mut self.scene.root, &mut self.scene.camera)
            .await
        {
            match f(&mut self.scene) {
                DrawResult::Continue => (),
                DrawResult::Break => return,
            }
        }
    }

    /// The rendered image will be stale unless you call `Renderer::draw_once` first.
    pub fn write_once_to_buf(&self) -> Vec<u8> {
        let n_bytes = (self.window.width() * self.window.height() * 4) as usize;
        let mut ret = Vec::with_capacity(n_bytes);
        self.window.snap(&mut ret);
        ret
    }
}
