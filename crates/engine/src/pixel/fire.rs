use crate::pixel::void::Void;
use crate::pixel::{Pixel, PixelFundamental, PixelInteract, PixelState, PixelType};

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Fire {
    life: u8,
    state: PixelState,
}
impl Default for Fire {
    fn default() -> Self {
        Self {
            life: 60,
            state: Default::default(),
        }
    }
}

impl PixelFundamental for Fire {
    fn name(&self) -> &'static str {
        "Fire"
    }

    fn pixel_type(&self) -> PixelType {
        PixelType::Gas(-1)
    }

    fn state(&self) -> &PixelState {
        &self.state
    }

    fn state_mut(&mut self) -> &mut PixelState {
        &mut self.state
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
