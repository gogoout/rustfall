use crate::pixel::{PixelFundamental, PixelInteract, PixelState, PixelType};

#[derive(Debug, Clone, Copy, Eq, PartialEq, Default)]
pub struct EternalFire {
    state: PixelState,
}

impl PixelFundamental for EternalFire {
    fn name(&self) -> &'static str {
        "Eternal fire"
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
}

impl PixelInteract for EternalFire {}
