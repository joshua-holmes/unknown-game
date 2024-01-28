pub enum CanvasError {
    ResolutionTooSmall,
    ResolutionTooLarge,
}

pub struct Resolution {
    pub height: u32,
    pub width: u32,
}

pub struct Canvas {
    pub resolution: Resolution,
    pub pixels: Vec<Vec<u8>>,
}
impl Canvas {
    pub fn new(pixels: Vec<Vec<u8>>) -> Result<Self, CanvasError> {
        if pixels.len() == 0 {
            return Err(CanvasError::ResolutionTooSmall);
        }
        let resolution = Resolution {
            height: pixels.len() as u32,
            width: pixels[0].len() as u32,
        };
        Ok(Self {
            resolution,
            pixels
        })
    }
}
