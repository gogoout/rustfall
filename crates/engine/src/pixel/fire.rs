use crate::pixel::void::Void;
use crate::pixel::{Pixel, PixelFundamental, PixelInteract, PixelType};

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Fire {
    life: u8,
}
impl Default for Fire {
    fn default() -> Self {
        Self { life: 60 }
    }
}

impl PixelFundamental for Fire {
    fn name(&self) -> &'static str {
        "Fire"
    }

    fn pixel_type(&self) -> PixelType {
        PixelType::Gas(-1)
    }

    fn update(&mut self) -> Option<Pixel> {
        self.life -= 1;

        if self.life == 0 {
            Some(Void::default().into())
        } else {
            None
        }
    }
}

impl PixelInteract for Fire {}
