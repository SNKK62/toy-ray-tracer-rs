use crate::hittable;
use crate::ray;
use crate::vec3;

pub mod dielectric;
pub use dielectric::Dielectric;

pub mod lambertian;
pub use lambertian::Lambertian;

pub mod metal;
pub use metal::Metal;

pub mod diffuse_light;
pub use diffuse_light::DiffuseLight;

pub mod isotropic;
pub use isotropic::Isotropic;

use std::fmt::Debug;

pub trait Material {
    fn scatter(
        &self,
        r_in: &ray::Ray,
        rec: &hittable::HitRecord,
        attenuation: &mut vec3::Color,
        scattered: &mut ray::Ray,
    ) -> bool;
    fn emitted(&self, _u: f64, _v: f64, _p: &crate::vec3::Point3) -> crate::vec3::Color {
        vec3::Color::zero()
    }
}

#[derive(Debug, Clone)]
pub enum MaterialKey {
    Lambertian,
    Metal,
    Dielectric,
    DiffuseLight,
    Isotropic,
}

// NOTE: This is a workaround for compiling into WASM
#[derive(Debug, Clone)]
pub struct MaterialStruct {
    pub key: MaterialKey,
    pub lambertian: Option<Lambertian>,
    pub metal: Option<Metal>,
    pub dielectric: Option<Dielectric>,
    pub diffuse_light: Option<DiffuseLight>,
    pub isotropic: Option<Isotropic>,
}

impl MaterialStruct {
    pub fn new(key: MaterialKey) -> Self {
        Self {
            key,
            lambertian: None,
            metal: None,
            dielectric: None,
            diffuse_light: None,
            isotropic: None,
        }
    }

    pub fn scatter(
        &self,
        r_in: &ray::Ray,
        rec: &hittable::HitRecord,
        attenuation: &mut vec3::Color,
        scattered: &mut ray::Ray,
    ) -> bool {
        match self.key {
            MaterialKey::Lambertian => {
                self.lambertian
                    .as_ref()
                    .unwrap()
                    .scatter(r_in, rec, attenuation, scattered)
            }
            MaterialKey::Metal => {
                self.metal
                    .as_ref()
                    .unwrap()
                    .scatter(r_in, rec, attenuation, scattered)
            }
            MaterialKey::Dielectric => {
                self.dielectric
                    .as_ref()
                    .unwrap()
                    .scatter(r_in, rec, attenuation, scattered)
            }
            MaterialKey::DiffuseLight => {
                self.diffuse_light
                    .as_ref()
                    .unwrap()
                    .scatter(r_in, rec, attenuation, scattered)
            }
            MaterialKey::Isotropic => {
                self.isotropic
                    .as_ref()
                    .unwrap()
                    .scatter(r_in, rec, attenuation, scattered)
            }
        }
    }

    pub fn emitted(&self, u: f64, v: f64, p: &vec3::Point3) -> vec3::Color {
        match self.key {
            MaterialKey::DiffuseLight => self.diffuse_light.as_ref().unwrap().emitted(u, v, p),
            _ => vec3::Color::zero(),
        }
    }
}

fn refract(uv: &vec3::Vec3, n: &vec3::Vec3, etai_over_etat: f64) -> vec3::Vec3 {
    let cos_theta = (-*uv).dot(n).min(1.0);
    let r_out_perp = etai_over_etat * (*uv + cos_theta * *n);
    let r_out_parallel = -(1.0 - r_out_perp.power()).sqrt() * *n;
    r_out_perp + r_out_parallel
}

fn reflect(v: &vec3::Vec3, n: &vec3::Vec3) -> vec3::Vec3 {
    *v - ((2.0 * v.dot(n)) * *n)
}
