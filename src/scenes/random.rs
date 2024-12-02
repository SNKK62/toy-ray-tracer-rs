use crate::hittable::{BvhNode, HittableKey, HittableStruct, Sphere};
use crate::material::{Dielectric, Lambertian, MaterialKey, MaterialStruct, Metal};
use crate::texture::{Checker, SolidColor, TextureKey, TextureStruct};
use crate::vec3::{Color, Point3};
use rand::Rng;

pub fn scene() -> HittableStruct {
    let mut world: Vec<HittableStruct> = Vec::new();
    let mut checker = TextureStruct::new(TextureKey::Checker);
    let mut texture1 = TextureStruct::new(TextureKey::SolidColor);
    texture1.solid_color = Some(SolidColor::new(Color::new(0.2, 0.3, 0.1)));
    let mut texture2 = TextureStruct::new(TextureKey::SolidColor);
    texture2.solid_color = Some(SolidColor::new(Color::new(0.9, 0.9, 0.9)));
    checker.checker = Some(Checker::new(texture1, texture2));

    let mut ground_material = MaterialStruct::new(MaterialKey::Lambertian);
    ground_material.lambertian = Some(Lambertian::new(&checker));

    let mut sphere = HittableStruct::new(HittableKey::Sphere);
    sphere.sphere = Some(Sphere::new(
        &Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material.clone(),
    ));
    world.push(sphere);

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rand::thread_rng().gen_range(0.0..1.0);
            let center = Point3::new(
                (a as f64) + 0.9 * rand::thread_rng().gen_range(0.0..1.0),
                0.2,
                (b as f64) + 0.9 * rand::thread_rng().gen_range(0.0..1.0),
            );

            let radius = 0.2;
            if (center - Point3::new(4.0, radius, 0.0)).len() > 0.9 {
                let mut sphere_material: MaterialStruct;
                if choose_mat < 0.7 {
                    // diffuse
                    let albedo = Color::rand() * Color::rand();
                    sphere_material = MaterialStruct::new(MaterialKey::Lambertian);
                    let mut texture = TextureStruct::new(TextureKey::SolidColor);
                    texture.solid_color = Some(SolidColor::new(albedo));
                    sphere_material.lambertian = Some(Lambertian::new(&texture));
                    let mut sphere = HittableStruct::new(HittableKey::Sphere);
                    sphere.sphere = Some(Sphere::new(&center, radius, sphere_material));
                    world.push(sphere);
                } else if choose_mat < 0.85 {
                    // metal
                    let albedo = Color::rand_range(0.5, 1.0);
                    let fuzz = rand::thread_rng().gen_range(0.0..0.5);
                    sphere_material = MaterialStruct::new(MaterialKey::Metal);
                    sphere_material.metal = Some(Metal::new(&albedo, fuzz));
                    let mut sphere = HittableStruct::new(HittableKey::Sphere);
                    sphere.sphere = Some(Sphere::new(&center, radius, sphere_material));
                    world.push(sphere);
                } else {
                    // glass
                    sphere_material = MaterialStruct::new(MaterialKey::Dielectric);
                    sphere_material.dielectric = Some(Dielectric::new(1.5));
                    let mut sphere = HittableStruct::new(HittableKey::Sphere);
                    sphere.sphere = Some(Sphere::new(&center, radius, sphere_material));
                    world.push(sphere);
                }
            }
        }
    }

    let mut material1 = MaterialStruct::new(MaterialKey::Dielectric);
    material1.dielectric = Some(Dielectric::new(1.5));
    let mut obj = HittableStruct::new(HittableKey::Sphere);
    obj.sphere = Some(Sphere::new(&Point3::new(0.0, 1.0, 0.0), 1.0, material1));
    world.push(obj);

    let mut material2 = MaterialStruct::new(MaterialKey::Lambertian);
    let mut texture = TextureStruct::new(TextureKey::SolidColor);
    texture.solid_color = Some(SolidColor::new(Color::new(0.4, 0.2, 0.1)));
    material2.lambertian = Some(Lambertian::new(&texture));
    let mut obj = HittableStruct::new(HittableKey::Sphere);
    obj.sphere = Some(Sphere::new(&Point3::new(-4.0, 1.0, 0.0), 1.0, material2));
    world.push(obj);

    let mut material3 = MaterialStruct::new(MaterialKey::Metal);
    material3.metal = Some(Metal::new(&Color::new(0.7, 0.6, 0.5), 0.0));
    let mut obj = HittableStruct::new(HittableKey::Sphere);
    obj.sphere = Some(Sphere::new(&Point3::new(4.0, 1.0, 0.0), 1.0, material3));
    world.push(obj);

    let mut bvh = HittableStruct::new(HittableKey::BvhNode);
    bvh.bvh_node = Some(Box::new(BvhNode::new(&mut world, 0.0, 1.0)));
    bvh
}
