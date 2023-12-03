use crate::pixel::fire::Fire;
use crate::pixel::{Pixel, PixelFundamental, PixelInteract, PixelState, PixelType};

#[derive(Debug, Clone, Copy, Eq, PartialEq, Default)]
pub struct Void {
    burn: bool,
    state: PixelState,
}

impl PixelFundamental for Void {
    fn name(&self) -> &'static str {
        "Void"
    }

    fn pixel_type(&self) -> PixelType {
        PixelType::Void
    }

    fn state(&self) -> &PixelState {
        &self.state
    }

    fn state_mut(&mut self) -> &mut PixelState {
        &mut self.state
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
