use bmp::Image;
use simdnoise::*;

pub struct Map {
    noise_vector: Vec<f32>,
    map_size: usize,
}

impl Map {
    pub fn new(size: usize) -> Map {
        Map {
            noise_vector: Vec::new(),
            map_size: size,
        }
    }

    pub fn generate_map(&mut self) {
        self.noise_vector = NoiseBuilder::fbm_2d(self.map_size, self.map_size)
            .with_seed(1337322)
            .with_freq(0.03)
            .with_lacunarity(0.55)
            .with_gain(2.5)
            .with_octaves(2)
            .generate_scaled(0.0, 1.0);
    }

    #[allow(dead_code)]
    pub fn save_image(self) {
        let mut img = Image::new(self.map_size as u32, self.map_size as u32);

        for x in 0..self.map_size - 1 {
            for y in 0..self.map_size - 1 {
                let height = self.noise_vector[x * self.map_size + y];
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
