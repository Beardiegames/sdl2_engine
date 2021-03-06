use std::path::Path;

use sdl2::surface::Surface;

use crate::swarm::Swarm;
use crate::{Entity, RenderContext};

pub struct Scene<EntityState: Default + Clone, GameData> {
    pub pool_size: usize,
    pub asset_paths: Vec<&'static str>, 
    //pub(crate) surfaces: Vec<(String, Surface<'s>)>,
    pub on_start: fn(&mut Swarm<Entity<EntityState>, RenderContext<GameData>>),
    pub on_update: fn(&mut Swarm<Entity<EntityState>, RenderContext<GameData>>),
    pub on_end: fn(),
}

impl<EntityState: Default + Clone, GameData> Scene <EntityState, GameData> {

    pub fn new(
        pool_size: usize,
        asset_paths: &[&'static str], 
        on_start: fn(&mut Swarm<Entity<EntityState>, RenderContext<GameData>>), 
        on_update: fn(&mut Swarm<Entity<EntityState>, RenderContext<GameData>>), 
        on_end: fn()
    ) -> Self {

        // let mut surfaces = Vec::<(String, Surface<'s>)>::new();

        // for path in asset_paths {
        //     if let Ok(surface) = Surface::load_bmp(Path::new(*path)) {
        //         surfaces.push((String::from(*path), surface));
        //     }
        // }

        Scene { pool_size, asset_paths: Vec::from(asset_paths), on_start, on_update, on_end, }
    }
}