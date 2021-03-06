use bmp::Image;
use ggez::graphics;
use ggez::graphics::{BlendMode, Color, DrawMode, DrawParam, Drawable, Rect, FilterMode, Text};
use ggez::graphics::spritebatch::SpriteBatch;
use ggez::nalgebra::{Point2, Vector2};
use ggez::Context;
use ggez::GameResult;
use rand::prelude::*;
use simdnoise::*;
use std::f32::{MAX, MIN};

use super::tiles::*;
use super::textures::TextureLoader;

pub struct MapBuilder {
    seed: i32,
    frequency: f32,
    lacunarity: f32,
    gain: f32,
    octaves: u8,
    map_size: usize,
}

impl MapBuilder {
    pub fn new() -> MapBuilder {
        MapBuilder {
            seed: 0,
            frequency: 0.0,
            lacunarity: 0.0,
            gain: 0.0,
            octaves: 0,
            map_size: 0,
        }
    }

    pub fn with_seed(mut self, seed: i32) -> MapBuilder {
        self.seed = seed;
        self
    }

    pub fn with_frequency(mut self, freq: f32) -> MapBuilder {
        self.frequency = freq;
        self
    }

    pub fn with_lacunarity(mut self, lacunarity: f32) -> MapBuilder {
        self.lacunarity = lacunarity;
        self
    }

    pub fn with_gain(mut self, gain: f32) -> MapBuilder {
        self.gain = gain;
        self
    }

    pub fn with_octaves(mut self, octaves: u8) -> MapBuilder {
        self.octaves = octaves;
        self
    }

    pub fn with_size(mut self, size: usize) -> MapBuilder {
        self.map_size = size;
        self
    }

    pub fn build(&self) -> Map {
        let mut map_data: Vec<f32> = Vec::new();
        map_data.resize(self.map_size * self.map_size, 0.0);
        Map {
            noise_vector: Vec::new(),
            map_data: map_data,
            noise_seed: self.seed,
            noise_frequency: self.frequency,
            noise_lacunarity: self.lacunarity,
            noise_gain: self.gain,
            noise_octaves: self.octaves,
            noise_scale: 1.0,
            noise_persistance: 0.5,
            map_size: self.map_size,
            offset: Point2::new(self.map_size as f32 / 2.0, self.map_size as f32 / 2.0),
            level_data: Vec::new(),
            texture_loader: TextureLoader::new(),
        }
    }
}


pub struct Map {
    noise_vector: Vec<f32>,
    map_data: Vec<f32>,
    noise_seed: i32,
    noise_frequency: f32,
    noise_lacunarity: f32,
    noise_gain: f32,
    noise_octaves: u8,
    noise_scale: f32,
    noise_persistance: f32,
    pub map_size: usize,
    offset: Point2<f32>,
    level_data: Vec<TileInfo>,
    texture_loader: TextureLoader,
}

impl Map {
    pub fn generate_noise_map(&mut self) {
        self.noise_vector = NoiseBuilder::fbm_2d(self.map_size, self.map_size)
            .with_seed(self.noise_seed)
            .with_freq(self.noise_frequency)
            .with_lacunarity(self.noise_lacunarity)
            .with_gain(self.noise_gain)
            .with_octaves(self.noise_octaves)
            .generate_scaled(0.0, 1.0);
        /*            .with_freq(0.03)
        .with_lacunarity(0.55)
        .with_gain(2.5)
        .with_octaves(2)
        .generate_scaled(0.0, 1.0);*/
/*
        let mut rng = thread_rng();

        let mut octave_offsets: Vec<Point2<f32>> = Vec::new();
        octave_offsets.resize(self.noise_octaves as usize, Point2::new(0.0, 0.0));

        for i in 0..self.noise_octaves as usize {
            //rng.gen_range(-10.0, 10.0);
            let offset_x = rng.gen_range(-10000.0, 10000.0) + self.offset.x;
            let offset_y = rng.gen_range(-10000.0, 10000.0) + self.offset.y;
            octave_offsets[i].x = offset_x;
            octave_offsets[i].y = offset_y;
        }

        if self.noise_scale <= 0.0 {
            self.noise_scale = 0.0001;
        }

        let mut max_noise_height = std::f32::MAX;
        let mut min_noise_height = std::f32::MIN;
        let half_width: f32 = self.map_size as f32 / 2.0;
        let half_height: f32 = self.map_size as f32 / 2.0;

        for y in 0..self.map_size {
            for x in 0..self.map_size {
                let mut amplitude = 1.0;
                let mut frequency = 1.0;
                let mut noise_height = 0.0;

                for i in 0..self.noise_octaves as usize {
                    let sample_x = (x as f32 - half_width) / self.noise_scale * frequency + octave_offsets[i].x;
                    let sample_y = (y as f32 - half_height) / self.noise_scale * frequency + octave_offsets[i].y;

                    let noise_value = self.noise_vector[sample_x as usize * self.map_size + sample_y as usize] * 2.0 - 1.0;
                    noise_height += noise_value * amplitude;

                    amplitude *= self.noise_persistance;
                    frequency *= self.noise_lacunarity;
                }

                if noise_height > max_noise_height {
                    max_noise_height = noise_height;
                } else if noise_height < min_noise_height {
                    min_noise_height = noise_height;
                }
                self.map_data[y * self.map_size + x] = noise_height;
            }
        }

        for y in 0..self.map_size {
            for x in 0..self.map_size {
                //self.map_data[y * self.map_size + x] = inverselerp(min_noise_height, max_noise_height, self.map_data[y * self.map_size + x]);
                self.map_data[y * self.map_size + x] = self.noise_vector[y * self.map_size + x];
            }
        }
        */
        //self.map_data.extend(self.noise_vector.iter().copied());
    }

