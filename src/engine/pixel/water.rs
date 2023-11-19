use crate::engine::pixel::steam::Steam;
use crate::engine::pixel::{Pixel, PixelFundamental, PixelInteract, PixelType};

#[derive(Debug, Clone, Copy, Eq, PartialEq, Default)]
pub struct Water {
    pub temp: u8,
}

impl Water {
    pub fn is_burning(&self) -> bool {
        self.temp >= 99
    }
}

impl PixelFundamental for Water {
    fn name(&self) -> &'static str {
        "Water"
    }

    fn pixel_type(&self) -> PixelType {
        PixelType::Liquid(10)
    }

    fn update(&mut self) -> Option<Pixel> {
        if self.is_burning() {
            Some(Steam::default().into())
        } else {
            None
        }
    }
}

impl PixelInteract for Water {
    fn interact(&mut self, target: Pixel) {
        match target {
            Pixel::Fire(_) => {
                if !self.is_burning() {
                    self.temp += 2;
                }
            }
            Pixel::Wood(val) => {
                if val.is_burning() && !self.is_burning() {
                    self.temp += 2;
                }
            }
            _ => {}
        }
    }
}
