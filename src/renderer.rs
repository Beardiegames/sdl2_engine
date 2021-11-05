
use std::{ cell::RefCell, rc::Rc, };

use sdl2::{
    render::{ Canvas, Texture }, 
    video::{ Window },
    event::Event as SdlEvent, 
};
use swarm::Swarm;

use crate::{
    Entity, Scene, camera::Camera, 
    input::{ self, Input }, 
    timer::UpdateTimer
};

#[derive(Clone)]
pub struct Screen {
    pub width: u32,
    pub height: u32,
    pub center_x: i32,
    pub center_y: i32,
}

pub struct RenderContext<'c, GameData> {
    textures: Vec<Texture<'c>>,
    canvas: Rc<RefCell<Canvas<Window>>>,
    pub timer: UpdateTimer,
    pub data: GameData,
    pub input: Input,
    pub camera: Camera,
    pub screen : Screen,
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

    pub fn play<'s, EntityState, GameData>(&mut self, scene: &mut Scene<'s, EntityState, GameData>, target_fps: u64)
         -> Result<(), String> 
    where 
    EntityState: Default + Clone,
    GameData: Default,
    {
        // create a texture container for the canvas to draw from
        let texture_creator = self.canvas.borrow_mut().texture_creator();

        // setup a render context to talk to while looping though all swarm pool objects
        let mut context = RenderContext { 
            textures: Vec::<Texture<'s>>::new(),
            canvas: self.canvas.clone(),
            timer: UpdateTimer::new(target_fps),
            data: GameData::default(),
            input: Input::new(),
            camera: Camera { x:0.0, y:0.0, zoom:1.0, zpow: 1.0, },
            screen: self.screen.clone(),
        };

        // create texture maps from loaded surfaces
        for surface in &scene.surfaces {
            context.textures.push(
                texture_creator
                    .create_texture_from_surface(&surface.1)
                    .map_err(|e| e.to_string())?
            );
        }
        
        // create scene object pool
        let mut swarm = Swarm::<Entity<EntityState>, RenderContext<GameData>>::new(scene.pool_size, context);

        // tell scene observer, scene initialization is complete
        (scene.on_start)(&mut swarm);

        // start game loop
        'game_loop: loop {

            // reset frame based events
            swarm.properties.input.keyboard.releave_activity();

            // capture/handle input events
            while let Some(event) = self.event_pump.poll_event() {
                match event {
                    SdlEvent::Quit{ .. } => break 'game_loop,

                    SdlEvent::KeyDown { keycode, .. } => {
                        if let Some (key) = keycode { 
                            input::map_keys(&mut swarm.properties.input.keyboard, key, true);
                        }
                    },
                    SdlEvent::KeyUp { keycode, .. } => {
                        if let Some (key) = keycode { 
                            input::map_keys(&mut swarm.properties.input.keyboard, key, false);
                        }
                    },

                    SdlEvent::MouseMotion {x, y, ..} => {
                        swarm.properties.input.mouse.x = x;
                        swarm.properties.input.mouse.y = y;
                    },
                    SdlEvent::MouseButtonUp {mouse_btn, ..} => {
                        match mouse_btn {
                            sdl2::mouse::MouseButton::Left => swarm.properties.input.mouse.left_button = false,
                            sdl2::mouse::MouseButton::Right => swarm.properties.input.mouse.right_button = false,
                            sdl2::mouse::MouseButton::Middle => swarm.properties.input.mouse.middle_button = false,
                            _ => {},
                        };
                    },
                    SdlEvent::MouseButtonDown {mouse_btn, ..} => {
                        match mouse_btn {
                            sdl2::mouse::MouseButton::Left => swarm.properties.input.mouse.left_button = true,
                            sdl2::mouse::MouseButton::Right => swarm.properties.input.mouse.right_button = true,
                            sdl2::mouse::MouseButton::Middle => swarm.properties.input.mouse.middle_button = true,
                            _ => {},
                        };
                    },

                    SdlEvent::ControllerDeviceAdded {which, ..} => {
                        swarm.properties.input.controllers.add(which);
                    },
                    SdlEvent::ControllerDeviceRemoved {which, ..} => {
                        swarm.properties.input.controllers.remove(which);
                    },
                    SdlEvent::ControllerAxisMotion {which, axis, value, ..} => {
                        swarm.properties.input.controllers.set_axis(which, &axis, value);
                    },
                    SdlEvent::ControllerButtonUp {which, button, ..} => {
                        swarm.properties.input.controllers.set_button(which, &button, false);
                    },
                    SdlEvent::ControllerButtonDown {which, button, ..} => {
                        swarm.properties.input.controllers.set_button(which, &button, true);
                    },
                    _ => {},
                }
            }

            // tell scene observer to update their frame code
            (scene.on_update)(&mut swarm);

            swarm.properties.camera.set_power();
            
            // clear screen buffer
            self.canvas.borrow_mut().clear();

            // write screen buffer
            swarm.for_all(|obj_index, pool, game| {

                //let target = &mut pool[*obj_index];

                // if let Some(sprite) = &mut target.sprite
                // {
                    pool[*obj_index].sprite.update_animation(&game.timer.frame_duration);

                    if let Some(dst) = &mut pool[*obj_index].sprite.dst.0 {
                        let x = pool[*obj_index].transform.x - game.camera.x;
                        let y = pool[*obj_index].transform.y - game.camera.y;
                        

                        dst.set_x(game.screen.center_x + (x * game.camera.zpow) as i32);
                        dst.set_y(game.screen.center_y + (y * game.camera.zpow) as i32);
                        dst.set_width((pool[*obj_index].transform.width as f32 * game.camera.zpow) as u32);
                        dst.set_height((pool[*obj_index].transform.height as f32 * game.camera.zpow) as u32);
                    }

                    game.canvas.borrow_mut().copy_ex(
                        &game.textures[pool[*obj_index].sprite.texure_id],
                        pool[*obj_index].sprite.src.0,
                        pool[*obj_index].sprite.dst.0,
                        pool[*obj_index].transform.rotation,
                        None,
                        pool[*obj_index].transform.flip_horizontal,
                        pool[*obj_index].transform.flip_vertical,
                    ).unwrap();
               // }

            });

            // present screen buffer
            self.canvas.borrow_mut().present();

            // update frame timer
            swarm.properties.timer.sync();
        }

        // tell the scene observer the scene has finisched
        (scene.on_end)();

        Ok(())
    }
}