use crate::pixel::{PixelFundamental, PixelInteract, PixelType};

#[derive(Debug, Clone, Copy, Eq, PartialEq, Default)]
pub struct Sand;

impl PixelFundamental for Sand {
    fn name(&self) -> &'static str {
        "Sand"
    }

    fn pixel_type(&self) -> PixelType {
        PixelType::Solid(50)
    }
}

impl PixelInteract for Sand {}
