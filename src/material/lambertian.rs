use super::Material;
use crate::texture::TextureStruct;
use crate::{hittable, ray, vec3};

#[derive(Debug, Clone)]
pub struct Lambertian {
    albedo: TextureStruct,
}

impl Lambertian {
    pub fn new(albedo: &TextureStruct) -> Self {
        Self {
            albedo: albedo.clone(),
        }
    }
}

impl Material for Lambertian {
    fn scatter(
        &self,
        r_in: &ray::Ray,
        rec: &hittable::HitRecord,
        attenuation: &mut vec3::Color,
        scattered: &mut ray::Ray,
    ) -> bool {
        let scatter_direction = rec.normal + vec3::Vec3::rand_unit_vector();
        *scattered = ray::Ray::new(&rec.p, &scatter_direction, r_in.time);
        *attenuation = self.albedo.value(rec.u, rec.v, &rec.p);
        true
    }
}
