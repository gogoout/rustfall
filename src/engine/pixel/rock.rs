use crate::engine::pixel::{PixelFundamental, PixelInteract, PixelType};

#[derive(Debug, Clone, Copy, Eq, PartialEq, Default)]
pub struct Rock;

impl PixelFundamental for Rock {
    fn name(&self) -> &'static str {
        "Rock"
    }

    fn pixel_type(&self) -> PixelType {
        PixelType::Wall
    }
}

impl PixelInteract for Rock {}
