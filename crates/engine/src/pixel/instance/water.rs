use crate::pixel::ice::Ice;
use crate::pixel::steam::Steam;
use crate::pixel::{PixelFundamental, PixelInstance, PixelInteract, PixelType};

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Water {
    pub temp: u8,
}

impl Default for Water {
    fn default() -> Self {
        Self { temp: 20 }
    }
}

impl Water {
    pub fn is_burning(&self) -> bool {
        self.temp >= 30
    }
    pub fn is_frozen(&self) -> bool {
        self.temp <= 10
    }
}

impl PixelFundamental for Water {
    fn name(&self) -> &'static str {
        "Water"
    }

    fn pixel_type(&self) -> PixelType {
        PixelType::Liquid(10)
    }

    fn friction(&self) -> i16 {
        5
    }

    fn update(&mut self) -> Option<PixelInstance> {
        if self.is_burning() {
            Some(Steam::default().into())
        } else if self.is_frozen() {
            Some(Ice::default().into())
        } else {
            None
        }
    }
}

impl PixelInteract for Water {
    fn interact(&mut self, target: PixelInstance) {
        match target {
            PixelInstance::Fire(_) | PixelInstance::EternalFire(_) => {
                if !self.is_burning() {
                    self.temp += 2;
                }
            }
            PixelInstance::Wood(val) => {
                if val.is_burning() && !self.is_burning() {
                    self.temp += 2;
                }
            }
            PixelInstance::Ice(_) => {
                if !self.is_frozen() {
                    self.temp -= 2;
                }
            }
            _ => {}
        }
    }
}
