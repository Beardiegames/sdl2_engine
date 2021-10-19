use std::ops::RangeInclusive;

use sdl2::rect::Rect;

use crate::transform::{Position, Size};


#[derive(Clone)]
pub(crate) struct PoolRect(pub(crate) Option<Rect>); 

impl Default for PoolRect {
    fn default() -> Self { 
        Self (Some(Rect::new(0,0,1,1)))
    }
}

pub struct SpriteBuilder(Sprite);

impl SpriteBuilder {
    pub fn new(texure_id: usize) -> Self {
        SpriteBuilder(Sprite {
            texure_id: texure_id, 

            tile_size: Size::default(),
            num_tile_cols: 4,
            animation: 0,
            animations: Vec::new(),

            src: PoolRect::default(),
            dst: PoolRect::default()
        })
    }
    pub fn with_tile_size(mut self, width: u32, height: u32) -> Self {
        self.0.tile_size = Size { width, height };
        self
    }

    pub fn with_column_count(mut self, num_tile_cols: u16)-> Self {
        self.0.num_tile_cols = num_tile_cols;
        self
    }
    pub fn with_start_animation(mut self, animation_index: usize)-> Self {
        self.0.animation = animation_index;
        self
    }
    pub fn with_animations(mut self, animations: Vec<SpriteAnimation>)-> Self {
        self.0.animations = animations;
        self
    }
    pub fn build(self) -> Option<Sprite> {
        Some(self.0)
    }
}


#[derive(Default, Clone)]
pub struct Sprite {
    pub texure_id: usize,

    pub tile_size: Size,
    pub num_tile_cols: u16,
    pub animation: usize,
    pub animations: Vec<SpriteAnimation>,

    pub(crate) src: PoolRect,
    pub(crate) dst: PoolRect,
}

impl Sprite {

    pub fn tile_position(&self) -> Position {
        if self.animation < self.animations.len() {
            let tile = self.animations[self.animation].tile_range.start() 
                + self.animations[self.animation].current_frame;

            let col = tile % self.num_tile_cols;
            let row = tile / self.num_tile_cols;

            Position {
               x: (col as u32 * self.tile_size.width) as i32, 
               y: (row as u32 * self.tile_size.height) as i32,
            }

        } else {
            Position{ x: 0, y: 0 }
        }
    }

    pub fn update_animation(&mut self, _delta_time: &u64) {
        if self.animation < self.animations.len() {

            // increment animation frame
            let anim = &mut self.animations[self.animation];
            
            anim.millis_passed += *_delta_time;
            anim.millis_passed = anim.millis_passed % 
                (anim.millis_per_frame * anim.tile_range.len() as u64);

            anim.current_frame = (anim.millis_passed / anim.millis_per_frame) as u16;

            // set tile position
            let tile = self.animations[self.animation].tile_range.start() 
                + self.animations[self.animation].current_frame;

            let col = tile % self.num_tile_cols;
            let row = tile / self.num_tile_cols;

            // update render positions
            if let Some(src) = &mut self.src.0 {
                src.set_x((col as u32 * self.tile_size.width) as i32);
                src.set_y((row as u32 * self.tile_size.height) as i32);
                src.set_width(self.tile_size.width);
                src.set_height(self.tile_size.height);
            }
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