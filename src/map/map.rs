use bmp::Image;
use simdnoise::*;

const SIZE: usize = 500;

pub struct Map {
    noise_vector: Vec<f32>,
}

impl Map {
    pub fn new() -> Map {
        Map {
            noise_vector: Vec::new()
        }
    }

    pub fn generate_map(self) -> Map {
        let noise_vector = NoiseBuilder::fbm_2d(SIZE, SIZE)
            .with_seed(1337322)
            .with_freq(0.03)
            .with_lacunarity(0.55)
            .with_gain(2.5)
            .with_octaves(2)
            .generate_scaled(0.0, 1.0);

        Map { noise_vector }
    }

    #[allow(dead_code)]
    pub fn save_image(self) {
        let mut img = Image::new(SIZE as u32, SIZE as u32);

        for x in 0..SIZE - 1 {
            for y in 0..SIZE - 1 {
                let height = self.noise_vector[x * SIZE + y];
                let color = 256.0 * height;
                img.set_pixel(
                    x as u32,
                    y as u32,
                    bmp::Pixel::new(color as u8, color as u8, color as u8),
                );
            }
        }
        let _ = img.save("map.bmp");
    }
}
