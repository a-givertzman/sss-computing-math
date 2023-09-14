pub struct CrossSection {
    id: i32,
    I_x: f64,
    I_y: f64,
    z_up: f64,
    z_down: f64,
}

impl CrossSection {
    pub fn new(id: i32, I_x: f64, I_y: f64, z_up: f64, z_down: f64) -> Self {
        CrossSection { id, I_x, I_y, z_up, z_down }
    }
}