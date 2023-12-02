use crate::pixel::{Direction, Pixel, PixelContainer, PixelFundamental, PixelInteract, PixelType};
use crate::sandbox::virtualbox::VirtualBox;
use crate::sandbox::{Coordinate, SandboxControl};
use rand::{thread_rng, Rng};
use rayon::prelude::*;

/// This needs to be at least the furthest possible distance a pixel can travel in one tick times 2
const VIRTUAL_COLUMNIZED_WIDTH: usize = 5;

#[derive(Debug)]
pub struct Sandbox {
    pub width: usize,
    pub height: usize,
    pub pixels: Vec<Vec<PixelContainer>>,
    ltr: bool,
}

impl SandboxControl for Sandbox {
    fn matrix(&self) -> &[Vec<PixelContainer>] {
        &self.pixels
    }

    fn matrix_mut(&mut self) -> &mut [Vec<PixelContainer>] {
        &mut self.pixels
    }

    fn width(&self) -> usize {
        self.width
    }

    fn height(&self) -> usize {
        self.height
    }
}

impl Sandbox {
    pub fn new(width: usize, height: usize) -> Sandbox {
        Self {
            width,
            height,
            pixels: vec![vec![PixelContainer::default(); height]; width],
            ltr: true,
        }
    }

    pub fn place_pixel<P: Into<Pixel>>(&mut self, pixel: P, cord: Coordinate) {
        if let Some(p) = self.get_pixel_mut(cord) {
            if p.pixel().pixel_type() != PixelType::Void {
                return;
            }
            *p = PixelContainer::new(pixel.into());
        }
    }

    pub fn place_pixel_force<P: Into<Pixel>>(&mut self, pixel: P, cord: Coordinate) {
        if let Some(p) = self.get_pixel_mut(cord) {
            *p = PixelContainer::new(pixel.into());
        }
    }

    fn exec_pixels_movement(&mut self) {
        if self.width <= VIRTUAL_COLUMNIZED_WIDTH * 4 {
            VirtualBox::new(0, self.width, self.height, &mut self.pixels, self.ltr).tick();
            return;
        }

        // split the vec into something like this
        // |---offset---|virtual|virtual|virtual|virtual|virtual|virtual|
        // |-----A------|---B---|---C---|---D---|---E---|---F---|---G---|
        // phase one (A)+B, C+(D+E)+F
        // phase two A+(B+C)+D, E+(F+G)

        let mut rng = thread_rng();
        let offset = rng.gen_range(1..=VIRTUAL_COLUMNIZED_WIDTH * 2);
        // phase one
        let (first, rest) = self.pixels.split_at_mut(offset + VIRTUAL_COLUMNIZED_WIDTH);
        let rest_splits = rest
            .par_chunks_mut(VIRTUAL_COLUMNIZED_WIDTH * 4)
            .filter_map(|each| match each.len() > VIRTUAL_COLUMNIZED_WIDTH {
                true => Some((VIRTUAL_COLUMNIZED_WIDTH, 2 * VIRTUAL_COLUMNIZED_WIDTH, each)),
                false => None,
            });

        [(0usize, offset, first)]
            .into_par_iter()
            .chain(rest_splits)
            .for_each(|(offset, virtual_width, pixels)| {
                VirtualBox::new(offset, virtual_width, self.height, pixels, self.ltr).tick();
            });

        // phase two
        let (first, rest) = self
            .pixels
            .split_at_mut(offset + VIRTUAL_COLUMNIZED_WIDTH * 3);
        let rest_splits = rest
            .par_chunks_mut(VIRTUAL_COLUMNIZED_WIDTH * 4)
            .filter_map(|each| match each.len() > VIRTUAL_COLUMNIZED_WIDTH {
                true => Some((VIRTUAL_COLUMNIZED_WIDTH, each)),
                false => None,
            });

        [(offset, first)]
            .into_par_iter()
            .chain(rest_splits)
            .for_each(|(offset, pixels)| {
                VirtualBox::new(
                    offset,
                    VIRTUAL_COLUMNIZED_WIDTH * 2,
                    self.height,
                    pixels,
                    self.ltr,
                )
                .tick();
            });

        self.pixels.par_iter_mut().for_each(|each| {
            each.par_iter_mut().for_each(|each| {
                each.mark_is_moved(false);
            });
        });
    }

