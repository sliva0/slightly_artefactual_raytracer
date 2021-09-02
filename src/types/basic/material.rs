#[derive(Debug, Copy, Clone)]
pub struct Material {
    pub ambient: f64,
    pub smoothness: i32,
    pub flare_intensity: f64,
    pub specularity: f64,
}

impl Material {
    pub const ERR_MATERIAL: Self = Material {
        ambient: 1.0,
        smoothness: 0,
        flare_intensity: 0.0,
        specularity: 0.0,
    };
}
