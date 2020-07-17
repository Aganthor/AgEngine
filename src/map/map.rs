use bmp::Image;
use ggez::graphics;
use ggez::graphics::{BlendMode, Color, DrawMode, DrawParam, Drawable, Rect};
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
            texture_loader: TextureLoader::new()
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
    map_size: usize,
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
                if map_value >= -1.0 && map_value < -0.25 {
                    self.level_data.push(TileInfo::new(tile_x_pos, tile_y_pos, TileType::DeepWater, false));
                } else if map_value >= -0.25 && map_value < 0.0 {
                    self.level_data.push(TileInfo::new(tile_x_pos, tile_y_pos, TileType::ShallowWater, false));
                } else if map_value >= 0.0 && map_value < 0.0625 {
                    self.level_data.push(TileInfo::new(tile_x_pos, tile_y_pos, TileType::Shore, true));
                } else if map_value >= 0.0625 && map_value < 0.1250 {
                    self.level_data.push(TileInfo::new(tile_x_pos, tile_y_pos, TileType::Sand, true));
                } else if map_value >= 0.1250 && map_value < 0.3750 {
                    self.level_data.push(TileInfo::new(tile_x_pos, tile_y_pos, TileType::Grass, true));
                } else if map_value >= 0.3750 && map_value < 0.75 {
                    self.level_data.push(TileInfo::new(tile_x_pos, tile_y_pos, TileType::Dirt, true));
                } else if map_value >= 0.75 && map_value < 1.0 {
                    self.level_data.push(TileInfo::new(tile_x_pos, tile_y_pos, TileType::Rock, false));
                } else {
                    self.level_data.push(TileInfo::new(tile_x_pos, tile_y_pos, TileType::Snow, true));
                }
            }
        }
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
fn inverselerp(x: f32, y: f32, value: f32) -> f32 {
    (value - x) / (y - x)
}

impl Drawable for Map {
    fn draw(&self, ctx: &mut Context, param: DrawParam) -> GameResult {
        for tileinfo in &self.level_data {
            let x = (tileinfo.x as f32 - param.offset.x) * param.scale.x;
            let y = (tileinfo.y as f32 - param.offset.y) * param.scale.x;
            let dest = Point2::new(x, y);

            graphics::draw(ctx, &self.texture_loader.textures[&tileinfo.tile_type], DrawParam::default().dest(dest)).unwrap();
        }
        Ok(())
    }

    fn dimensions(&self, ctx: &mut Context) -> Option<Rect> {
        Some(Rect::new(
            0.0,
            0.0,
            self.map_size as f32,
            self.map_size as f32,
        ))
    }

    fn set_blend_mode(&mut self, mode: Option<BlendMode>) {}

    fn blend_mode(&self) -> Option<BlendMode> {
        Some(BlendMode::Alpha)
    }
}
