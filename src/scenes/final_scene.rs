use crate::hittable::{
    BvhNode, ConstantMedium, Cuboid, HittableKey, HittableList, HittableStruct, MovingSphere,
    RotateY, Sphere, Translation, XZRect,
};
use crate::material::{Dielectric, DiffuseLight, Lambertian, MaterialKey, MaterialStruct, Metal};
use crate::texture::{ImageTexture, NoiseTexture, SolidColor, TextureKey, TextureStruct};
use crate::vec3::{Color, Point3, Vec3};
use rand::Rng;

use std::boxed::Box;

pub fn scene() -> HittableStruct {
    let mut box_world: Vec<HittableStruct> = Vec::new();

    let mut ground = MaterialStruct::new(MaterialKey::Lambertian);
    let mut texture = TextureStruct::new(TextureKey::SolidColor);
    texture.solid_color = Some(SolidColor::new(Color::new(0.5, 0.5, 0.5)));
    ground.lambertian = Some(Lambertian::new(&texture));
    let boxes_per_side = 20;
    for i in 0..boxes_per_side {
        for j in 0..boxes_per_side {
            let w = 100.0;
            let x0 = -1000.0 + i as f64 * w;
            let z0 = -1000.0 + j as f64 * w;
            let y0 = 0.0;
            let x1 = x0 + w;
            let y1 = rand::thread_rng().gen_range(1.0..101.0);
            let z1 = z0 + w;

            let mut box_obj = HittableStruct::new(HittableKey::Cuboid);
            box_obj.cuboid = Some(Cuboid::new(
                &Point3::new(x0, y0, z0),
                &Point3::new(x1, y1, z1),
                ground.clone(),
            ));
            box_world.push(box_obj);
        }
    }

    let mut box_bvh = HittableStruct::new(HittableKey::BvhNode);
    box_bvh.bvh_node = Some(Box::new(BvhNode::new(&mut box_world, 0.0, 1.0)));
    let mut hlist = HittableList::new();
    hlist.add(box_bvh);

    let intensity = 7.0;
    let mut light = MaterialStruct::new(MaterialKey::DiffuseLight);
    let mut texture = TextureStruct::new(TextureKey::SolidColor);
    texture.solid_color = Some(SolidColor::new(Color::new(1.0, 1.0, 1.0) * intensity));
    light.diffuse_light = Some(DiffuseLight::new(&texture));

    let mut obj = HittableStruct::new(HittableKey::XZRect);
    obj.xz_rect = Some(XZRect::new(
        123.0,
        423.0,
        147.0,
        412.0,
        554.0,
        light.clone(),
    ));
    hlist.add(obj);

    let center1 = Point3::new(400.0, 400.0, 200.0);
    let center2 = center1 + Vec3::new(30.0, 0.0, 0.0);
    let mut moving_sphere_material = MaterialStruct::new(MaterialKey::Lambertian);
    let mut texture = TextureStruct::new(TextureKey::SolidColor);
    texture.solid_color = Some(SolidColor::new(Color::new(0.7, 0.3, 0.1)));
    moving_sphere_material.lambertian = Some(Lambertian::new(&texture));
    // moving sphere
    let mut moving_sphere = HittableStruct::new(HittableKey::MovingSphere);
    moving_sphere.moving_sphere = Some(MovingSphere::new(
        &center1,
        &center2,
        50.0,
        moving_sphere_material.clone(),
        0.0,
        1.0,
    ));
    hlist.add(moving_sphere);
    // dielectric sphere
    let mut dielectric_material = MaterialStruct::new(MaterialKey::Dielectric);
    dielectric_material.dielectric = Some(Dielectric::new(1.5));
    let mut obj = HittableStruct::new(HittableKey::Sphere);
    obj.sphere = Some(Sphere::new(
        &Point3::new(260.0, 150.0, 45.0),
        50.0,
        dielectric_material,
    ));
    hlist.add(obj);
    // metal sphere
    let mut metal_material = MaterialStruct::new(MaterialKey::Metal);
    metal_material.metal = Some(Metal::new(&Color::new(0.8, 0.8, 0.9), 10.0));
    let mut obj = HittableStruct::new(HittableKey::Sphere);
    obj.sphere = Some(Sphere::new(
        &Point3::new(0.0, 150.0, 145.0),
        50.0,
        metal_material,
    ));
    hlist.add(obj);

    let mut dielectric_material = MaterialStruct::new(MaterialKey::Dielectric);
    dielectric_material.dielectric = Some(Dielectric::new(1.5));
    let mut boundary = HittableStruct::new(HittableKey::Sphere);
    boundary.sphere = Some(Sphere::new(
        &Point3::new(260.0, 150.0, 145.0),
        70.0,
        dielectric_material,
    ));
    hlist.add(boundary.clone());

    let mut obj = HittableStruct::new(HittableKey::ConstantMedium);
    let mut texture = TextureStruct::new(TextureKey::SolidColor);
    texture.solid_color = Some(SolidColor::new(Color::new(0.2, 0.4, 0.9)));
    obj.constant_medium = Some(Box::new(ConstantMedium::new(boundary, 0.2, texture)));
    hlist.add(obj);

    let mut boundary = HittableStruct::new(HittableKey::Sphere);
    let mut dielectric_material = MaterialStruct::new(MaterialKey::Dielectric);
    dielectric_material.dielectric = Some(Dielectric::new(1.5));
    boundary.sphere = Some(Sphere::new(
        &Point3::new(0.0, 0.0, 0.0),
        5000.0,
        dielectric_material,
    ));
    let mut obj = HittableStruct::new(HittableKey::ConstantMedium);
    let mut texture = TextureStruct::new(TextureKey::SolidColor);
    texture.solid_color = Some(SolidColor::new(Color::new(1.0, 1.0, 1.0)));
    obj.constant_medium = Some(Box::new(ConstantMedium::new(boundary, 0.0001, texture)));
    hlist.add(obj);

    // earth sphere
    let mut earth_material = MaterialStruct::new(MaterialKey::Lambertian);
    let mut texture = TextureStruct::new(TextureKey::ImageTexture);
    texture.image_texture = Some(ImageTexture::new("images/earth.png"));
    earth_material.lambertian = Some(Lambertian::new(&texture));
    let mut obj = HittableStruct::new(HittableKey::Sphere);
    obj.sphere = Some(Sphere::new(
        &Point3::new(400.0, 200.0, 400.0),
        100.0,
        earth_material,
    ));
    hlist.add(obj);

    // perlin sphere
    let mut pertext = TextureStruct::new(TextureKey::NoiseTexture);
    pertext.noise_texture = Some(NoiseTexture::new(0.1));
    let mut sphere_material = MaterialStruct::new(MaterialKey::Lambertian);
    sphere_material.lambertian = Some(Lambertian::new(&pertext));
    let mut obj = HittableStruct::new(HittableKey::Sphere);
    obj.sphere = Some(Sphere::new(
        &Point3::new(220.0, 280.0, 300.0),
        80.0,
        sphere_material,
    ));
    hlist.add(obj);

    // random spheres as box
    let mut box_world: Vec<HittableStruct> = Vec::new();
    let mut white = MaterialStruct::new(MaterialKey::Lambertian);
    let mut texture = TextureStruct::new(TextureKey::SolidColor);
    texture.solid_color = Some(SolidColor::new(Color::new(0.73, 0.73, 0.73)));
    white.lambertian = Some(Lambertian::new(&texture));
    let ns = 1000;
    for _ in 0..ns {
        let mut obj = HittableStruct::new(HittableKey::Sphere);
        obj.sphere = Some(Sphere::new(
            &Point3::rand_range(0.0, 165.0),
            10.0,
            white.clone(),
        ));
        box_world.push(obj);
    }

    let mut obj = HittableStruct::new(HittableKey::BvhNode);
    obj.bvh_node = Some(Box::new(BvhNode::new(&mut box_world, 0.0, 1.0)));

    let mut new_obj = HittableStruct::new(HittableKey::RotateY);
    new_obj.rotate_y = Some(Box::new(RotateY::new(obj, 15.0)));
    obj = new_obj;

    let mut new_obj = HittableStruct::new(HittableKey::Translation);
    new_obj.translation = Some(Translation::new(obj, Vec3::new(-100.0, 270.0, 395.0)));
    obj = new_obj;
    hlist.add(obj);

    let mut obj = HittableStruct::new(HittableKey::HittableList);
    obj.hittable_list = Some(Box::new(hlist));
    obj
}
