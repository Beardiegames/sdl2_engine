

#[derive(Default, Clone, Debug)]
pub struct MouseInput {
    pub x: i32,
    pub y: i32,
    pub left_button: bool,
    pub middle_button: bool,
    pub right_button: bool,
    pub wheel: (i32, i32),
}

impl MouseInput {
    pub(crate) fn new() -> Self {
        MouseInput {
            x: 0,
            y: 0,
            left_button: false,
            middle_button: false,
            right_button: false,
            wheel: (0, 0),
        }
    }
}


// MouseMotion {
//     timestamp: u32,
//     window_id: u32,
//     which: u32,
//     mousestate: MouseState,
//     x: i32,
//     y: i32,
//     xrel: i32,
//     yrel: i32,
// },
// MouseButtonDown {
//     timestamp: u32,
//     window_id: u32,
//     which: u32,
//     mouse_btn: MouseButton,
//     clicks: u8,
//     x: i32,
//     y: i32,
// },
// MouseButtonUp {
//     timestamp: u32,
//     window_id: u32,
//     which: u32,
//     mouse_btn: MouseButton,
//     clicks: u8,
//     x: i32,
//     y: i32,
// },
// MouseWheel {
//     timestamp: u32,
//     window_id: u32,
//     which: u32,
//     x: i32,
//     y: i32,
//     direction: MouseWheelDirection,
// },

