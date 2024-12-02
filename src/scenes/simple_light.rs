use crate::hittable::{BvhNode, HittableKey, HittableStruct, Sphere, XYRect};
use crate::material::{DiffuseLight, Lambertian, MaterialKey, MaterialStruct};
use crate::texture::{NoiseTexture, SolidColor, TextureKey, TextureStruct};
use crate::vec3::{Color, Point3};

pub fn scene() -> HittableStruct {
    let mut world: Vec<HittableStruct> = Vec::new();
    let mut pertext = TextureStruct::new(TextureKey::NoiseTexture);
    pertext.noise_texture = Some(NoiseTexture::new(4.0));

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

    let mut difflight = MaterialStruct::new(MaterialKey::DiffuseLight);
    let mut texture = TextureStruct::new(TextureKey::SolidColor);
    texture.solid_color = Some(SolidColor::new(Color::new(4.0, 4.0, 4.0)));
    difflight.diffuse_light = Some(DiffuseLight::new(&texture));
    let mut obj = HittableStruct::new(HittableKey::Sphere);
    obj.sphere = Some(Sphere::new(
        &Point3::new(0.0, 7.0, 0.0),
        2.0,
        difflight.clone(),
    ));
    world.push(obj);

    let mut obj = HittableStruct::new(HittableKey::XYRect);
    obj.xy_rect = Some(XYRect::new(3.0, 5.0, 1.0, 3.0, -2.0, difflight.clone()));
    world.push(obj);

    let mut bvh = HittableStruct::new(HittableKey::BvhNode);
    bvh.bvh_node = Some(Box::new(BvhNode::new(&mut world, 0.0, 0.0)));

    bvh
}
