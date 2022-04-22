
extern crate sdl2_engine as engine;

use engine::{ Renderer, Scene, Entity, };
use engine::input::{ Key };
use engine::sprites::{ SpriteAnimation, SpriteBuilder };
use engine::transform::{ Transform };


#[derive(Default, Clone)]
struct ObjState {
    character: Character,
}

#[derive(Default)]
struct GameData {

}

fn main() -> Result<(), String> {
    let mut renderer = Renderer::new("TEST", 640, 480)?;

    

    let mut scene = Scene::<ObjState, GameData>::new(
        100_000,
        &["assets/characters.bmp"],

        |swarm| {

            let mut populous = Vec::<Entity<ObjState>>::new();

            for i in 0..50_000 {
                let x = (i as f32 % 100.0) * 64.0;
                let y = (i as f32 / 100.0) * 64.0;

                populous.push(
                    Entity {
                        transform: Transform::default()
                                .with_position(x, y)
                                .with_size(64, 64),
                
                        sprite: SpriteBuilder::new(0)
                                .with_tile_size(32, 32)
                                .with_column_count(4)
                                .with_start_animation(0)
                                .with_animations(vec![
                                    BABY_IDLE_ANIM.clone(),
                                    BABY_WALK_ANIM.clone(),
                                    KING_IDLE_ANIM.clone(),
                                    KING_WALK_ANIM.clone(),
                                    SOLDIER_IDLE_ANIM.clone(),
                                    SOLDIER_WALK_ANIM.clone(),
                                ])
                                .build(),
                
                        state: ObjState::default(),
                    }
                );
            }

            swarm.populate(&populous);
        },

        |swarm| {

            let delta_time = swarm.properties.timer.delta_time as f32;

            let cam_left = swarm.properties.input.keyboard.down(Key::A);
            let cam_right = swarm.properties.input.keyboard.down(Key::D);
            let cam_up = swarm.properties.input.keyboard.down(Key::W);
            let cam_down = swarm.properties.input.keyboard.down(Key::S);
            let cam_zoom_in = swarm.properties.input.keyboard.down(Key::E);
            let cam_zoom_out = swarm.properties.input.keyboard.down(Key::Q);

            if cam_left ^ cam_right {
                if cam_left {
                    swarm.properties.camera.x -= delta_time * 200.0;
                } else if cam_right {
                    swarm.properties.camera.x += delta_time * 200.0;     
                }
            }
            
            if cam_up ^ cam_down {
                if cam_up {
                    swarm.properties.camera.y -= delta_time * 200.0;
                } else if cam_down {
                    swarm.properties.camera.y += delta_time * 200.0;     
                }
            }

            if cam_zoom_in ^ cam_zoom_out {
                if cam_zoom_in {
                    swarm.properties.camera.zoom -= delta_time * 0.1;
                } else if cam_zoom_out {
                    swarm.properties.camera.zoom += delta_time * 0.1;    
                }
            }


            swarm.for_all(|object_index, pool, game| {

                let target = &mut pool[*object_index];
                let delta_time = game.timer.delta_time as f32;

                let key_pressed_left = game.input.keyboard.pressed(Key::Left);
                let key_pressed_right = game.input.keyboard.pressed(Key::Right);
                let key_released_space = game.input.keyboard.released(Key::Space);

                let key_left = game.input.keyboard.down(Key::Left);
                let key_right = game.input.keyboard.down(Key::Right);
                let key_up = game.input.keyboard.down(Key::Up);
                let key_down = game.input.keyboard.down(Key::Down);

                
                if key_pressed_left { target.transform.flip_horizontal = false; }
                if key_pressed_right { target.transform.flip_horizontal = true; }

                if key_left ^ key_right {
                    if key_left {
                        target.transform.x -= delta_time * 200.0;
                    } else if key_right {
                        target.transform.x += delta_time * 200.0;     
                    }
                }
                
                if key_up ^ key_down {
                    if key_up {
                        target.transform.y -= delta_time * 200.0;
                        target.transform.rotation -= delta_time as f64 * 0.5;
                    } else if key_down {
                        target.transform.y += delta_time * 200.0;
                        target.transform.rotation += delta_time as f64 * 0.5;
                    }
                }

                //if let Some(sprite) = &mut target.sprite {
                    if key_up | key_down | key_left | key_right {
                       target.sprite.animation = match target.state.character {
                            Character::Baby => BABY_WALK,
                            Character::King => KING_WALK,
                            Character::Soldier => SOLDIER_WALK,
                        };
                    } else {
                       target.sprite.animation = match target.state.character {
                            Character::Baby => BABY_IDLE,
                            Character::King => KING_IDLE,
                            Character::Soldier => SOLDIER_IDLE,
                        };
                    }
                //}

                if key_released_space {
                    target.state.character.cycle();
                }
            });
        },

        ||{

        },
    );

    renderer.play(&mut scene, 100)?;

    Ok(())
}

#[derive(Clone)]
enum Character { Baby, King, Soldier }
impl Character {
    fn cycle(&mut self) {
        println!("cycle");
        *self = match self {
            Self::Baby => Self::King,
            Self::King => Self::Soldier,
            Self::Soldier => Self::Baby,
        }
    }
}

impl Default for Character {
    fn default() -> Self { Character::Baby }
}


// GAME DATA

const BABY_IDLE: usize = 0;
static BABY_IDLE_ANIM: SpriteAnimation = SpriteAnimation {
    current_frame: 0,
    tile_range: (0..=0),
    millis_per_frame: 80,
    millis_passed: 0,
};


const BABY_WALK: usize = 1;
static BABY_WALK_ANIM: SpriteAnimation = SpriteAnimation {
    current_frame: 0,
    tile_range: (0..=3),
    millis_per_frame: 80,
    millis_passed: 0,
};


const KING_IDLE: usize = 2;
static KING_IDLE_ANIM: SpriteAnimation = SpriteAnimation {
    current_frame: 0,
    tile_range: (4..=4),
    millis_per_frame: 80,
    millis_passed: 0,
};

const KING_WALK: usize = 3;
static KING_WALK_ANIM: SpriteAnimation = SpriteAnimation {
    current_frame: 0,
    tile_range: (4..=7),
    millis_per_frame: 80,
    millis_passed: 0,
};

const SOLDIER_IDLE: usize = 4;
static SOLDIER_IDLE_ANIM: SpriteAnimation = SpriteAnimation {
    current_frame: 0,
    tile_range: (8..=8),
    millis_per_frame: 80,
    millis_passed: 0,
};

const SOLDIER_WALK: usize = 5;
static SOLDIER_WALK_ANIM: SpriteAnimation = SpriteAnimation {
    current_frame: 0,
    tile_range: (8..=11),
    millis_per_frame: 80,
    millis_passed: 0,
};
