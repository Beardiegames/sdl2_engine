
#[derive(Default, Clone)]

pub struct Transform {
    pub x: f32, 
    pub y: f32,
    pub z: f32,
    pub width: u32, 
    pub height: u32,
    pub rotation: f64,
    pub flip_horizontal: bool,
    pub flip_vertical: bool,
}

impl Transform {
    pub fn with_position(mut self, x: f32, y: f32) -> Self {
        self.x = x;
        self.y = y;
        self
    }
    pub fn with_depth(mut self, z: f32) -> Self {
        self.z = z;
        self
    }
    pub fn with_size(mut self, width: u32, height: u32) -> Self {
        self.width = width;
        self.height = height;
        self
    }
    pub fn with_rotation(mut self, deg: f64) -> Self {
        self.rotation = deg;
        self
    }
    pub fn with_horizontal_flip(mut self) -> Self {
        self.flip_horizontal = true;
        self
    }
    pub fn with_vertical_flip(mut self) -> Self {
        self.flip_vertical = true;
        self
    }
}

// #[derive(Default, Clone)]
// pub struct Position { pub x: i32, pub y: i32 }


// #[derive(Default, Clone)]
// pub struct Size { pub width: u32, pub height: u32 }
    

// #[derive(Default, Clone)]
// pub struct Flip {
//     pub horizontal: bool,
//     pub vertical: bool,
// }
