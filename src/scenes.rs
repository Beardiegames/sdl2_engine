use std::path::Path;

use sdl2::surface::Surface;

use crate::swarm::Swarm;
use crate::{Entity, RenderContext};

pub struct Scene<'s, EntityState: Default + Clone, GameData> {
    pub(crate) surfaces: Vec<(String, Surface<'s>)>,
    pub on_start: fn(&mut Swarm<Entity<EntityState>, RenderContext<GameData>>),
    pub on_update: fn(&mut Swarm<Entity<EntityState>, RenderContext<GameData>>),
    pub on_end: fn(),
}

impl<'s, EntityState: Default + Clone, GameData> Scene <'s, EntityState, GameData> {

    pub fn new(
        asset_paths: &[&str], 
        on_start: fn(&mut Swarm<Entity<EntityState>, RenderContext<GameData>>), 
        on_update: fn(&mut Swarm<Entity<EntityState>, RenderContext<GameData>>), 
        on_end: fn()
    ) -> Self {

        let mut surfaces = Vec::<(String, Surface<'s>)>::new();

        for path in asset_paths {
            if let Ok(surface) = Surface::load_bmp(Path::new(*path)) {
                surfaces.push((String::from(*path), surface));
            }
        }

        Scene { surfaces, on_start, on_update, on_end, }
    }
}