    pub fn map_max_size(&self) -> f32 {
        self.map_size as f32 * TILE_SIZE as f32
    }

    pub fn prepare_textures(&mut self, ctx: &mut Context) {
        self.texture_loader.load_textures(ctx);
    }

    pub fn generate_level(&mut self) {
        for y in 0..self.map_size {
            for x in 0..self.map_size {
                let map_value = self.noise_vector[y * self.map_size + x];
                let tile_x_pos = x * TILE_SIZE as usize;
                let tile_y_pos = y * TILE_SIZE as usize;
                let tile_type = self.biome(map_value);
                self.level_data.push(TileInfo::new(tile_x_pos, tile_y_pos, tile_type, false));
            }
        }
    }

    fn biome(&self, map_elevation: f32) -> TileType {
        if map_elevation < 0.1 {
            return TileType::DeepWater
        } else if map_elevation < 0.2 {
            return TileType::Shore
        } else if map_elevation < 0.3 {
            return TileType::Grass
        } else if map_elevation < 0.5 {
            return TileType::Forest
        } else if map_elevation < 0.8 {
            return TileType::Savannah
        } else if map_elevation < 0.9 {
            return TileType::Sand
        } else if map_elevation < 0.95 {
            return TileType::Rock
        } else {
            return TileType::Mountain
        }
    }

    pub fn get_tileinfo_at(&self, x: usize, y: usize) -> TileInfo {
        self.level_data[y * self.map_size + x as usize]
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

#[inline]
#[allow(dead_code)]
fn inverselerp(x: f32, y: f32, value: f32) -> f32 {
    (value - x) / (y - x)
}

impl Drawable for Map {
    fn draw(&self, ctx: &mut Context, param: DrawParam) -> GameResult {
        graphics::set_default_filter(ctx, FilterMode::Nearest);

        //Get the window size... so we can take the right number of tiles.
        let window = graphics::window(ctx);
        let window_size = window.get_inner_size().unwrap();
        let tiles_skip_x: usize = param.offset.x as usize / 32;
        let tiles_skip_y: usize = param.offset.y as usize / 32;
        let mut max_tiles_x = (window_size.width / TILE_SIZE as f64) as usize + tiles_skip_x;
        let mut max_tiles_y = (window_size.height / TILE_SIZE as f64) as usize + tiles_skip_y;

        if max_tiles_x >= self.map_size as usize {
            max_tiles_x = self.map_size as usize - 1;
        }

        if max_tiles_y >= self.map_size as usize {
            max_tiles_y = self.map_size as usize - 1;
        }

        let mut nb_tiles_drawn = 0;

        for y in tiles_skip_y..max_tiles_y as usize {
            for x in tiles_skip_x..max_tiles_x as usize {
                let tileinfo = self.get_tileinfo_at(x, y);
                let tile_x = (tileinfo.x as f32 - param.offset.x) * param.scale.x;
                let tile_y = (tileinfo.y as f32 - param.offset.y) * param.scale.x;
                let dest = Point2::new(tile_x, tile_y);

                graphics::draw(ctx,
                    &self.texture_loader.textures[&tileinfo.tile_type],
                    DrawParam::default().dest(dest)).unwrap();

                nb_tiles_drawn += 1;
            }
        }

        let message = format!("Offset is {},{} and tiles drawn = {}", param.offset.x, param.offset.y, nb_tiles_drawn);
        graphics::window(ctx).set_title(&message);

        Ok(())
    }

    fn dimensions(&self, _ctx: &mut Context) -> Option<Rect> {
        Some(Rect::new(
            0.0,
            0.0,
            self.map_size as f32,
            self.map_size as f32,
        ))
    }

    fn set_blend_mode(&mut self, _mode: Option<BlendMode>) {}

    fn blend_mode(&self) -> Option<BlendMode> {
        Some(BlendMode::Alpha)
    }
}
