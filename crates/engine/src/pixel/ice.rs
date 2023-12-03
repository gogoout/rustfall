use crate::pixel::water::Water;
use crate::pixel::{Pixel, PixelFundamental, PixelInteract, PixelState, PixelType};

#[derive(Debug, Clone, Copy, Eq, PartialEq, Default)]
pub struct Ice {
    pub temp: u8,
    state: PixelState,
}

impl Ice {
    pub fn is_burning(&self) -> bool {
        self.temp >= 99
    }
}

impl PixelFundamental for Ice {
    fn name(&self) -> &'static str {
        "Ice"
    }

    fn pixel_type(&self) -> PixelType {
        PixelType::Wall
    }

    fn state(&self) -> &PixelState {
        &self.state
    }

    fn state_mut(&mut self) -> &mut PixelState {
        &mut self.state
    }

    fn update(&mut self) -> Option<Pixel> {
        if self.is_burning() {
            Some(Water::default().into())
        } else {
            None
        }
    }
}

impl PixelInteract for Ice {
    fn interact(&mut self, target: Pixel) {
        match target {
            Pixel::Fire(_) | Pixel::EternalFire(_) => {
                if !self.is_burning() {
                    self.temp += 20;
                }
            }
            Pixel::Water(_) => {
                if !self.is_burning() {
                    self.temp += 10;
                }
            }
            Pixel::Steam(_) => {
                if !self.is_burning() {
                    self.temp += 15;
                }
            }
            _ => {}
        }
    }
}
