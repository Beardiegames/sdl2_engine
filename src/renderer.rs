
use std::{cell::RefCell, fmt::Debug, rc::Rc, sync::{Arc, Mutex}};

use multi_threaded_pool::{Cluster, ObjectPool};
use sdl2::{event::Event as SdlEvent, rect::Rect, render::{ Canvas, Texture }, video::{ Window }};
use swarm::Swarm;
use thread_pool::ThreadPool;

use crate::{Entity, Scene, camera::Camera, input::{ self, Input }, sprites::Image, timer::UpdateTimer};


#[derive(Default, Clone, Debug)]
pub struct Screen {
    pub width: u32,
    pub height: u32,
    pub center_x: i32,
    pub center_y: i32,
}

// #[derive(Clone, Debug)]
// pub struct DrawSprite {
//     pub image: usize, 
//     pub source: Rect, 
//     pub dest: Rect,
// }


#[derive(Default, Clone, Debug)]
pub struct Engine<GameData>
where   GameData: Default + Clone + Debug,
{
    pub game_data: GameData,
    pub delta_time: f32,
    pub layer: usize,
    pub input: Input,
    pub camera: Camera,
    pub screen : Screen,

    iter_pos: usize,
    iter_count: usize,
    draw: Vec::<Image>,
}

pub type Layer<ObjectType, GameData> = Cluster<Entity<ObjectType>, Engine<GameData>>;

pub trait IObserver<ObjectType, GameData> 
where   GameData: Default + Clone + Send + Debug,
        ObjectType: Default + Clone + Send,
{
    fn on_start(layer: &mut Layer<ObjectType, GameData>);
    fn on_update(layer: &mut Layer<ObjectType, GameData>);
}

pub struct Renderer {
    pub event_pump: sdl2::EventPump,
    pub canvas: Rc<RefCell<Canvas<Window>>>,
    pub screen : Screen,
}

impl Renderer {
    pub fn new(title: &str, width: u32, height: u32) -> Result<Renderer, String> {
        let sdl_context = sdl2::init()?;
        let video_subsystem = sdl_context.video()?;

        let window = video_subsystem
            .window(title, width, height)
            .position_centered()
            .build()
            .map_err(|e| e.to_string())?;

        let canvas = window
            .into_canvas()
            .accelerated()
            .build()
            .map_err(|e| e.to_string())?;

        let canvas = Rc::new(RefCell::new(canvas));
        let event_pump = sdl_context.event_pump()?;

        let renderer = Renderer {
            event_pump,
            canvas,
            screen: Screen { 
                width, 
                height,
                center_x: width as i32 / 2,
                center_y: height as i32 / 2,
            },
        };

        Ok(renderer)
    }

