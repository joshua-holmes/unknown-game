use winit::dpi::PhysicalSize;

#[derive(Debug)]
#[allow(dead_code)]
pub enum CanvasError {
    ResolutionTooSmall,
    ResolutionTooLarge,
}

#[derive(Debug)]
pub struct Resolution {
    pub height: u32,
    pub width: u32,
}
impl From<PhysicalSize<u32>> for Resolution {
    fn from(value: PhysicalSize<u32>) -> Self {
        Self {
            height: value.height,
            width: value.width
        }
    }
}

#[derive(Debug)]
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

    pub fn new_mock_from_resolution(resolution: &Resolution) -> Self {
        Self::new(
            (0..resolution.height).map(|i| {
                (0..resolution.width).map(|_| {
                    (i % u8::MAX as u32) as u8
                }).collect()
            }).collect()
        )
        .unwrap()
    }

    pub fn flattened_pixels(&self) -> Vec<u8> {
        (0..self.resolution.width * self.resolution.height).map(|i| {
            let row = (i / self.resolution.width) as usize;
            let col = (i % self.resolution.width) as usize;
            self.pixels[row][col]
        }).collect()
    }
}
