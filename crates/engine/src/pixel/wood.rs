use crate::pixel::void::Void;
use crate::pixel::{Pixel, PixelFundamental, PixelInteract, PixelState, PixelType};

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Wood {
    pub temp: u8,
    pub life: u8,
    state: PixelState,
}

impl Default for Wood {
    fn default() -> Self {
        Self {
            temp: 0,
            life: 225,
            state: Default::default(),
        }
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

    fn state(&self) -> &PixelState {
        &self.state
    }

    fn state_mut(&mut self) -> &mut PixelState {
        &mut self.state
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
            Pixel::Fire(_) | Pixel::EternalFire(_) => {
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
