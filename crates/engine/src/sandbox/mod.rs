use crate::pixel::{Direction, PixelContainer};

pub mod sandbox;
mod virtualbox;

pub type Coordinate = (usize, usize);

pub trait SandboxControl {
    fn matrix(&self) -> &[Vec<PixelContainer>];
    fn matrix_mut(&mut self) -> &mut [Vec<PixelContainer>];
    fn width(&self) -> usize;
    fn height(&self) -> usize;
    fn get_pixel(&self, cord: Coordinate) -> Option<&PixelContainer> {
        let (x, y) = cord;
        self.matrix().get(x).and_then(|p| p.get(y))
    }
    fn get_pixel_mut(&mut self, cord: Coordinate) -> Option<&mut PixelContainer> {
        let (x, y) = cord;
        self.matrix_mut().get_mut(x).and_then(|p| p.get_mut(y))
    }

    fn is_coordinate_in_bound(&self, cord: Coordinate) -> bool {
        let (x, y) = cord;
        // don't check for negative values here, assume it's not possible to have canvas this big
        x < self.width() && y < self.height()
    }

    fn get_neighbour_coordinates(&self, cord: Coordinate, dir: Direction) -> Option<Coordinate> {
        let (x, y) = cord;

        let is_not_on_top = || y > 0;
        let is_not_on_bottom = || y < self.height() - 1;
        let is_not_on_left = || x > 0;
        let is_not_on_right = || x < self.width() - 1;

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

    fn get_neighbour_pixel(
        &self,
        cord: Coordinate,
        dir: Direction,
    ) -> Option<(Coordinate, &PixelContainer)> {
        self.get_neighbour_coordinates(cord, dir)
            .and_then(|new_cord| self.get_pixel(new_cord).map(|p| (new_cord, p)))
    }

    fn swap_pixels(&mut self, cord1: Coordinate, cord2: Coordinate) {
        let (x1, y1) = cord1;
        let (x2, y2) = cord2;

        let p1 = self.get_pixel(cord1);
        let p2 = self.get_pixel(cord2);

        if let (Some(p1), Some(p2)) = (p1, p2) {
            let p1 = *p1;
            let p2 = *p2;

            self.matrix_mut()[x1][y1] = p2;
            self.matrix_mut()[x2][y2] = p1;
        }
    }
}
