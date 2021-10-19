
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
        let texture_creator = self.canvas.borrow_mut().texture_creator();

        let mut context = RenderContext { 
            textures: Vec::<Texture<'s>>::new(),
            canvas: self.canvas.clone(),
            timer: UpdateTimer::new(target_fps),
            data: GameData::default(),
            input: Input::new(),
        };

        for surface in &scene.surfaces {
            context.textures.push(
                texture_creator
                    .create_texture_from_surface(&surface.1)
                    .map_err(|e| e.to_string())?
            );
        }
        
        let mut swarm = Swarm::<Entity<EntityState>, RenderContext<GameData>>::new(1000, context);

        (scene.on_start)(&mut swarm);

        let mut running = true;

        'game_loop: while running {

            swarm.properties.input.keyboard.releave_activity();

            for sdl_event in self.event_pump.poll_iter() {

                if let SdlEvent::Quit{ .. } = sdl_event {
                    break 'game_loop;
                } 

                if let SdlEvent::KeyDown { keycode, .. } = sdl_event {
                    if let Some (key) = keycode {
                        input::map_keyboard_input(&mut swarm.properties.input.keyboard, key, true);
                    }
                }
        
                if let SdlEvent::KeyUp { keycode, .. } = sdl_event {
                    if let Some (key) = keycode {
                        input::map_keyboard_input(&mut swarm.properties.input.keyboard, key, false);
                    }
                }
            }

            (scene.on_update)(&mut swarm);
            
            self.canvas.borrow_mut().clear();


            swarm.for_all(|target, pool, props| {

                if let Some(sprite) = &mut pool[*target].sprite
                {
                    sprite.update_animation(&props.timer.delta_time);

                    if let Some(dst) = &mut sprite.dst.0 {
                        dst.set_x(pool[*target].transform.position.x);
                        dst.set_y(pool[*target].transform.position.y);
                        dst.set_width(pool[*target].transform.size.width);
                        dst.set_height(pool[*target].transform.size.height);
                    }

                    props.canvas.borrow_mut().copy_ex(
                        &props.textures[sprite.texure_id],
                        sprite.src.0,
                        sprite.dst.0,
                        pool[*target].transform.rotation,
                        None,
                        pool[*target].transform.flip.horizontal,
                        pool[*target].transform.flip.vertical,
                    ).unwrap();
                }

            });

            self.canvas.borrow_mut().present();

            swarm.properties.timer.sync();
        }

        (scene.on_end)();
        Ok(())
    }
}