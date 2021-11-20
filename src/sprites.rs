use std::ops::RangeInclusive;

use sdl2::rect::Rect;

use crate::transform::Transform;


#[derive(Clone, Debug)]
pub(crate) struct PoolRect(pub(crate) Option<Rect>); 

impl Default for PoolRect {
    fn default() -> Self { 
        Self (Some(Rect::new(0,0,1,1)))
    }
}


#[derive(Clone)]
pub struct TileSize { pub width: u32, pub height: u32 }

impl Default for TileSize {
    fn default() -> Self { Self { width: 64, height: 64 } }
}


#[derive(Default, Clone)]
pub struct TilePosition { pub x: i32, pub y: i32 }


pub struct SpriteBuilder(Sprite);

impl SpriteBuilder {
    pub fn new(texture_id: usize) -> Self {
        SpriteBuilder(Sprite {
            texture_id: texture_id, 

            tile_size: TileSize::default(),
            num_tile_cols: 4,
            animation: 0,
            animations: Vec::new(),

            // src: PoolRect::default(),
            // dst: PoolRect::default()
        })
    }
    pub fn with_tile_size(mut self, width: u32, height: u32) -> Self {
        self.0.tile_size = TileSize { width, height };
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
    pub fn build(self) -> Sprite {
        self.0
    }
}

#[derive(Default, Clone, Debug)]
pub struct Image {
    pub(crate) texture_id: usize,
    pub(crate) src: PoolRect,
    pub(crate) dst: PoolRect,
    pub(crate) transform: Transform,
}


#[derive(Default, Clone)]
pub struct Sprite {
    pub texture_id: usize,
     
    pub tile_size: TileSize,
    pub num_tile_cols: u16,
    pub animation: usize,
    pub animations: Vec<SpriteAnimation>,

    //pub(crate) src: PoolRect,
    //pub(crate) dst: PoolRect,
}

impl Sprite {

    // pub fn tile_position(&self) -> TilePosition {
    //     if self.animation < self.animations.len() {
    //         let tile = self.animations[self.animation].tile_range.start() 
    //             + self.animations[self.animation].current_frame;

    //         let col = tile % self.num_tile_cols;
    //         let row = tile / self.num_tile_cols;

    //         TilePosition {
    //            x: (col as u32 * self.tile_size.width) as i32, 
    //            y: (row as u32 * self.tile_size.height) as i32,
    //         }

    //     } else {
    //         TilePosition::default()
    //     }
    // }

    pub fn update_animation(&mut self, frame_dration: &u32, image: &mut Image) {
        if self.animation < self.animations.len() {

            // increment animation frame
            let anim = &mut self.animations[self.animation];
            
            anim.millis_passed += *frame_dration; //(*_delta_time * 1_000.0) as u64;
            anim.millis_passed = anim.millis_passed % 
                (anim.millis_per_frame * anim.tile_range.len() as u32);

            anim.current_frame = (anim.millis_passed / anim.millis_per_frame) as u16;

            // set tile position
            let tile = self.animations[self.animation].tile_range.start() 
                + self.animations[self.animation].current_frame;

            let col = tile % self.num_tile_cols;
            let row = tile / self.num_tile_cols;

            // update render positions
            image.texture_id = self.texture_id;

            if let Some(src) = &mut image.src.0 {
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
    pub millis_per_frame: u32,
    pub millis_passed: u32,
}