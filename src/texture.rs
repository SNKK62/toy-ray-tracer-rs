use crate::vec3::{Color, Vec3};
mod noise;

pub mod solid_color;
pub use solid_color::SolidColor;

pub mod checker;
pub use checker::Checker;

pub mod noise_texture;
pub use noise_texture::NoiseTexture;

pub mod image_texture;
pub use image_texture::ImageTexture;

use std::fmt::Debug;

pub trait Texture {
    fn value(&self, u: f64, v: f64, p: &Vec3) -> Color;
}

#[derive(Debug, Clone)]
pub enum TextureKey {
    SolidColor,
    Checker,
    NoiseTexture,
    ImageTexture,
}

// NOTE: This is a workaround for compiling into WASM
#[derive(Debug, Clone)]
pub struct TextureStruct {
    pub key: TextureKey,
    pub solid_color: Option<SolidColor>,
    pub checker: Option<Checker>,
    pub noise_texture: Option<NoiseTexture>,
    pub image_texture: Option<ImageTexture>,
}

impl TextureStruct {
    pub fn new(key: TextureKey) -> Self {
        Self {
            key,
            solid_color: None,
            checker: None,
            noise_texture: None,
            image_texture: None,
        }
    }

    pub fn value(&self, u: f64, v: f64, p: &Vec3) -> Color {
        match self.key {
            TextureKey::SolidColor => self.solid_color.as_ref().unwrap().value(u, v, p),
            TextureKey::Checker => self.checker.as_ref().unwrap().value(u, v, p),
            TextureKey::NoiseTexture => self.noise_texture.as_ref().unwrap().value(u, v, p),
            TextureKey::ImageTexture => self.image_texture.as_ref().unwrap().value(u, v, p),
        }
    }
}
