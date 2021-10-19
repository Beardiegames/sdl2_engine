extern crate swarm_pool as swarm;

pub mod input;
pub mod sprites;
pub mod timer;
pub mod transform;

mod renderer;
mod scenes;

pub use scenes::Scene;
pub use renderer::{ Renderer, RenderContext };

use sprites::Sprite;
use transform::Transform;


#[derive(Default, Clone)]
pub struct Entity<C: Default + Clone> {
    pub transform: Transform,
    pub sprite: Option<Sprite>,
    pub state: C,
}
