use crate::pixel::fire::Fire;
use crate::pixel::{Pixel, PixelFundamental, PixelInteract, PixelType};

#[derive(Debug, Clone, Copy, Eq, PartialEq, Default)]
pub struct Void {
    burn: bool,
}

impl PixelFundamental for Void {
    fn name(&self) -> &'static str {
        "Void"
    }

    fn pixel_type(&self) -> PixelType {
        PixelType::Void
    }

    fn update(&mut self) -> Option<Pixel> {
        if self.burn {
            Some(Fire::default().into())
        } else {
            None
        }
    }
}

impl PixelInteract for Void {
    fn interact(&mut self, target: Pixel) {
        match target {
            Pixel::EternalFire(_) => self.burn = true,
            _ => {}
        }
    }
}
