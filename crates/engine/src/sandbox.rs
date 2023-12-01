use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};

use crate::pixel::{Direction, Pixel, PixelContainer, PixelFundamental, PixelInteract, PixelType};

#[derive(Debug)]
pub struct Sandbox {
    pub width: usize,
    pub height: usize,
    pub pixels: Vec<Vec<PixelContainer>>,
}

impl Sandbox {
    pub fn new(width: usize, height: usize) -> Sandbox {
        Self {
            width,
            height,
            pixels: vec![vec![PixelContainer::default(); height]; width],
        }
    }

    pub fn is_coordinate_in_bound(&self, x: usize, y: usize) -> bool {
        // don't check for negative values here, assume it's not possible to have canvas this big
        x < self.width && y < self.height
    }

    pub fn get_neighbour_coordinates(
        &self,
        x: usize,
        y: usize,
        dir: Direction,
    ) -> Option<(usize, usize)> {
        let is_not_on_top = || y > 0;
        let is_not_on_bottom = || y < self.height - 1;
        let is_not_on_left = || x > 0;
        let is_not_on_right = || x < self.width - 1;

        match dir {
            Direction::Up if is_not_on_top() => Some((x, y - 1)),
            Direction::Down if is_not_on_bottom() => Some((x, y + 1)),
            Direction::Left if is_not_on_left() => Some((x - 1, y)),
            Direction::Right if is_not_on_right() => Some((x + 1, y)),
            Direction::UpLeft if is_not_on_top() && is_not_on_left() => Some((x - 1, y - 1)),
            Direction::UpRight if is_not_on_top() && is_not_on_right() => Some((x + 1, y - 1)),
            Direction::DownLeft if is_not_on_bottom() && is_not_on_left() => Some((x - 1, y + 1)),
            Direction::DownRight if is_not_on_bottom() && is_not_on_right() => Some((x + 1, y + 1)),
            _ => None,
        }
    }

    pub fn get_neighbour_pixel(
        &self,
        x: usize,
        y: usize,
        dir: Direction,
    ) -> Option<(usize, usize, &PixelContainer)> {
        self.get_neighbour_coordinates(x, y, dir)
            .and_then(|(x, y)| self.get_pixel(x, y).map(|p| (x, y, p)))
    }

    pub fn get_pixel(&self, x: usize, y: usize) -> Option<&PixelContainer> {
        self.pixels.get(x).and_then(|p| p.get(y))
    }
    pub fn get_pixel_mut(&mut self, x: usize, y: usize) -> Option<&mut PixelContainer> {
        self.pixels.get_mut(x).and_then(|p| p.get_mut(y))
    }

    pub fn place_pixel<P: Into<Pixel>>(&mut self, pixel: P, x: usize, y: usize) {
        if let Some(p) = self.get_pixel_mut(x, y) {
            if p.pixel().pixel_type() != PixelType::Void {
                return;
            }
            *p = PixelContainer::new(pixel.into());
        }
    }

    pub fn place_pixel_force<P: Into<Pixel>>(&mut self, pixel: P, x: usize, y: usize) {
        if let Some(p) = self.get_pixel_mut(x, y) {
            *p = PixelContainer::new(pixel.into());
        }
    }

