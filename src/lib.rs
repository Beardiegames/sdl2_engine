extern crate swarm_pool as swarm;

pub mod input;
pub mod sprites;
pub mod timer;
pub mod transform;
pub mod camera;

mod renderer;
mod scenes;

pub use scenes::Scene;
pub use renderer::{ Renderer, RenderContext };

use sprites::Sprite;
use transform::Transform;
#[allow(unused)]
use camera::Camera;


#[derive(Default, Clone)]
pub struct Entity<C: Default + Clone> {
    pub transform: Transform,
    pub sprite: Sprite,
    pub state: C,
}
