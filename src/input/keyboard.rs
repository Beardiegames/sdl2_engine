

#[derive(Clone)]
pub enum Key {
    Esc = 0, Left = 1, Right = 2, Up = 3, Down = 4,

    ShiftLeft = 5, ShiftRight = 6, CtrlLeft = 7, CtrlRight = 8, AltLeft = 9,
    AltRight = 10, Delete = 11, Enter = 12, Space = 13,

    Nr1 = 14, Nr2 = 15, Nr3 = 16, Nr4 = 17, Nr5 = 18, Nr6 = 19, Nr7 = 20,
    Nr8 = 21, Nr9 = 22, Nr0 = 23,
    
    Q = 24, W = 25, E = 26, R = 27, T = 28, Y = 29, U = 30, I = 31, O = 32, P = 33,
    A = 34, S = 35, D = 36, F = 37, G = 38, H = 39, J = 40, K = 41, L = 42, Z = 43,
    X = 44, C = 45, V = 46, B = 47, N = 48, M = 49,
}

pub struct KeyboardInput {
    pub states: Vec<bool>,
    pub changed: Vec<bool>,
    recent_activity: Vec<usize>,
}

impl KeyboardInput {
    pub fn new() -> Self {
        KeyboardInput {
            states: vec![false; 50],
            changed: vec![false; 50],
            recent_activity: Vec::with_capacity(50),
        }
    }

    pub(crate) fn releave_activity(&mut self) {
        while let Some(activity) = self.recent_activity.pop() {
            self.changed[activity] = false;
        }
    }

    pub(crate) fn set_state(&mut self, key: Key, state: bool) {
        let index = key as usize;
        self.changed[index] = self.states[index] != state;
        self.states[index] = state;
        
        if self.changed[index] { self.recent_activity.push(index); }
    }

    pub fn pressed(&self, key: Key) -> bool {
        let index = key as usize;
        self.states[index] && self.changed[index]
    }
    pub fn down(&self, key: Key) -> bool {
        self.states[key as usize]
    }
    pub fn released(&self, key: Key) -> bool {
        let index = key as usize;
        !self.states[index] && self.changed[index]
    }
}