    pub fn tick(&mut self) {
        for idx in (0..self.pixels.len() - 1).rev() {
            let pixel = self.pixels.get(idx).unwrap();
            if pixel.pixel().pixel_type() == PixelType::Void {
                continue;
            }

            if pixel.is_moved {
                continue;
            }

            let (x, y) = self.index_to_coordinates(idx);

            if let Some((new_x, new_y)) = pixel.pixel().tick_move(x, y, self) {
                let new_index = self.coordinates_to_index(new_x, new_y);

                let pixel = self.pixels.get_mut(idx).unwrap();
                pixel.mark_is_moved(true);
                let swapping_pixel = self.pixels.get_mut(new_index).unwrap();
                if swapping_pixel.pixel().pixel_type() != PixelType::Void {
                    swapping_pixel.mark_is_moved(true);
                }

                self.pixels.swap(idx, new_index);
            }
        }

        for idx in (0..self.pixels.len() - 1).rev() {
            let (x, y) = self.index_to_coordinates(idx);

            let neighbour = [
                self.get_neighbour_pixel(x, y, Direction::Up)
                    .map(|(_, _, c)| c.pixel()),
                self.get_neighbour_pixel(x, y, Direction::Down)
                    .map(|(_, _, c)| c.pixel()),
                self.get_neighbour_pixel(x, y, Direction::Left)
                    .map(|(_, _, c)| c.pixel()),
                self.get_neighbour_pixel(x, y, Direction::Right)
                    .map(|(_, _, c)| c.pixel()),
            ];

            let pixel = self.pixels.get_mut(idx).unwrap();
            neighbour.into_iter().for_each(|t| {
                if let Some(target) = t {
                    pixel.pixel_mut().interact(target);
                }
            });

            if let Some(new_pixel) = PixelFundamental::update(pixel.pixel_mut()) {
                pixel.pixel = new_pixel;
            }
        }

        self.pixels.iter_mut().for_each(|p| p.mark_is_moved(false));
    }

    pub fn resize(&mut self, width: usize, height: usize) {
        let width_delta = width as isize - self.width as isize;
        let height_delta = height as isize - self.height as isize;

        let mut new_sandbox = Sandbox::<SmallRng>::new(width, height);
        self.pixels.iter().enumerate().for_each(|(idx, p)| {
            let (x, y) = self.index_to_coordinates(idx);
            let new_x = x as isize + width_delta / 2;
            let new_y = y as isize + height_delta / 2;
            if new_sandbox.is_coordinate_in_bound(new_x as usize, new_y as usize) {
                new_sandbox.place_pixel_force(p.pixel, new_x as usize, new_y as usize);
            }
        });

        self.width = new_sandbox.width;
        self.height = new_sandbox.height;
        self.pixels = new_sandbox.pixels;
    }
}

#[cfg(test)]
mod test {
    use rand::rngs::mock::StepRng;

    use crate::pixel::sand::Sand;
    use crate::pixel::water::Water;
    use crate::sandbox::Sandbox;

    fn new_rng() -> StepRng {
        StepRng::new(42, 1)
    }

    #[test]
    fn test_sandbox_creation() {
        // create a sandbox
        let sandbox = Sandbox::new_with_rng(3, 3, new_rng());
        assert_eq!(sandbox.pixels.len(), 9);
    }
    #[test]
    fn test_sandbox_tick() {
        // create a sandbox
        let mut sandbox = Sandbox::new_with_rng(3, 3, new_rng());
        sandbox.place_pixel_force(Sand, 1, 0);
        sandbox.tick();
        let new_cord = sandbox.coordinates_to_index(1, 1);
        assert_eq!(sandbox.pixels[new_cord].pixel, Sand.into());
        sandbox.tick();
        let new_cord = sandbox.coordinates_to_index(1, 2);
        assert_eq!(sandbox.pixels[new_cord].pixel, Sand.into());
    }
    #[test]
    fn test_sandbox_tick2() {
        // create a sandbox
        let mut sandbox = Sandbox::new_with_rng(3, 3, new_rng());
        sandbox.place_pixel_force(Sand, 1, 0);
        sandbox.place_pixel_force(Water::default(), 1, 1);
        sandbox.tick();
        let sand_new_cord = sandbox.coordinates_to_index(1, 1);
        let water_new_cord = sandbox.coordinates_to_index(1, 2);
        assert_eq!(
            sandbox.pixels[sand_new_cord].pixel,
            Sand.into(),
            "{:?}",
            &sandbox.pixels
        );
        assert_eq!(
            sandbox.pixels[water_new_cord].pixel,
            Water::default().into(),
            "{:?}",
            &sandbox.pixels
        );
        sandbox.tick();
        let sand_new_cord = sandbox.coordinates_to_index(1, 2);
        let water_new_cord = sandbox.coordinates_to_index(0, 2);
        assert_eq!(
            sandbox.pixels[sand_new_cord].pixel,
            Sand.into(),
            "{:?}",
            &sandbox.pixels
        );
        assert_eq!(
            sandbox.pixels[water_new_cord].pixel,
            Water::default().into(),
            "{:?}",
            &sandbox.pixels
        );
    }
}
