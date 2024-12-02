use super::Material;
use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::texture::TextureStruct;
use crate::vec3::{Color, Vec3};

#[derive(Debug, Clone)]
pub struct Isotropic {
    albedo: TextureStruct,
}

impl Isotropic {
    pub fn new(albedo: &TextureStruct) -> Self {
        Self {
            albedo: albedo.clone(),
        }
    }
}

impl Material for Isotropic {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        *scattered = Ray::new(&rec.p, &Vec3::rand_unit_sphere(), r_in.time);
        *attenuation = self.albedo.value(rec.u, rec.v, &rec.p);
        true
    }
}
