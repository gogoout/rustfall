use crate::engine::pixel::{PixelFundamental, PixelInteract, PixelType};

#[derive(Debug, Clone, Copy, Eq, PartialEq, Default)]
pub struct EternalFire;

impl PixelFundamental for EternalFire {
    fn name(&self) -> &'static str {
        "Eternal fire"
    }

    fn pixel_type(&self) -> PixelType {
        PixelType::Wall
    }
}

impl PixelInteract for EternalFire {}
