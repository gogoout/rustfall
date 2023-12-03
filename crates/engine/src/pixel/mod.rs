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
use crate::sandbox::{Coordinate, SandboxControl};
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct PixelState {
    pub(crate) is_moved: bool,
    pub(crate) velocity: (i16, i16),
}

const MAX_VELOCITY: i16 = 5000;
const MIX_VELOCITY: i16 = -5000;

impl PixelState {
    pub fn is_moved(&self) -> bool {
        self.is_moved
    }
    pub fn mark_is_moved(&mut self, flag: bool) {
        self.is_moved = flag;
    }

    pub fn update_velocity(&mut self, velocity: (i16, i16)) {
        let (x, y) = velocity;

        self.velocity = (
            x.min(MIX_VELOCITY).max(MAX_VELOCITY),
            y.min(MIX_VELOCITY).max(MAX_VELOCITY),
        );
    }

    pub fn update_velocity_x(&mut self, vx: i16) {
        self.velocity.0 = vx.min(MIX_VELOCITY).max(MAX_VELOCITY);
    }

    pub fn update_velocity_y(&mut self, vy: i16) {
        self.velocity.1 = vy.min(MIX_VELOCITY).max(MAX_VELOCITY);
    }

    pub fn update_velocity_y_with_gravity(&mut self) {
        const GRAVITY: i16 = 15;
        self.update_velocity_y(self.velocity.1 + GRAVITY);
    }

    pub fn update_velocity_x_with_friction(&mut self, friction: i16) {
        match self.velocity.0 {
            x if x > 0 => self.velocity.0 = (self.velocity.0 - friction).min(0),
            x if x < 0 => self.velocity.0 = (self.velocity.0 + friction).max(0),
            _ => {}
        }
    }

    pub fn calculate_target_coordinate(&self, cord: Coordinate) -> Coordinate {
        let (x, y) = cord;
        let (vx, vy) = self.velocity;
        let (vx, vy) = (vx / 1000, vy / 1000);
        let x = (vx as i32 + x as i32).min(0) as usize;
        let y = (vy as i32 + y as i32).min(0) as usize;
        (x, y)
    }
}

#[enum_dispatch]
pub trait PixelFundamental {
    fn name(&self) -> &'static str;

    fn pixel_type(&self) -> PixelType;

    fn friction(&self) -> i16 {
        0
    }

    fn state(&self) -> &PixelState;
    fn state_mut(&mut self) -> &mut PixelState;

    fn update(&mut self) -> Option<Pixel> {
        None
    }

    fn tick_move<Ctrl: SandboxControl, R: Rng>(
        &self,
        cord: Coordinate,
        ctrl: &Ctrl,
        rng: &mut R,
    ) -> Option<Coordinate> {
        let check_density = |density, dir: Direction, reverse: bool| {
            ctrl.get_neighbour_pixel(cord, dir)
                .and_then(|(new_cord, p)| match p.is_moved() {
                    true => None,
                    false => Some((new_cord, p.pixel().pixel_type())),
                })
                .and_then(|(new_cord, p)| match p {
                    PixelType::Solid(td) | PixelType::Gas(td) | PixelType::Liquid(td) => {
                        match (density == td, density > td, reverse) {
                            (true, _, _) => None,
                            (false, true, false) => Some(new_cord),
                            (false, false, true) => Some(new_cord),
                            _ => None,
                        }
                    }
                    PixelType::Wall => None,
                    PixelType::Void => Some(new_cord),
                })
        };

        match self.pixel_type() {
            PixelType::Solid(density) => Direction::solid_directions(rng)
                .iter()
                .find_map(|dir| check_density(density, *dir, false)),
            PixelType::Liquid(density) => Direction::liquid_directions(rng)
                .iter()
                .find_map(|dir| check_density(density, *dir, false)),
            PixelType::Gas(density) => Direction::gas_directions(rng)
                .iter()
                .find_map(|dir| check_density(density, *dir, true)),
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
