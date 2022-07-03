#[derive(Debug, Copy, Clone)]
pub enum MaterialType {
    DefaultType,
    ReflectiveType {
        reflectance: f64,
    },
    RefractiveType {
        surface_transparency: f64,
        index: f64,
        // TODO muddiness:
        // transparency: f64, 
        // color: Color,
    },
}

#[derive(Debug, Copy, Clone)]
pub struct Material {
    pub ambient: f64,
    pub diffuse: f64,
    pub specular: f64,
    pub shininess: i32,
    pub m_type: MaterialType,
}

impl Material {
    pub const ERR_MATERIAL: Self = Material {
        ambient: 1.0,
        diffuse: 0.0,
        specular: 0.0,
        shininess: 0,
        m_type: MaterialType::DefaultType,
    };
}
