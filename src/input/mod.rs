mod keyboard;

use sdl2::keyboard::Keycode;
pub use keyboard::{ Key, KeyboardInput };

pub struct Input {
    pub keyboard: KeyboardInput,
}

impl Input {
    pub fn new() -> Self {
        Input { 
            keyboard: KeyboardInput::new(),
        }
    }
}

pub(crate) fn map_keys(input: &mut KeyboardInput, keycode: Keycode, state: bool) {
    match keycode {
        Keycode::Right => input.set_state(Key::Right, state),
        Keycode::Left => input.set_state(Key::Left, state),
        Keycode::Down => input.set_state(Key::Down, state),
        Keycode::Up => input.set_state(Key::Up, state),

        Keycode::LShift => input.set_state(Key::ShiftLeft, state),
        Keycode::RShift => input.set_state(Key::ShiftRight, state),
        Keycode::LCtrl => input.set_state(Key::CtrlLeft, state),
        Keycode::RCtrl => input.set_state(Key::CtrlRight, state),
        Keycode::LAlt => input.set_state(Key::AltLeft, state),
        Keycode::RAlt => input.set_state(Key::AltRight, state),
        Keycode::Escape => input.set_state(Key::Esc, state),
        Keycode::Return => input.set_state(Key::Enter, state),
        Keycode::Space => input.set_state(Key::Space, state),

        Keycode::Num0 => input.set_state(Key::Nr0, state),
        Keycode::Num1 => input.set_state(Key::Nr1, state),
        Keycode::Num2 => input.set_state(Key::Nr2, state),
        Keycode::Num3 => input.set_state(Key::Nr3, state),
        Keycode::Num4 => input.set_state(Key::Nr4, state),
        Keycode::Num5 => input.set_state(Key::Nr5, state),
        Keycode::Num6 => input.set_state(Key::Nr6, state),
        Keycode::Num7 => input.set_state(Key::Nr7, state),
        Keycode::Num8 => input.set_state(Key::Nr8, state),
        Keycode::Num9 => input.set_state(Key::Nr9, state),

        Keycode::Q => input.set_state(Key::Q, state),
        Keycode::W => input.set_state(Key::W, state),
        Keycode::E => input.set_state(Key::E, state),
        Keycode::R => input.set_state(Key::R, state),
        Keycode::T => input.set_state(Key::T, state),
        Keycode::Y => input.set_state(Key::Y, state),
        Keycode::U => input.set_state(Key::U, state),
        Keycode::I => input.set_state(Key::I, state),
        Keycode::O => input.set_state(Key::O, state),
        Keycode::P => input.set_state(Key::P, state),
        Keycode::A => input.set_state(Key::A, state),
        Keycode::S => input.set_state(Key::S, state),
        Keycode::D => input.set_state(Key::D, state),
        Keycode::F => input.set_state(Key::F, state),
        Keycode::G => input.set_state(Key::G, state),
        Keycode::H => input.set_state(Key::H, state),
        Keycode::J => input.set_state(Key::J, state),
        Keycode::K => input.set_state(Key::K, state),
        Keycode::L => input.set_state(Key::L, state),
        Keycode::Z => input.set_state(Key::Z, state),
        Keycode::X => input.set_state(Key::X, state),
        Keycode::C => input.set_state(Key::C, state),
        Keycode::V => input.set_state(Key::V, state),
        Keycode::B => input.set_state(Key::B, state),
        Keycode::N => input.set_state(Key::N, state),
        Keycode::M => input.set_state(Key::M, state),
        _ => {},
    }
}