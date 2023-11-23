pub mod eternal_fire;
pub mod fire;
pub mod ice;
pub mod rock;
pub mod sand;
pub mod steam;
pub mod void;
pub mod water;
pub mod wood;

use crate::pixel::eternal_fire::EternalFire;
use crate::pixel::fire::Fire;
use crate::pixel::ice::Ice;
use crate::pixel::rock::Rock;
use crate::pixel::sand::Sand;
use crate::pixel::steam::Steam;
use crate::pixel::void::Void;
use crate::pixel::water::Water;
use crate::pixel::wood::Wood;
use crate::sandbox::Sandbox;
use enum_dispatch::enum_dispatch;
use itertools::Itertools;
use rand::distributions::Distribution;
use rand::distributions::Uniform;
use rand::Rng;
use std::fmt::{Display, Formatter};
use std::sync::OnceLock;

/// Holds the type and density of a pixel
#[derive(Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum PixelType {
    /// Gas may move to top randomly
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
#[repr(u8)]
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
    pub fn gas_directions<R: Rng>(rng: &mut R) -> &'static [Direction] {
        static DIRECTIONS: OnceLock<Vec<[Direction; 5]>> = OnceLock::new();
        let v = DIRECTIONS.get_or_init(|| {
            let v = vec![
                [Direction::Up, Direction::UpLeft, Direction::UpRight],
                [Direction::Up, Direction::UpRight, Direction::UpLeft],
                [Direction::UpLeft, Direction::UpRight, Direction::Up],
                [Direction::UpLeft, Direction::Up, Direction::UpRight],
                [Direction::UpRight, Direction::UpLeft, Direction::Up],
                [Direction::UpRight, Direction::Up, Direction::UpLeft],
            ];

            v.into_iter()
                .flat_map(|arr| {
                    [
                        [arr[0], arr[1], arr[2], Direction::Left, Direction::Right],
                        [arr[0], arr[1], arr[2], Direction::Right, Direction::Left],
                    ]
                })
                .collect::<Vec<_>>()
        });

        static BETWEEN: OnceLock<Uniform<usize>> = OnceLock::new();
        let between = BETWEEN.get_or_init(|| Uniform::new(0, v.len()));

        let idx = between.sample(rng);
        v[idx].as_ref()
    }
    pub fn liquid_directions<R: Rng>(rng: &mut R) -> &'static [Direction] {
        static DIRECTIONS: OnceLock<Vec<[Direction; 5]>> = OnceLock::new();
        let v = DIRECTIONS.get_or_init(|| {
            let v1 = vec![
                [Direction::DownLeft, Direction::DownRight],
                [Direction::DownRight, Direction::DownLeft],
            ];
            let v2 = vec![
                [Direction::Left, Direction::Right],
                [Direction::Right, Direction::Left],
            ];

            v1.into_iter()
                .flat_map(|v1| {
                    v2.iter()
                        .map(|v2| [Direction::Down, v1[0], v1[1], v2[0], v2[1]])
                        .collect_vec()
                })
                .collect::<Vec<_>>()
        });

        static BETWEEN: OnceLock<Uniform<usize>> = OnceLock::new();
        let between = BETWEEN.get_or_init(|| Uniform::new(0, v.len()));

        let idx = between.sample(rng);
        v[idx].as_ref()
    }
    pub fn solid_directions<R: Rng>(rng: &mut R) -> &'static [Direction] {
        static DIRECTIONS: OnceLock<Vec<[Direction; 3]>> = OnceLock::new();
        let v = DIRECTIONS.get_or_init(|| {
            let v = vec![
                [Direction::DownLeft, Direction::DownRight],
                [Direction::DownRight, Direction::DownLeft],
            ];

            v.into_iter()
                .map(|v| [Direction::Down, v[0], v[1]])
                .collect::<Vec<_>>()
        });

        static BETWEEN: OnceLock<Uniform<usize>> = OnceLock::new();
        let between = BETWEEN.get_or_init(|| Uniform::new(0, v.len()));

        let idx = between.sample(rng);
        v[idx].as_ref()
    }
}

#[enum_dispatch]
pub trait PixelFundamental {
    fn name(&self) -> &'static str;

    fn pixel_type(&self) -> PixelType;

    fn update(&mut self) -> Option<Pixel> {
        None
    }

    fn tick_move(&self, x: usize, y: usize, sandbox: &mut Sandbox) -> Option<(usize, usize)> {
        let check_density = |sandbox: &Sandbox, density, dir: Direction, reverse: bool| {
            sandbox
                .get_neighbour_pixel(x, y, dir)
                .and_then(|(x, y, p)| match p.is_moved() {
                    true => None,
                    false => Some((x, y, p.pixel().pixel_type())),
                })
                .and_then(|(x, y, p)| match p {
                    PixelType::Solid(td) | PixelType::Gas(td) | PixelType::Liquid(td) => {
                        match (density == td, density > td, reverse) {
                            (true, _, _) => None,
                            (false, true, false) => Some((x, y)),
                            (false, false, true) => Some((x, y)),
                            _ => None,
                        }
                    }
                    PixelType::Wall => None,
                    PixelType::Void => Some((x, y)),
                })
        };

        match self.pixel_type() {
            PixelType::Gas(density) => Direction::gas_directions(&mut sandbox.rng)
                .iter()
                .find_map(|dir| check_density(sandbox, density, *dir, true)),
            PixelType::Liquid(density) => Direction::liquid_directions(&mut sandbox.rng)
                .iter()
                .find_map(|dir| check_density(sandbox, density, *dir, false)),
            PixelType::Solid(density) => Direction::solid_directions(&mut sandbox.rng)
                .iter()
                .find_map(|dir| check_density(sandbox, density, *dir, false)),
            PixelType::Wall | PixelType::Void => None,
        }
    }
}

#[enum_dispatch]
pub trait PixelInteract {
    fn interact(&mut self, _target: Pixel) {}
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, strum_macros::EnumIter)]
#[repr(u8)]
#[enum_dispatch(PixelInteract, PixelFundamental)]
pub enum Pixel {
    Steam(Steam),
    Sand(Sand),
    Rock(Rock),
    Water(Water),
    Ice(Ice),
    Fire(Fire),
    EternalFire(EternalFire),
    Wood(Wood),
    Void(Void),
}

impl Default for Pixel {
    fn default() -> Self {
        Void::default().into()
    }
}

impl Display for Pixel {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}
