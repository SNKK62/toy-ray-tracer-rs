use crate::hittable::{BvhNode, HittableKey, HittableStruct, Sphere};
use crate::material::{Lambertian, MaterialKey, MaterialStruct};
use crate::texture::{ImageTexture, TextureKey, TextureStruct};
use crate::vec3::Point3;
use std::boxed::Box;

pub fn scene() -> HittableStruct {
    let mut world: Vec<HittableStruct> = Vec::new();
    let mut texture = TextureStruct::new(TextureKey::ImageTexture);
    texture.image_texture = Some(ImageTexture::new("images/earth.png"));

    let mut sphere_material = MaterialStruct::new(MaterialKey::Lambertian);
    sphere_material.lambertian = Some(Lambertian::new(&texture));
    let mut obj = HittableStruct::new(HittableKey::Sphere);
    obj.sphere = Some(Sphere::new(
        &Point3::new(0.0, 0.0, 0.0),
        2.0,
        sphere_material,
    ));
    world.push(obj);

    let mut obj = HittableStruct::new(HittableKey::BvhNode);
    obj.bvh_node = Some(Box::new(BvhNode::new(&mut world, 0.0, 0.0)));

    obj
}