    pub fn play<'s, Observer, ObjectType, GameData>(
        &mut self, 
        scene: &mut Scene<'s>, 
        target_fps: u64,
        observer: Observer,

    ) -> Result<(), String> 
    where   GameData: Default + Clone + Send + Debug + 'static,
            ObjectType: Default + Clone + Send + 'static,
            Observer: IObserver<ObjectType, GameData> + Default + Send + 'static,
    {
        // create a texture container for the canvas to draw from
        let texture_creator = self.canvas.borrow_mut().texture_creator();
        let mut textures = Vec::<Texture<'s>>::new();

        // create texture maps from loaded surfaces
        for surface in &scene.surfaces {
            textures.push(
                texture_creator
                    .create_texture_from_surface(&surface.1)
                    .map_err(|e| e.to_string())?
            );
        }

        //let draw = Vec::with_capacity(100);
        //for drawlayer in 0..100 { draw.push(Vec::new()); } 

        let mut input = Input::new();
        let mut camera = Camera { x:0.0, y:0.0, zoom:1.0, zpow: 1.0, };

        // setup a render context to talk to while looping though all swarm pool objects
        // let mut context = Engine { 
        //     // textures
        //     // canvas: self.canvas.clone(),
        //     //timer: UpdateTimer::new(target_fps),
        //     game_data: GameData::default();
        //     input: Input::new(),
        //     camera: Camera { x:0.0, y:0.0, zoom:1.0, zpow: 1.0, },
        //     screen: self.screen.clone(),

        //     draw,
        // };

        // create scene object pool
        let mut tpool = ThreadPool::<Entity<ObjectType>, Engine<GameData>>::new(
            100, 
            1000     
        );

        tpool.start(
            |cluster| {

                cluster.shared.write(
                    *cluster.thread_id(),
                    |d| {
                        d.draw = Vec::with_capacity(1000);
                        for _i in 0..1000 { d.draw.push(Image::default()); }
                    }
                );

                // tell scene observer to set the scene
                Observer::on_start(cluster);
            },
            |cluster, dt| {
                
                Observer::on_update(cluster);

                // update sprites
                let thread = cluster.thread_id().clone();
                let count = cluster.count();

                cluster.shared.catch(
                    thread,
                    &(thread, dt, count), 
                    |v, d| {
                        d.layer = v.0;
                        d.delta_time = *v.1;
                        d.iter_pos = 0;
                        d.iter_count = v.2;
                    }
                );

                cluster.iter(|pool, game| {
                    
                    let target = pool.target();
                    
                    game.catch_mut(
                        *pool.thread_id(), 
                        pool.target(), 
                        |v, d| {     
                            v.sprite.update_animation(&(d.delta_time as u32 * 1000), &mut d.draw[d.iter_pos]);

                            if let Some(dst) = &mut d.draw[d.iter_pos].dst.0 {
                                let x = v.transform.x - d.camera.x;
                                let y = v.transform.y - d.camera.y;
                                
                                dst.set_x(d.screen.center_x + (x * d.camera.zpow) as i32);
                                dst.set_y(d.screen.center_y + (y * d.camera.zpow) as i32);
                                dst.set_width((v.transform.width as f32 * d.camera.zpow) as u32);
                                dst.set_height((v.transform.height as f32 * d.camera.zpow) as u32);
                            }

                            d.draw[d.iter_pos].transform.x = v.transform.x;
                            d.draw[d.iter_pos].transform.y = v.transform.y;
                            d.draw[d.iter_pos].transform.z = v.transform.z;
                            d.draw[d.iter_pos].transform.width = v.transform.width; 
                            d.draw[d.iter_pos].transform.height = v.transform.height;
                            d.draw[d.iter_pos].transform.rotation = v.transform.rotation;
                            d.draw[d.iter_pos].transform.flip_horizontal = v.transform.flip_horizontal;
                            d.draw[d.iter_pos].transform.flip_vertical = v.transform.flip_vertical;

                            d.iter_pos += 1;
                        }
                    );
                });
            }
        );

        
        // start game loop
        'game_loop: loop {

            // reset frame based events
            //swarm.properties.input.keyboard.releave_activity();
            input.keyboard.releave_activity();

            // capture/handle input events
            while let Some(event) = self.event_pump.poll_event() {
                match event {
                    SdlEvent::Quit{ .. } => break 'game_loop,

                    SdlEvent::KeyDown { keycode, .. } => {
                        if let Some (key) = keycode { 
                            input::map_keys(&mut input.keyboard, key, true);
                        }
                    },
                    SdlEvent::KeyUp { keycode, .. } => {
                        if let Some (key) = keycode { 
                            input::map_keys(&mut input.keyboard, key, false);
                        }
                    },

                    SdlEvent::MouseMotion {x, y, ..} => {
                        input.mouse.x = x;
                        input.mouse.y = y;
                    },
                    SdlEvent::MouseButtonUp {mouse_btn, ..} => {
                        match mouse_btn {
                            sdl2::mouse::MouseButton::Left => input.mouse.left_button = false,
                            sdl2::mouse::MouseButton::Right => input.mouse.right_button = false,
                            sdl2::mouse::MouseButton::Middle => input.mouse.middle_button = false,
                            _ => {},
                        };
                    },
                    SdlEvent::MouseButtonDown {mouse_btn, ..} => {
                        match mouse_btn {
                            sdl2::mouse::MouseButton::Left => input.mouse.left_button = true,
                            sdl2::mouse::MouseButton::Right => input.mouse.right_button = true,
                            sdl2::mouse::MouseButton::Middle => input.mouse.middle_button = true,
                            _ => {},
                        };
                    },

                    SdlEvent::ControllerDeviceAdded {which, ..} => {
                        input.controllers.add(which);
                    },
                    SdlEvent::ControllerDeviceRemoved {which, ..} => {
                        input.controllers.remove(which);
                    },
                    SdlEvent::ControllerAxisMotion {which, axis, value, ..} => {
                        input.controllers.set_axis(which, &axis, value);
                    },
                    SdlEvent::ControllerButtonUp {which, button, ..} => {
                        input.controllers.set_button(which, &button, false);
                    },
                    SdlEvent::ControllerButtonDown {which, button, ..} => {
                        input.controllers.set_button(which, &button, true);
                    },
                    _ => {},
                }
            }

            // tell scene observer to update their frame code
            //(scene.on_update)(&mut swarm);

            camera.set_power();
            
            // clear screen buffer
            self.canvas.borrow_mut().clear();

            // write screen buffer
            tpool.shared.catch_mut_all(
                &mut (self.canvas.borrow_mut(), &textures),
                |v, d| {
                    let mut i = d.draw.len();
                    while i > 0 {
                        i -= 1;
                        v.0.copy_ex(
                            &v.1[d.draw[i].texture_id],
                            d.draw[i].src.0,
                            d.draw[i].dst.0,
                            d.draw[i].transform.rotation,
                            None,
                            d.draw[i].transform.flip_horizontal,
                            d.draw[i].transform.flip_vertical,
                        ).unwrap();
                    }
                }
            );


            // swarm.for_all(|obj_index, pool, game| {

            //     //let target = &mut pool[*obj_index];

            //     // if let Some(sprite) = &mut target.sprite
            //     // {
            //         pool[*obj_index].sprite.update_animation(&game.timer.frame_duration);

            //         if let Some(dst) = &mut pool[*obj_index].sprite.dst.0 {
            //             let x = pool[*obj_index].transform.x - game.camera.x;
            //             let y = pool[*obj_index].transform.y - game.camera.y;
                        

            //             dst.set_x(game.screen.center_x + (x * game.camera.zpow) as i32);
            //             dst.set_y(game.screen.center_y + (y * game.camera.zpow) as i32);
            //             dst.set_width((pool[*obj_index].transform.width as f32 * game.camera.zpow) as u32);
            //             dst.set_height((pool[*obj_index].transform.height as f32 * game.camera.zpow) as u32);
            //         }

            //         game.canvas.borrow_mut().copy_ex(
            //             &game.textures[pool[*obj_index].sprite.texure_id],
            //             pool[*obj_index].sprite.src.0,
            //             pool[*obj_index].sprite.dst.0,
            //             pool[*obj_index].transform.rotation,
            //             None,
            //             pool[*obj_index].transform.flip_horizontal,
            //             pool[*obj_index].transform.flip_vertical,
            //         ).unwrap();
            //    // }

            // });

            // present screen buffer
            self.canvas.borrow_mut().present();

            // update frame timer
            //swarm.properties.timer.sync();
        }

        // tell the scene observer the scene has finisched
        //(scene.on_end)();

        Ok(())
    }
}