pub mod rock;
pub mod sand;
pub mod steam;
pub mod void;
pub mod water;

use crate::engine::pixel::rock::Rock;
use crate::engine::pixel::sand::Sand;
use crate::engine::pixel::steam::Steam;
use crate::engine::pixel::void::Void;
use crate::engine::pixel::water::Water;
use std::fmt::{Display, Formatter};

/// Holds the type and density of a pixel
#[derive(Debug, Eq, PartialEq)]
pub enum PixelType {
    /// Gas may move in any direction randomly
    Gas(i8),
    /// Liquid moves down, down left, down right, left, or right
    Liquid(i8),
    /// Solid moves down, down left, or down right
    Solid(i8),
    /// Wall doesn't move
    Wall,
    /// Empty pixel
    Void,
}

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
    UpLeft,
    UpRight,
    DownLeft,
    DownRight,
}

impl Direction {
    pub fn shuffled_slice() -> [Direction; 8] {
        use rand::seq::SliceRandom;
        let mut directions = [
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
            Direction::UpLeft,
            Direction::UpRight,
            Direction::DownLeft,
            Direction::DownRight,
        ];
        directions.shuffle(&mut rand::thread_rng());
        directions
    }
}

pub struct AdjacentPixels<'a> {
    pub top: Option<&'a Pixel>,
    pub bottom: Option<&'a Pixel>,
    pub left: Option<&'a Pixel>,
    pub right: Option<&'a Pixel>,
    pub top_left: Option<&'a Pixel>,
    pub top_right: Option<&'a Pixel>,
    pub bottom_left: Option<&'a Pixel>,
    pub bottom_right: Option<&'a Pixel>,
}

pub trait BasicPixel {
    fn name(&self) -> &'static str {
        std::any::type_name::<Self>()
    }

    fn pixel_type(&self) -> PixelType;

    fn tick_move(&self, adjacent_pixels: &AdjacentPixels) -> Option<Direction> {
        let check_density = |density, target: &Option<&Pixel>, dir: Direction, reverse: bool| {
            target.and_then(|target| match target.pixel_type() {
                PixelType::Solid(td) | PixelType::Gas(td) | PixelType::Liquid(td) => {
                    match (density == td, density > td, reverse) {
                        (true, _, _) => None,
                        (false, true, false) => Some(dir),
                        (false, false, true) => Some(dir),
                        _ => None,
                    }
                }
                PixelType::Wall => None,
                PixelType::Void => Some(dir),
            })
        };

        match self.pixel_type() {
            PixelType::Gas(density) => {
                Direction::shuffled_slice()
                    .iter()
                    .find_map(|dir| match dir {
                        Direction::Up => check_density(density, &adjacent_pixels.top, *dir, true),
                        Direction::Left => {
                            check_density(density, &adjacent_pixels.left, *dir, true)
                        }
                        Direction::Right => {
                            check_density(density, &adjacent_pixels.right, *dir, true)
                        }
                        Direction::UpLeft => {
                            check_density(density, &adjacent_pixels.top_left, *dir, true)
                        }
                        Direction::UpRight => {
                            check_density(density, &adjacent_pixels.top_right, *dir, true)
                        }
                        Direction::Down => {
                            check_density(density, &adjacent_pixels.bottom, *dir, false)
                        }
                        Direction::DownLeft => {
                            check_density(density, &adjacent_pixels.bottom_left, *dir, false)
                        }
                        Direction::DownRight => {
                            check_density(density, &adjacent_pixels.bottom_right, *dir, false)
                        }
                    })
            }
            PixelType::Liquid(density) => {
                check_density(density, &adjacent_pixels.bottom, Direction::Down, false)
                    .or_else(|| {
                        check_density(
                            density,
                            &adjacent_pixels.bottom_left,
                            Direction::DownLeft,
                            false,
                        )
                    })
                    .or_else(|| {
                        check_density(
                            density,
                            &adjacent_pixels.bottom_right,
                            Direction::DownRight,
                            false,
                        )
                    })
                    .or_else(|| {
                        check_density(density, &adjacent_pixels.left, Direction::Left, false)
                    })
                    .or_else(|| {
                        check_density(density, &adjacent_pixels.right, Direction::Right, false)
                    })
            }
            PixelType::Solid(density) => {
                check_density(density, &adjacent_pixels.bottom, Direction::Down, false)
                    .or_else(|| {
                        check_density(
                            density,
                            &adjacent_pixels.bottom_left,
                            Direction::DownLeft,
                            false,
                        )
                    })
                    .or_else(|| {
                        check_density(
                            density,
                            &adjacent_pixels.bottom_right,
                            Direction::DownRight,
                            false,
                        )
                    })
            }
            PixelType::Wall | PixelType::Void => None,
        }
    }
}

#[macro_export]
macro_rules! implement_basic_pixel {
    ($type_name:ty,$pixel_type:expr, $pixel_pat:path) => {
        impl BasicPixel for $type_name {
            fn pixel_type(&self) -> PixelType {
                $pixel_type
            }
        }

        impl From<$type_name> for Pixel {
            fn from(val: $type_name) -> Self {
                $pixel_pat(val)
            }
        }

        impl TryFrom<Pixel> for $type_name {
            type Error = anyhow::Error;

            fn try_from(value: Pixel) -> Result<Self, Self::Error> {
                match value {
                    $pixel_pat(val) => Ok(val),
                    _ => Err(anyhow!("{} is not a Pixel", value)),
                }
            }
        }
    };
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Pixel {
    Steam(Steam),
    Sand(Sand),
    Rock(Rock),
    Water(Water),
    Void(Void),
}

impl Default for Pixel {
    fn default() -> Self {
        Self::Void(Void)
    }
}

impl BasicPixel for Pixel {
    fn name(&self) -> &'static str {
        match self {
            Pixel::Steam(val) => val.name(),
            Pixel::Sand(val) => val.name(),
            Pixel::Rock(val) => val.name(),
            Pixel::Water(val) => val.name(),
            Pixel::Void(val) => val.name(),
        }
    }

    fn pixel_type(&self) -> PixelType {
        match self {
            Pixel::Steam(val) => val.pixel_type(),
            Pixel::Sand(val) => val.pixel_type(),
            Pixel::Rock(val) => val.pixel_type(),
            Pixel::Water(val) => val.pixel_type(),
            Pixel::Void(val) => val.pixel_type(),
        }
    }
}

impl Display for Pixel {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}
