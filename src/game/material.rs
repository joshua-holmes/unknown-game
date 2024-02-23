#[derive(Debug, Copy, Clone)]
pub enum Material {
    EmptySpace = 0x00,
    Sand = 0x01,
    Dirt = 0x02,
}
impl Material {
    pub fn properties(&self) -> MaterialProperties {
        match self {
            Material::EmptySpace => MaterialProperties {
                drag: 1.
            },
            Material::Sand => MaterialProperties {
                drag: 0.8
            },
            Material::Dirt => MaterialProperties {
                drag: 0.8
            },
        }
    }
}

pub struct MaterialProperties {
    /// value should be between 0.0 and 1.0 exclusive where 1.0 is no drag and
    /// 0.0 is max drag (material will not move)
    pub drag: f64,
}
