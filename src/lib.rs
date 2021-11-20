extern crate swarm_pool as swarm;
extern crate multi_threaded_pool as thread_pool;

pub mod input;
pub mod sprites;
pub mod timer;
pub mod transform;
pub mod camera;

mod renderer;
mod scenes;

pub use scenes::Scene;
pub use renderer::{ Renderer };

use sprites::Sprite;
use transform::Transform;
use camera::Camera;


#[derive(Default, Clone)]
pub struct Entity<ObjectType: Default + Clone + Send> {
    pub transform: Transform,
    pub sprite: Sprite,
    pub object: ObjectType,
}
