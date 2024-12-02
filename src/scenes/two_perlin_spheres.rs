use crate::hittable::{BvhNode, HittableKey, HittableStruct, Sphere};
use crate::material::{Lambertian, MaterialKey, MaterialStruct};
use crate::texture::{NoiseTexture, TextureKey, TextureStruct};
use crate::vec3::Point3;

pub fn scene() -> HittableStruct {
    let mut world: Vec<HittableStruct> = Vec::new();

    let mut pertext = TextureStruct::new(TextureKey::NoiseTexture);
    pertext.noise_texture = Some(NoiseTexture::new(3.0));
    let mut sphere_material = MaterialStruct::new(MaterialKey::Lambertian);
    sphere_material.lambertian = Some(Lambertian::new(&pertext));
    let mut obj = HittableStruct::new(HittableKey::Sphere);
    obj.sphere = Some(Sphere::new(
        &Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        sphere_material.clone(),
    ));
    world.push(obj);

    let mut obj = HittableStruct::new(HittableKey::Sphere);
    obj.sphere = Some(Sphere::new(
        &Point3::new(0.0, 2.0, 0.0),
        2.0,
        sphere_material.clone(),
    ));
    world.push(obj);

    let mut bvh = HittableStruct::new(HittableKey::BvhNode);
    bvh.bvh_node = Some(Box::new(BvhNode::new(&mut world, 0.0, 0.0)));
    bvh
}
