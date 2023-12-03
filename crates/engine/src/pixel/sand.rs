use crate::pixel::{PixelFundamental, PixelInteract, PixelState, PixelType};

#[derive(Debug, Clone, Copy, Eq, PartialEq, Default)]
pub struct Sand {
    state: PixelState,
}

impl PixelFundamental for Sand {
    fn name(&self) -> &'static str {
        "Sand"
    }

    fn pixel_type(&self) -> PixelType {
        PixelType::Solid(50)
    }

    fn friction(&self) -> i16 {
        15
    }

    fn state(&self) -> &PixelState {
        &self.state
    }

    fn state_mut(&mut self) -> &mut PixelState {
        &mut self.state
    }
}

impl PixelInteract for Sand {}
