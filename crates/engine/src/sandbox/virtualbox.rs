use itertools::Either;
use rand::thread_rng;

use crate::pixel::{Pixel, PixelFundamental, PixelType};
use crate::sandbox::SandboxControl;

pub(crate) struct VirtualBox<'a> {
    pub offset: usize,
    pub virtual_width: usize,
    pub height: usize,
    pub pixels: &'a mut [Vec<Pixel>],
    ltr: bool,
}

impl SandboxControl for VirtualBox<'_> {
    fn matrix(&self) -> &[Vec<Pixel>] {
        self.pixels
    }

    fn matrix_mut(&mut self) -> &mut [Vec<Pixel>] {
        self.pixels
    }

    fn width(&self) -> usize {
        self.pixels.len()
    }

    fn height(&self) -> usize {
        self.height
    }
}

impl<'a> VirtualBox<'a> {
    pub(crate) fn new(
        offset: usize,
        virtual_width: usize,
        height: usize,
        pixels: &'a mut [Vec<Pixel>],
        ltr: bool,
    ) -> Self {
        Self {
            offset,
            virtual_width: virtual_width.min(pixels.len() - offset),
            height,
            pixels,
            ltr,
        }
    }

    pub fn tick(&mut self) {
        let mut rng = thread_rng();

        let x_range = match self.ltr {
            true => Either::Left(self.offset..self.virtual_width + self.offset),
            false => Either::Right((self.offset..self.virtual_width + self.offset).rev()),
        };
        for y in (0..self.height).rev() {
            for x in x_range.clone() {
                let cord = (x, y);

                let pixel = self.get_pixel(cord);
                let Some(pixel) = pixel else {
                    println!("pixel not found at x: {}, y :{}, width: {}, virtual width: {}, height: {}, offset: {}", cord.0, cord.1, self.width(), self.virtual_width, self.height(), self.offset);
                    continue;
                };

                if pixel.pixel().pixel_type() == PixelType::Void {
                    continue;
                }

                if pixel.is_moved {
                    continue;
                }

                if let Some(new_cord) = pixel.pixel().tick_move(cord, self, &mut rng) {
                    let pixel = self.get_pixel_mut(cord).unwrap();
                    pixel.mark_is_moved(true);

                    let swapping_pixel = self.get_pixel_mut(new_cord).unwrap();
                    if swapping_pixel.pixel().pixel_type() != PixelType::Void {
                        swapping_pixel.mark_is_moved(true);
                    }

                    self.swap_pixels(cord, new_cord);
                }
            }
        }
    }
}
