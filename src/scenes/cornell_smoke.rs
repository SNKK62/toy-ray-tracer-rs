use crate::hittable::{
    BvhNode, ConstantMedium, Cuboid, HittableKey, HittableStruct, RotateY, Translation, XYRect,
    XZRect, YZRect,
};
use crate::material::{DiffuseLight, Lambertian, MaterialKey, MaterialStruct};
use crate::texture::{SolidColor, TextureKey, TextureStruct};
use crate::vec3::Color;

pub fn scene() -> HittableStruct {
    let mut world: Vec<HittableStruct> = Vec::new();

    let mut red = MaterialStruct::new(MaterialKey::Lambertian);
    let mut texture = TextureStruct::new(TextureKey::SolidColor);
    texture.solid_color = Some(SolidColor::new(Color::new(0.65, 0.05, 0.05)));
    red.lambertian = Some(Lambertian::new(&texture));

    let mut white = MaterialStruct::new(MaterialKey::Lambertian);
    let mut texture = TextureStruct::new(TextureKey::SolidColor);
    texture.solid_color = Some(SolidColor::new(Color::new(0.73, 0.73, 0.73)));
    white.lambertian = Some(Lambertian::new(&texture));

    let mut green = MaterialStruct::new(MaterialKey::Lambertian);
    let mut texture = TextureStruct::new(TextureKey::SolidColor);
    texture.solid_color = Some(SolidColor::new(Color::new(0.12, 0.45, 0.15)));
    green.lambertian = Some(Lambertian::new(&texture));

    let mut light = MaterialStruct::new(MaterialKey::DiffuseLight);
    let mut texture = TextureStruct::new(TextureKey::SolidColor);
    texture.solid_color = Some(SolidColor::new(Color::new(15.0, 15.0, 15.0)));
    light.diffuse_light = Some(DiffuseLight::new(&texture));

    let mut obj = HittableStruct::new(HittableKey::YZRect);
    obj.yz_rect = Some(YZRect::new(0.0, 555.0, 0.0, 555.0, 555.0, green.clone()));
    world.push(obj);

    let mut obj = HittableStruct::new(HittableKey::YZRect);
    obj.yz_rect = Some(YZRect::new(0.0, 555.0, 0.0, 555.0, 0.0, red.clone()));
    world.push(obj);

    let mut obj = HittableStruct::new(HittableKey::XZRect);
    obj.xz_rect = Some(XZRect::new(
        213.0,
        343.0,
        227.0,
        332.0,
        554.0,
        light.clone(),
    ));
    world.push(obj);

    let mut obj = HittableStruct::new(HittableKey::XZRect);
    obj.xz_rect = Some(XZRect::new(0.0, 555.0, 0.0, 555.0, 555.0, white.clone()));
    world.push(obj);

    let mut obj = HittableStruct::new(HittableKey::XZRect);
    obj.xz_rect = Some(XZRect::new(0.0, 555.0, 0.0, 555.0, 0.0, white.clone()));
    world.push(obj);

    let mut obj = HittableStruct::new(HittableKey::XYRect);
    obj.xy_rect = Some(XYRect::new(0.0, 555.0, 0.0, 555.0, 555.0, white.clone()));
    world.push(obj);

    // Box
    let mut obj = HittableStruct::new(HittableKey::Cuboid);
    obj.cuboid = Some(Cuboid::new(
        &crate::vec3::Point3::new(0.0, 0.0, 0.0),
        &crate::vec3::Point3::new(165.0, 330.0, 165.0),
        white.clone(),
    ));
    let mut new_obj = HittableStruct::new(HittableKey::RotateY);
    new_obj.rotate_y = Some(Box::new(RotateY::new(obj, 15.0)));
    obj = new_obj;

    let mut new_obj = HittableStruct::new(HittableKey::Translation);
    new_obj.translation = Some(Translation::new(
        obj,
        crate::vec3::Vec3::new(265.0, 0.0, 295.0),
    ));
    obj = new_obj;

    let mut new_obj = HittableStruct::new(HittableKey::ConstantMedium);
    let mut texture = TextureStruct::new(TextureKey::SolidColor);
    texture.solid_color = Some(SolidColor::new(Color::new(1.0, 1.0, 1.0)));
    new_obj.constant_medium = Some(Box::new(ConstantMedium::new(obj, 0.01, texture)));
    obj = new_obj;
    world.push(obj);

    let mut obj = HittableStruct::new(HittableKey::Cuboid);
    obj.cuboid = Some(Cuboid::new(
        &crate::vec3::Point3::new(0.0, 0.0, 0.0),
        &crate::vec3::Point3::new(165.0, 165.0, 165.0),
        white.clone(),
    ));
    let mut new_obj = HittableStruct::new(HittableKey::RotateY);
    new_obj.rotate_y = Some(Box::new(RotateY::new(obj, -18.0)));
    obj = new_obj;

    let mut new_obj = HittableStruct::new(HittableKey::Translation);
    new_obj.translation = Some(Translation::new(
        obj,
        crate::vec3::Vec3::new(130.0, 0.0, 65.0),
    ));
    obj = new_obj;

    let mut new_obj = HittableStruct::new(HittableKey::ConstantMedium);
    let mut texture = TextureStruct::new(TextureKey::SolidColor);
    texture.solid_color = Some(SolidColor::new(Color::new(0.0, 0.0, 0.0)));
    new_obj.constant_medium = Some(Box::new(ConstantMedium::new(obj, 0.01, texture)));
    obj = new_obj;
    world.push(obj);

    let mut bvh = HittableStruct::new(HittableKey::BvhNode);
    bvh.bvh_node = Some(Box::new(BvhNode::new(&mut world, 0.0, 0.0)));

    bvh
}
