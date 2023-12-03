use crate::pixel::ice::Ice;
use crate::pixel::steam::Steam;
use crate::pixel::{Pixel, PixelFundamental, PixelInteract, PixelState, PixelType};

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Water {
    pub temp: u8,
    state: PixelState,
}

impl Default for Water {
    fn default() -> Self {
        Self {
            temp: 20,
            state: Default::default(),
        }
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

    fn state(&self) -> &PixelState {
        &self.state
    }

    fn state_mut(&mut self) -> &mut PixelState {
        &mut self.state
    }

    fn update(&mut self) -> Option<Pixel> {
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
    fn interact(&mut self, target: Pixel) {
        match target {
            Pixel::Fire(_) | Pixel::EternalFire(_) => {
                if !self.is_burning() {
                    self.temp += 2;
                }
            }
            Pixel::Wood(val) => {
                if val.is_burning() && !self.is_burning() {
                    self.temp += 2;
                }
            }
            Pixel::Ice(_) => {
                if !self.is_frozen() {
                    self.temp -= 2;
                }
            }
            _ => {}
        }
    }
}
