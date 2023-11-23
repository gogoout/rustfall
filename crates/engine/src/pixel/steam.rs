use crate::pixel::water::Water;
use crate::pixel::{Pixel, PixelFundamental, PixelInteract, PixelType};

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Steam {
    temp: u8,
}
impl Default for Steam {
    fn default() -> Self {
        Self { temp: 200 }
    }
}

impl PixelFundamental for Steam {
    fn name(&self) -> &'static str {
        "Steam"
    }

    fn pixel_type(&self) -> PixelType {
        PixelType::Gas(-10)
    }
    fn update(&mut self) -> Option<Pixel> {
        if self.temp < 10 {
            Some(Water::default().into())
        } else {
            None
        }
    }
}

impl PixelInteract for Steam {
    fn interact(&mut self, target: Pixel) {
        match target {
            Pixel::Water(_) | Pixel::Steam(_) => {
                if self.temp > 0 {
                    self.temp -= 1;
                }
            }
            Pixel::Ice(_) => {
                if self.temp > 1 {
                    self.temp -= 2;
                } else {
                    self.temp = 0;
                }
            }
            _ => {}
        }
    }
}
