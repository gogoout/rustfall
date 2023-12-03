use crate::pixel::{PixelFundamental, PixelInteract, PixelState, PixelType};

#[derive(Debug, Clone, Copy, Eq, PartialEq, Default)]
pub struct Rock {
    state: PixelState,
}

impl PixelFundamental for Rock {
    fn name(&self) -> &'static str {
        "Rock"
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

impl PixelInteract for Rock {}
