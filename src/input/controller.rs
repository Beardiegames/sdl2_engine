use sdl2::controller::Axis;
use sdl2::controller::Button;

pub struct ControllerList(Vec<ControllerInput>);

impl ControllerList {
    pub fn new() -> Self {
        ControllerList(Vec::new())
    }

    pub fn add(&mut self, id: u32) {
        self.0.push(ControllerInput::new(id));
    }

    pub fn remove(&mut self, id: u32) {
        if let Some(index) = self.0.iter_mut().position(|x| x.id == id) 
        {
            self.0.remove(index);
        }
    }

    pub fn set_axis(&mut self, id: u32, axis: &Axis, value: i16) {
        if let Some(index) = self.0.iter_mut().position(|x| x.id == id) 
        {
            self.0[index].set_axis(axis, value);
        }   
    }

    pub fn set_button(&mut self, id: u32, button: &Button, value: bool) {
        if let Some(index) = self.0.iter_mut().position(|x| x.id == id) 
        {
            self.0[index].set_button(button, value);
        }   
    }
}

pub struct ControllerInput {
    pub id: u32,

    pub axis_left_x: i16,
    pub axis_left_y: i16,
    pub axis_right_x: i16,
    pub axis_right_y: i16,
    pub axis_trigger_left: i16,
    pub axis_trigger_right: i16,

    pub button_a: bool,
    pub button_b: bool,
    pub button_x: bool,
    pub button_y: bool,
    pub button_back: bool,
    pub button_guide: bool,
    pub button_start: bool,
    pub button_left_stick: bool,
    pub button_right_stick: bool,
    pub button_left_shoulder: bool,
    pub button_right_shoulder: bool,
    pub button_d_pad_up: bool,
    pub button_d_pad_down: bool,
    pub button_d_pad_left: bool,
    pub button_d_pad_right: bool,
}

impl ControllerInput {

    pub fn new(id: u32) -> Self {
        ControllerInput {
            id,
        
            axis_left_x: 0,
            axis_left_y: 0,
            axis_right_x: 0,
            axis_right_y: 0,
            axis_trigger_left: 0,
            axis_trigger_right: 0,
        
            button_a: false,
            button_b: false,
            button_x: false,
            button_y: false,
            button_back: false,
            button_guide: false,
            button_start: false,
            button_left_stick: false,
            button_right_stick: false,
            button_left_shoulder: false,
            button_right_shoulder: false,
            button_d_pad_up: false,
            button_d_pad_down: false,
            button_d_pad_left: false,
            button_d_pad_right: false,
        }
    }

    pub fn set_axis(&mut self, axis: &Axis, value: i16) {
        match *axis {
            Axis::LeftX => self.axis_left_x = value,
            Axis::LeftY => self.axis_left_y = value,
            Axis::RightX => self.axis_right_x = value,
            Axis::RightY => self.axis_right_y = value,
            Axis::TriggerLeft => self.axis_trigger_left = value,
            Axis::TriggerRight => self.axis_trigger_right = value,
            _ => {}
        }
    }

    pub fn set_button(&mut self, button: &Button, value: bool) {
        match *button {
            Button::A => self.button_a = value,
            Button::B => self.button_b = value,
            Button::X => self.button_x = value,
            Button::Y => self.button_y = value,
            Button::Back => self.button_back = value,
            Button::Guide => self.button_guide = value,
            Button::Start => self.button_start = value,
            Button::LeftStick => self.button_left_stick = value,
            Button::RightStick => self.button_right_stick = value,
            Button::LeftShoulder => self.button_left_shoulder = value,
            Button::RightShoulder => self.button_right_shoulder = value,
            Button::DPadUp => self.button_d_pad_up = value,
            Button::DPadDown => self.button_d_pad_down = value,
            Button::DPadLeft => self.button_d_pad_left = value,
            Button::DPadRight => self.button_d_pad_right = value,
            _ => {}
        }
    }
}



// ControllerAxisMotion {
//     timestamp: u32,
//     which: u32,
//     axis: Axis,
//     value: i16,
// },
// ControllerButtonDown {
//     timestamp: u32,
//     which: u32,
//     button: Button,
// },
// ControllerButtonUp {
//     timestamp: u32,
//     which: u32,
//     button: Button,
// },
// ControllerDeviceAdded {
//     timestamp: u32,
//     which: u32,
// },
// ControllerDeviceRemoved {
//     timestamp: u32,
//     which: u32,
// },
// ControllerDeviceRemapped {
//     timestamp: u32,
//     which: u32,
// },