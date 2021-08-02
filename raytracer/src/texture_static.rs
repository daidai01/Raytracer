use crate::clamp;
use crate::perlin::Perlin;
pub use crate::vec3::Vec3;
use image::DynamicImage;
use imageproc::drawing::Canvas;
use std::path::Path;

pub trait Texture {
    fn value(&self, u: f64, v: f64, p: &Vec3) -> Vec3;
}

#[derive(Clone)]
pub struct SolidColor {
    color_value: Vec3,
}

impl SolidColor {
    pub fn new_with_vec(c: Vec3) -> Self {
        Self { color_value: c }
    }

    pub fn new_with_col(red: f64, green: f64, blue: f64) -> Self {
        Self {
            color_value: Vec3::new(red, green, blue),
        }
    }
}

impl Texture for SolidColor {
    fn value(&self, u: f64, v: f64, p: &Vec3) -> Vec3 {
        self.color_value
    }
}

#[derive(Clone)]
pub struct CheckerTexture<O: Texture, E: Texture> {
    pub odd: O,
    pub even: E,
}

impl<O: Texture, E: Texture> CheckerTexture<O, E> {
    pub fn new(_even: E, _odd: O) -> Self {
        Self {
            even: _even,
            odd: _odd,
        }
    }
}

impl<O: Texture, E: Texture> Texture for CheckerTexture<O, E> {
    fn value(&self, u: f64, v: f64, p: &Vec3) -> Vec3 {
        let sines = (10.0 * p.x).sin() * (10.0 * p.y).sin() * (10.0 * p.z).sin();
        if sines < 0.0 {
            self.odd.value(u, v, p)
        } else {
            self.even.value(u, v, p)
        }
    }
}

#[derive(Clone)]
pub struct NoiseTexture {
    noise: Perlin,
    scale: f64,
}

impl NoiseTexture {
    pub fn new(sc: f64) -> Self {
        Self {
            noise: Perlin::new(),
            scale: sc,
        }
    }
}

impl Texture for NoiseTexture {
    fn value(&self, u: f64, v: f64, p: &Vec3) -> Vec3 {
        Vec3::ones() * 0.5 * (1.0 + (self.scale * p.z + 10.0 * self.noise.turb(p, 7)).sin())
    }
}

#[derive(Clone)]
pub struct ImageTexture {
    image: DynamicImage,
    width: u32,
    height: u32,
}

impl ImageTexture {
    pub fn new(filename: &str) -> Self {
        let img = image::open(Path::new(filename)).unwrap();
        Self {
            image: img.clone(),
            width: img.dimensions().0,
            height: img.dimensions().1,
        }
    }
}

impl Texture for ImageTexture {
    fn value(&self, u: f64, v: f64, p: &Vec3) -> Vec3 {
        let _u = clamp(u, 0.0, 1.0);
        let _v = 1.0 - clamp(v, 0.0, 1.0);
        let mut i = (_u * self.width as f64) as u32;
        let mut j = (_v * self.height as f64) as u32;
        if i >= self.width {
            i = self.width - 1;
        }
        if j >= self.height {
            j = self.height - 1;
        }
        let color_scale = 1.0 / 255.0;
        let pixel = self.image.get_pixel(i, j);
        Vec3::new(
            color_scale * pixel[0] as f64,
            color_scale * pixel[1] as f64,
            color_scale * pixel[2] as f64,
        )
    }
}
