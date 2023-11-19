use crate::engine::pixel::{PixelFundamental, PixelInteract, PixelType};

#[derive(Debug, Clone, Copy, Eq, PartialEq, Default)]
pub struct Void;

impl PixelFundamental for Void {
    fn name(&self) -> &'static str {
        "Void"
    }

    fn pixel_type(&self) -> PixelType {
        PixelType::Void
    }
}

impl PixelInteract for Void {}
