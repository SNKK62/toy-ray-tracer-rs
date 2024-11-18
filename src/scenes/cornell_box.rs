use crate::hittable::{BvhNode, Hittable, HittableList, XYRect, XZRect, YZRect};
use crate::material::{DiffuseLight, Lambertian};
use crate::texture::SolidColor;
use crate::vec3::Color;
use std::{cell::RefCell, rc::Rc};

pub fn scene() -> HittableList {
    let mut world: Vec<Rc<dyn Hittable>> = Vec::new();

    let red = Lambertian::new(Rc::new(SolidColor::new(Color::new(0.65, 0.05, 0.05))));
    let white = Lambertian::new(Rc::new(SolidColor::new(Color::new(0.73, 0.73, 0.73))));
    let green = Lambertian::new(Rc::new(SolidColor::new(Color::new(0.12, 0.45, 0.15))));
    let light = DiffuseLight::new(Rc::new(SolidColor::new(Color::new(15.0, 15.0, 15.0))));

    world.push(Rc::new(YZRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        Rc::new(RefCell::new(green)),
    )));
    world.push(Rc::new(YZRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        0.0,
        Rc::new(RefCell::new(red)),
    )));
    world.push(Rc::new(XZRect::new(
        213.0,
        343.0,
        227.0,
        332.0,
        554.0,
        Rc::new(RefCell::new(light.clone())),
    )));
    world.push(Rc::new(XZRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        0.0,
        Rc::new(RefCell::new(white.clone())),
    )));
    world.push(Rc::new(XZRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        Rc::new(RefCell::new(white.clone())),
    )));
    world.push(Rc::new(XYRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        Rc::new(RefCell::new(white.clone())),
    )));

    let bvh = BvhNode::new(&mut world, 0.0, 0.0);

    let mut world = HittableList::new();
    world.add(Rc::new(bvh));
    world
}
