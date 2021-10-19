
extern crate sdl2_engine as engine;

use engine::{ 
    Renderer,
    Scene,
    sprites::Sprite,
    sprites::SpriteAnimation,
    Entity,
    transform::Position,
    transform::Size,
    input::Key,
};


#[derive(Default, Clone)]
struct EntityState {
    character: Character,
}

#[derive(Default)]
struct GameData {

}

fn main() -> Result<(), String> {
    let mut renderer = Renderer::new("TEST", 640, 480)?;

    let mut scene = Scene::<EntityState, GameData>::new(

        &["assets/characters.bmp"],

        |swarm| {
            swarm.populate(&[
                Entity {
                    sprite: Sprite {
                        texure_id: 0,
                        position: Position { x: 100, y: 100 },
                        size: Size { x: 64, y: 64 },
        
                        tile_size: Size { x: 32, y: 32 },
                        num_tile_cols: 4,
                        animation: 0,
                        animations: vec![
                            BABY_IDLE_ANIM.clone(),
                            BABY_WALK_ANIM.clone(),
                            KING_IDLE_ANIM.clone(),
                            KING_WALK_ANIM.clone(),
                            SOLDIER_IDLE_ANIM.clone(),
                            SOLDIER_WALK_ANIM.clone(),
                        ]
                    },
                    state: EntityState::default(),
                }
            ]);
        },

        |swarm| {
            swarm.for_all(|target, pool, props| {

                let key_left = props.input.keyboard.down(Key::Left);
                let key_right = props.input.keyboard.down(Key::Right);
                let key_up = props.input.keyboard.down(Key::Up);
                let key_down = props.input.keyboard.down(Key::Down);
                let key_space = props.input.keyboard.pressed(Key::Space);
    
                if key_left ^ key_right {
                    if key_left {
                        pool[*target].sprite.position.x -= (props.timer.delta_time / 2) as i32;
                    } else if key_right {
                        pool[*target].sprite.position.x += (props.timer.delta_time / 2) as i32;
                    }
                }
                
                if key_up ^ key_down {
                    if key_up {
                        pool[*target].sprite.position.y -= (props.timer.delta_time / 2) as i32;
                    } else if key_down {
                        pool[*target].sprite.position.y += (props.timer.delta_time / 2) as i32;
                    }
                }
    
                if key_up | key_down | key_left | key_right {
                    pool[*target].sprite.animation = match pool[*target].state.character {
                        Character::Baby => BABY_WALK,
                        Character::King => KING_WALK,
                        Character::Soldier => SOLDIER_WALK,
                    };
                } else {
                    pool[*target].sprite.animation = match pool[*target].state.character {
                        Character::Baby => BABY_IDLE,
                        Character::King => KING_IDLE,
                        Character::Soldier => SOLDIER_IDLE,
                    };
                }
    
                if key_space {
                    pool[*target].state.character.cycle();
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
