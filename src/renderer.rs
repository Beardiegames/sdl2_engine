
use std::{ cell::RefCell, rc::Rc, };

use sdl2::{
    render::{ Canvas, Texture }, 
    video::{ Window },
    event::Event as SdlEvent, 
};
use swarm::Swarm;

use crate::{Entity, Scene, input::{self, Input}, timer::UpdateTimer};


pub struct RenderContext<'c, GameData> {
    textures: Vec<Texture<'c>>,
    canvas: Rc<RefCell<Canvas<Window>>>,
    pub timer: UpdateTimer,
    pub data: GameData,
    pub input: Input,
}

pub struct Renderer {
    pub event_pump: sdl2::EventPump,
    pub canvas: Rc<RefCell<Canvas<Window>>>,
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
                    _ => {},
                }
            }

            // tell scene observer to update their frame code
            (scene.on_update)(&mut swarm);
            
            // clear screen buffer
            self.canvas.borrow_mut().clear();

            // write screen buffer
            swarm.for_all(|obj_index, pool, game| {

                let target = &mut pool[*obj_index];

                if let Some(sprite) = &mut target.sprite
                {
                    sprite.update_animation(&game.timer.delta_time);

                    if let Some(dst) = &mut sprite.dst.0 {
                        dst.set_x(target.transform.x);
                        dst.set_y(target.transform.y);
                        dst.set_width(target.transform.width);
                        dst.set_height(target.transform.height);
                    }

                    game.canvas.borrow_mut().copy_ex(
                        &game.textures[sprite.texure_id],
                        sprite.src.0,
                        sprite.dst.0,
                        target.transform.rotation,
                        None,
                        target.transform.flip_horizontal,
                        target.transform.flip_vertical,
                    ).unwrap();
                }

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