    fn exec_pixels_interaction(&mut self) {
        let mut buffer = self.pixels.clone();

        buffer.par_iter_mut().enumerate().for_each(|(x, y_axle)| {
            y_axle.par_iter_mut().enumerate().for_each(|(y, pixel)| {
                let cord = (x, y);
                let neighbour = [
                    self.get_neighbour_pixel(cord, Direction::Up)
                        .map(|(_, c)| c.pixel()),
                    self.get_neighbour_pixel(cord, Direction::Down)
                        .map(|(_, c)| c.pixel()),
                    self.get_neighbour_pixel(cord, Direction::Left)
                        .map(|(_, c)| c.pixel()),
                    self.get_neighbour_pixel(cord, Direction::Right)
                        .map(|(_, c)| c.pixel()),
                ];

                neighbour.into_iter().for_each(|t| {
                    if let Some(target) = t {
                        pixel.pixel_mut().interact(target);
                    }
                });

                if let Some(new_pixel) = PixelFundamental::update(pixel.pixel_mut()) {
                    *pixel = PixelContainer::new(new_pixel);
                }
            });
        });

        self.pixels = buffer;
    }

    pub fn tick(&mut self) {
        self.exec_pixels_interaction();
        self.exec_pixels_movement();
        self.ltr = !self.ltr;
    }

    pub fn resize(&mut self, width: usize, height: usize) {
        // let width_delta = width as isize - self.width as isize;
        // let height_delta = height as isize - self.height as isize;
        //
        // let mut new_sandbox = Sandbox::<SmallRng>::new(width, height);
        // self.pixels.iter().enumerate().for_each(|(idx, p)| {
        //     let (x, y) = self.index_to_coordinates(idx);
        //     let new_x = x as isize + width_delta / 2;
        //     let new_y = y as isize + height_delta / 2;
        //     if new_sandbox.is_coordinate_in_bound(new_x as usize, new_y as usize) {
        //         new_sandbox.place_pixel_force(p.pixel, new_x as usize, new_y as usize);
        //     }
        // });
        //
        // self.width = new_sandbox.width;
        // self.height = new_sandbox.height;
        // self.pixels = new_sandbox.pixels;
    }
}
//
// #[cfg(test)]
// mod test {
//     use rand::rngs::mock::StepRng;
//
//     use crate::pixel::sand::Sand;
//     use crate::pixel::water::Water;
//     use crate::sandbox::Sandbox;
//
//     fn new_rng() -> StepRng {
//         StepRng::new(42, 1)
//     }
//
//     #[test]
//     fn test_sandbox_creation() {
//         // create a sandbox
//         let sandbox = Sandbox::new_with_rng(3, 3, new_rng());
//         assert_eq!(sandbox.pixels.len(), 9);
//     }
//     #[test]
//     fn test_sandbox_tick() {
//         // create a sandbox
//         let mut sandbox = Sandbox::new_with_rng(3, 3, new_rng());
//         sandbox.place_pixel_force(Sand, 1, 0);
//         sandbox.tick();
//         let new_cord = sandbox.coordinates_to_index(1, 1);
//         assert_eq!(sandbox.pixels[new_cord].pixel, Sand.into());
//         sandbox.tick();
//         let new_cord = sandbox.coordinates_to_index(1, 2);
//         assert_eq!(sandbox.pixels[new_cord].pixel, Sand.into());
//     }
//     #[test]
//     fn test_sandbox_tick2() {
//         // create a sandbox
//         let mut sandbox = Sandbox::new_with_rng(3, 3, new_rng());
//         sandbox.place_pixel_force(Sand, 1, 0);
//         sandbox.place_pixel_force(Water::default(), 1, 1);
//         sandbox.tick();
//         let sand_new_cord = sandbox.coordinates_to_index(1, 1);
//         let water_new_cord = sandbox.coordinates_to_index(1, 2);
//         assert_eq!(
//             sandbox.pixels[sand_new_cord].pixel,
//             Sand.into(),
//             "{:?}",
//             &sandbox.pixels
//         );
//         assert_eq!(
//             sandbox.pixels[water_new_cord].pixel,
//             Water::default().into(),
//             "{:?}",
//             &sandbox.pixels
//         );
//         sandbox.tick();
//         let sand_new_cord = sandbox.coordinates_to_index(1, 2);
//         let water_new_cord = sandbox.coordinates_to_index(0, 2);
//         assert_eq!(
//             sandbox.pixels[sand_new_cord].pixel,
//             Sand.into(),
//             "{:?}",
//             &sandbox.pixels
//         );
//         assert_eq!(
//             sandbox.pixels[water_new_cord].pixel,
//             Water::default().into(),
//             "{:?}",
//             &sandbox.pixels
//         );
//     }
// }
