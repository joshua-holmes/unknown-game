#[derive(Debug, Copy, Clone)]
pub enum Material {
    EmptySpace = 0x00,
    Sand = 0x01,
    Dirt = 0x02,
    Blue = 0x03,
    Orange = 0x04,
}
impl Material {
    pub fn properties(&self) -> MaterialProperties {
        match self {
            Material::EmptySpace => MaterialProperties {
                drag: 0.,
                bounce: 0.,
            },
            Material::Sand => MaterialProperties {
                drag: 0.,
                bounce: 0.,
            },
            Material::Dirt => MaterialProperties {
                drag: 0.,
                bounce: 0.,
            },
            Material::Blue => MaterialProperties {
                drag: 0.,
                bounce: 0.,
            },
            Material::Orange => MaterialProperties {
                drag: 0.,
                bounce: 0.,
            },
        }
    }
}

pub struct MaterialProperties {
    /// Higher number means more drag
    /// Terminal velocity is gravity / drag
    /// Range is 0.0 - 1.0 inclusive
    pub drag: f64,

    /// Higher means more bounce
    /// Range is 0.0 - 1.0 inclusive
    pub bounce: f64,
}
