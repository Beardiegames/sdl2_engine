use std::marker::PhantomData;
use std::path::Path;

use multi_threaded_pool::Cluster;
use sdl2::surface::Surface;

use crate::swarm::Swarm;
use crate::{Entity};


pub type ObserverCallback<ObjectType, LocalData> = dyn FnMut(&mut Cluster<Entity<ObjectType>, LocalData>);

pub struct Scene<'s> //, ObjectType, Callback, SharedData, LocalData>
// where   ObjectType: Default + Clone + Send,
//         Callback: FnMut(&mut Cluster<Entity<ObjectType>, SharedData, LocalData>) + Clone,
{
    pub pool_size: usize,
    pub(crate) surfaces: Vec<(String, Surface<'s>)>,
    // pub on_start: Callback,
    // pub on_update: Callback,
    // pub on_end: Callback,

    // ph_ot: PhantomData<ObjectType>,
    // ph_sd: PhantomData<SharedData>,
    // ph_ld: PhantomData<LocalData>,
}

// impl<'s, ObjectType, Callback, SharedData, LocalData> Scene <'s, ObjectType, Callback, SharedData, LocalData>
// where   ObjectType: Default + Clone + Send,
//         Callback: FnMut(&mut Cluster<Entity<ObjectType>, SharedData, LocalData>) + Clone,
// {
impl<'s> Scene <'s> {
    pub fn new(
        pool_size: usize,
        asset_paths: &[&str], 
        // on_start: Callback, 
        // on_update: Callback, 
        // on_end: Callback
    ) -> Self {

        let mut surfaces = Vec::<(String, Surface<'s>)>::new();

        for path in asset_paths {
            if let Ok(surface) = Surface::load_bmp(Path::new(*path)) {
                surfaces.push((String::from(*path), surface));
            }
        }

        Scene { 
            pool_size, surfaces, 
            // on_start, on_update, on_end, 
            // ph_ot: PhantomData,
            // ph_sd: PhantomData,
            // ph_ld: PhantomData,
        }
    }
}