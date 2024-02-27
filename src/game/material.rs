#[derive(Debug, Copy, Clone)]
pub enum Material {
    EmptySpace = 0x00,
    Sand = 0x01,
    Dirt = 0x02,
}
impl Material {
    pub fn properties(&self) -> MaterialProperties {
        match self {
            Material::EmptySpace => MaterialProperties { drag: 1. },
            Material::Sand => MaterialProperties { drag: 20. },
            Material::Dirt => MaterialProperties { drag: 20. },
        }
    }
}

pub struct MaterialProperties {
    /// Higher number means more drag
    pub drag: f64,
}
