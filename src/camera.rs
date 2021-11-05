pub struct Camera {
    pub x: f32,
    pub y: f32,
    pub zoom: f32,
    pub(crate) zpow: f32,
}

impl Camera {
    pub fn set_power(&mut self) {
        self.zpow = self.zoom.powf(10.0);
    }
}