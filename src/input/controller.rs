use sdl2::controller::Axis;
use sdl2::controller::Button;


#[derive(Default, Clone, Debug)]
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


#[derive(Default, Clone, Debug)]
pub struct ControllerInput {
    pub id: u32,

    pub axis_LeftX: i16,
    pub axis_LeftY: i16,
    pub axis_RightX: i16,
    pub axis_RightY: i16,
    pub axis_TriggerLeft: i16,
    pub axis_TriggerRight: i16,

    pub button_A: bool,
    pub button_B: bool,
    pub button_X: bool,
    pub button_Y: bool,
    pub button_Back: bool,
    pub button_Guide: bool,
    pub button_Start: bool,
    pub button_LeftStick: bool,
    pub button_RightStick: bool,
    pub button_LeftShoulder: bool,
    pub button_RightShoulder: bool,
    pub button_DPadUp: bool,
    pub button_DPadDown: bool,
    pub button_DPadLeft: bool,
    pub button_DPadRight: bool,
}

impl ControllerInput {

    pub fn new(id: u32) -> Self {
        ControllerInput {
            id,
        
            axis_LeftX: 0,
            axis_LeftY: 0,
            axis_RightX: 0,
            axis_RightY: 0,
            axis_TriggerLeft: 0,
            axis_TriggerRight: 0,
        
            button_A: false,
            button_B: false,
            button_X: false,
            button_Y: false,
            button_Back: false,
            button_Guide: false,
            button_Start: false,
            button_LeftStick: false,
            button_RightStick: false,
            button_LeftShoulder: false,
            button_RightShoulder: false,
            button_DPadUp: false,
            button_DPadDown: false,
            button_DPadLeft: false,
            button_DPadRight: false,
        }
    }

    pub fn set_axis(&mut self, axis: &Axis, value: i16) {
        match *axis {
            Axis::LeftX => self.axis_LeftX = value,
            Axis::LeftY => self.axis_LeftY = value,
            Axis::RightX => self.axis_RightX = value,
            Axis::RightY => self.axis_RightY = value,
            Axis::TriggerLeft => self.axis_TriggerLeft = value,
            Axis::TriggerRight => self.axis_TriggerRight = value,
            _ => {},
        }
    }

    pub fn set_button(&mut self, button: &Button, value: bool) {
        match *button {
            Button::A => self.button_A = value,
            Button::B => self.button_B = value,
            Button::X => self.button_X = value,
            Button::Y => self.button_Y = value,
            Button::Back => self.button_Back = value,
            Button::Guide => self.button_Guide = value,
            Button::Start => self.button_Start = value,
            Button::LeftStick => self.button_LeftStick = value,
            Button::RightStick => self.button_RightStick = value,
            Button::LeftShoulder => self.button_LeftShoulder = value,
            Button::RightShoulder => self.button_RightShoulder = value,
            Button::DPadUp => self.button_DPadUp = value,
            Button::DPadDown => self.button_DPadDown = value,
            Button::DPadLeft => self.button_DPadLeft = value,
            Button::DPadRight => self.button_DPadRight = value,
            _ => {},
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