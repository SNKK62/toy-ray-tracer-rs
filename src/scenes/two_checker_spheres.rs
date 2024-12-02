use crate::hittable::{BvhNode, HittableKey, HittableStruct, Sphere};
use crate::material::{Lambertian, MaterialKey, MaterialStruct};
use crate::texture::{Checker, SolidColor, TextureKey, TextureStruct};
use crate::vec3::{Color, Point3};

pub fn scene() -> HittableStruct {
    let mut world: Vec<HittableStruct> = Vec::new();
    let mut checker = TextureStruct::new(TextureKey::Checker);
    let mut texture1 = TextureStruct::new(TextureKey::SolidColor);
    texture1.solid_color = Some(SolidColor::new(Color::new(0.2, 0.3, 0.1)));
    let mut texture2 = TextureStruct::new(TextureKey::SolidColor);
    texture2.solid_color = Some(SolidColor::new(Color::new(0.9, 0.9, 0.9)));
    checker.checker = Some(Checker::new(texture1, texture2));

    let mut sphere_material = MaterialStruct::new(MaterialKey::Lambertian);
    sphere_material.lambertian = Some(Lambertian::new(&checker));
    let mut obj = HittableStruct::new(HittableKey::Sphere);
    obj.sphere = Some(Sphere::new(
        &Point3::new(0.0, -10.0, 0.0),
        10.0,
        sphere_material.clone(),
    ));
    world.push(obj);

    let mut obj = HittableStruct::new(HittableKey::Sphere);
    obj.sphere = Some(Sphere::new(
        &Point3::new(0.0, 10.0, 0.0),
        10.0,
        sphere_material.clone(),
    ));
    world.push(obj);

    let mut bvh = HittableStruct::new(HittableKey::BvhNode);
    bvh.bvh_node = Some(Box::new(BvhNode::new(&mut world, 0.0, 0.0)));
    bvh
}
