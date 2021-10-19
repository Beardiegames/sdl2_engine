use std::ops::RangeInclusive;

use crate::transform::{Position, Size};


#[derive(Default, Clone)]
pub struct Sprite {
    pub texure_id: usize,
    pub position: Position,
    pub size: Size,

    pub tile_size: Size,
    pub num_tile_cols: u16,

    pub animation: usize,
    pub animations: Vec<SpriteAnimation>,
}

impl Sprite {
    pub fn tile_position(&self) -> Position {
        if self.animation < self.animations.len() {
            let tile = self.animations[self.animation].tile_range.start() 
                + self.animations[self.animation].current_frame;

            let col = tile % self.num_tile_cols;
            let row = tile / self.num_tile_cols;

            Position {
               x: (col as u32 * self.tile_size.x) as i32, 
               y: (row as u32 * self.tile_size.y) as i32,
            }

        } else {
            Position{ x: 0, y: 0 }
        }
    }

    pub fn update_animation(&mut self, _delta_time: &u64) {
        if self.animation < self.animations.len() {
            let anim = &mut self.animations[self.animation];
            
            anim.millis_passed += *_delta_time;
            anim.millis_passed = anim.millis_passed % 
                (anim.millis_per_frame * anim.tile_range.len() as u64);

            anim.current_frame = (anim.millis_passed / anim.millis_per_frame) as u16;
        }
    }
}

#[derive(Clone)]
pub struct SpriteAnimation {
    pub current_frame: u16,
    pub tile_range: RangeInclusive<u16>,
    pub millis_per_frame: u64,
    pub millis_passed: u64,
}