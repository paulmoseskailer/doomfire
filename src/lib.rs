use embedded_graphics::{
    geometry::Point,
    pixelcolor::{Rgb565, Rgb888},
    Drawable, Pixel,
};
use rand::{Rng, SeedableRng};

/// RGBA color pallet
const PALLET: [[u8; 4]; 37] = [
    [0x07, 0x07, 0x07, 0xFF],
    [0x1F, 0x07, 0x07, 0xFF],
    [0x2F, 0x0F, 0x07, 0xFF],
    [0x47, 0x0F, 0x07, 0xFF],
    [0x57, 0x17, 0x07, 0xFF],
    [0x67, 0x1F, 0x07, 0xFF],
    [0x77, 0x1F, 0x07, 0xFF],
    [0x8F, 0x27, 0x07, 0xFF],
    [0x9F, 0x2F, 0x07, 0xFF],
    [0xAF, 0x3F, 0x07, 0xFF],
    [0xBF, 0x47, 0x07, 0xFF],
    [0xC7, 0x47, 0x07, 0xFF],
    [0xDF, 0x4F, 0x07, 0xFF],
    [0xDF, 0x57, 0x07, 0xFF],
    [0xDF, 0x57, 0x07, 0xFF],
    [0xD7, 0x5F, 0x07, 0xFF],
    [0xD7, 0x5F, 0x07, 0xFF],
    [0xD7, 0x67, 0x0F, 0xFF],
    [0xCF, 0x6F, 0x0F, 0xFF],
    [0xCF, 0x77, 0x0F, 0xFF],
    [0xCF, 0x7F, 0x0F, 0xFF],
    [0xCF, 0x87, 0x17, 0xFF],
    [0xC7, 0x87, 0x17, 0xFF],
    [0xC7, 0x8F, 0x17, 0xFF],
    [0xC7, 0x97, 0x1F, 0xFF],
    [0xBF, 0x9F, 0x1F, 0xFF],
    [0xBF, 0x9F, 0x1F, 0xFF],
    [0xBF, 0xA7, 0x27, 0xFF],
    [0xBF, 0xA7, 0x27, 0xFF],
    [0xBF, 0xAF, 0x2F, 0xFF],
    [0xB7, 0xAF, 0x2F, 0xFF],
    [0xB7, 0xB7, 0x2F, 0xFF],
    [0xB7, 0xB7, 0x37, 0xFF],
    [0xCF, 0xCF, 0x6F, 0xFF],
    [0xDF, 0xDF, 0x9F, 0xFF],
    [0xEF, 0xEF, 0xC7, 0xFF],
    [0xFF, 0xFF, 0xFF, 0xFF],
];

pub struct DoomFire {
    fire_pixels: Vec<usize>,
    width: usize,
    height: usize,
}

impl DoomFire {
    pub fn new(width: usize, height: usize) -> Self {
        // set the whole screen to color 0
        let mut fire_pixels = vec![0; width * height];
        // set the bottom line to color 36
        for i in 0..width {
            fire_pixels[(height - 1) * width + i] = 36;
        }

        DoomFire {
            fire_pixels,
            width,
            height,
        }
    }

    /// Update the internal state of each pixel for the Doom Fire effect.
    /// This is main code for the Doom Fire effect.
    pub fn update(&mut self) {
        // No particular reason to use ChaCha8Rng. Was just part of
        // an example on the [rust rand
        // cookbook](https://rust-random.github.io/book/guide-start.html#fixed-seed-rngs)
        let mut rng = rand_chacha::ChaCha8Rng::seed_from_u64(666);

        for x in 0..self.width {
            for y in 1..self.height {
                let src = y * self.width + x;
                let pixel = self.fire_pixels[src];

                if pixel == 0 {
                    self.fire_pixels[src - self.width] = 0;
                } else {
                    let rand_idx = (rng.gen_range(0.0, 3.0) + 0.5) as usize & 3;
                    let dst = src - rand_idx + 1;
                    self.fire_pixels[dst - self.width] = pixel - (rand_idx & 1);
                }
            }
        }
    }

    /// Draw the next frame to a generic byte slice.
    /// frame will usually be some reference to a pixel buffer provided by some rendering library.
    /// This function will fill the frame with RGBA pixels
    pub fn draw_to_byte_slice(&self, frame: &mut [u8]) {
        for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
            pixel.copy_from_slice(&PALLET[self.fire_pixels[i]])
        }
    }
}

impl Default for DoomFire {
    fn default() -> Self {
        DoomFire::new(320, 168)
    }
}

impl Drawable for DoomFire {
    type Color = Rgb565;

    type Output = ();

    fn draw<D>(&self, target: &mut D) -> Result<Self::Output, D::Error>
    where
        D: embedded_graphics::prelude::DrawTarget<Color = Self::Color>,
    {
        let colors = self.fire_pixels.iter().map(|i| {
            let colors = &PALLET[*i][0..3];
            Rgb565::from(Rgb888::new(colors[0], colors[1], colors[2]))
        });
        let points = (0..self.height)
            .flat_map(|y| (0..self.width).map(move |x| Point::new(x as i32, y as i32)));
        let pixels = points.zip(colors).map(|(p, c)| Pixel(p, c));
        target.draw_iter(pixels)
    }
}
