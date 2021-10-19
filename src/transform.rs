
#[derive(Default, Clone)]

pub struct Transform {
    pub position: Position,
    pub size: Size,
    pub rotation: f64,
    pub flip: Flip,
}

impl Transform {
    pub fn with_position(mut self, x: i32, y: i32) -> Self {
        self.position.x = x;
        self.position.y = y;
        self
    }
    pub fn with_size(mut self, width: u32, height: u32) -> Self {
        self.size.width = width;
        self.size.height = height;
        self
    }
    pub fn with_rotation(mut self, deg: f64) -> Self {
        self.rotation = deg;
        self
    }
    pub fn with_horizontal_flip(mut self) -> Self {
        self.flip.horizontal = true;
        self
    }
    pub fn with_vertical_flip(mut self) -> Self {
        self.flip.vertical = true;
        self
    }
}

#[derive(Default, Clone)]
pub struct Position { pub x: i32, pub y: i32 }


#[derive(Default, Clone)]
pub struct Size { pub width: u32, pub height: u32 }
    

#[derive(Default, Clone)]
pub struct Flip {
    pub horizontal: bool,
    pub vertical: bool,
}
