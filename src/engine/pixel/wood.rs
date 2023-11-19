use crate::engine::pixel::void::Void;
use crate::engine::pixel::{Pixel, PixelFundamental, PixelInteract, PixelType};

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Wood {
    pub temp: u8,
    pub life: u8,
}

impl Default for Wood {
    fn default() -> Self {
        Self { temp: 0, life: 225 }
    }
}

impl Wood {
    pub fn is_burning(&self) -> bool {
        self.temp >= 99
    }
}

impl PixelFundamental for Wood {
    fn name(&self) -> &'static str {
        "Wood"
    }

    fn pixel_type(&self) -> PixelType {
        if self.life <= 30 {
            PixelType::Solid(9)
        } else {
            PixelType::Wall
        }
    }

    fn update(&mut self) -> Option<Pixel> {
        if self.is_burning() && self.life > 0 {
            self.life -= 1;
        }
        if self.life == 0 {
            Some(Void::default().into())
        } else {
            None
        }
    }
}

impl PixelInteract for Wood {
    fn interact(&mut self, target: Pixel) {
        match target {
            Pixel::Water(_) => {
                if self.is_burning() {
                    self.temp -= 20;
                }
            }
            Pixel::Ice(_) => {
                if self.is_burning() {
                    self.temp -= 30;
                }
            }
            Pixel::Fire(_) => {
                if !self.is_burning() {
                    self.temp += 20;
                }
            }
            Pixel::Wood(val) => {
                if val.is_burning() && !self.is_burning() {
                    self.temp += 20;
                }
            }
            _ => {}
        }
    }
}
