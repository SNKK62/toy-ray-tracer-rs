use crate::material;
use crate::ray;
use crate::vec3;

pub mod aabb;
pub use aabb::AABB;

pub mod bvh;
pub use bvh::BvhNode;

pub mod hittable_list;
pub use hittable_list::HittableList;

pub mod moving_sphere;
pub use moving_sphere::MovingSphere;

pub mod sphere;
pub use sphere::Sphere;

pub mod xy_rect;
pub use xy_rect::XYRect;
pub mod xz_rect;
pub use xz_rect::XZRect;
pub mod yz_rect;
pub use yz_rect::YZRect;

pub mod cuboid;
pub use cuboid::Cuboid;

pub mod translation;
pub use translation::Translation;

pub mod rotate_x;
pub use rotate_x::RotateX;
pub mod rotate_y;
pub use rotate_y::RotateY;
pub mod rotate_z;
pub use rotate_z::RotateZ;

pub mod constant_medium;
pub use constant_medium::ConstantMedium;

use std::boxed::Box;
use std::fmt::Debug;

/// p should be a unit sphere
fn get_sphere_uv(p: &vec3::Point3) -> (f64, f64) {
    let pi = std::f64::consts::PI;
    let phi = f64::atan2(p.z(), p.x());
    let theta = f64::asin(p.y());
    let u = 1.0 - (phi + pi) / (2.0 * pi);
    let v = (theta + pi / 2.0) / pi;
    (u, v)
}

#[derive(Clone)]
pub struct HitRecord {
    pub p: vec3::Point3,
    pub normal: vec3::Vec3,
    pub material: Option<material::MaterialStruct>,
    pub t: f64,
    pub u: f64,
    pub v: f64,
    pub front_face: bool,
}

impl Default for HitRecord {
    fn default() -> Self {
        Self::new()
    }
}

impl HitRecord {
    pub fn new() -> Self {
        HitRecord {
            p: vec3::Point3::zero(),
            normal: vec3::Vec3::zero(),
            material: None,
            t: 0.0,
            u: 0.0,
            v: 0.0,
            front_face: false,
        }
    }
}

pub trait Hittable {
    fn hit(&self, r: &ray::Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut AABB) -> bool;
    fn set_front_face(&self, r: &ray::Ray, outward_normal: &vec3::Vec3, record: &mut HitRecord) {
        let is_front_face = r.direction.dot(outward_normal) <= 0.0;
        record.front_face = is_front_face;
        record.normal = if is_front_face {
            *outward_normal
        } else {
            -(*outward_normal)
        };
    }
}

#[derive(Debug, Clone)]
pub enum HittableKey {
    HittableList,
    BvhNode,
    Sphere,
    MovingSphere,
    XYRect,
    XZRect,
    YZRect,
    Cuboid,
    Translation,
    RotateX,
    RotateY,
    RotateZ,
    ConstantMedium,
}

// NOTE: This is a workaround for compiling into WASM
#[derive(Debug, Clone)]
pub struct HittableStruct {
    pub key: HittableKey,
    pub hittable_list: Option<Box<HittableList>>,
    pub bvh_node: Option<Box<BvhNode>>,
    pub sphere: Option<Sphere>,
    pub moving_sphere: Option<MovingSphere>,
    pub xy_rect: Option<XYRect>,
    pub xz_rect: Option<XZRect>,
    pub yz_rect: Option<YZRect>,
    pub cuboid: Option<Cuboid>,
    pub translation: Option<Translation>,
    pub rotate_x: Option<Box<RotateX>>,
    pub rotate_y: Option<Box<RotateY>>,
    pub rotate_z: Option<Box<RotateZ>>,
    pub constant_medium: Option<Box<ConstantMedium>>,
}

impl HittableStruct {
    pub fn new(key: HittableKey) -> Self {
        Self {
            key,
            hittable_list: None,
            bvh_node: None,
            sphere: None,
            moving_sphere: None,
            xy_rect: None,
            xz_rect: None,
            yz_rect: None,
            cuboid: None,
            translation: None,
            rotate_x: None,
            rotate_y: None,
            rotate_z: None,
            constant_medium: None,
        }
    }

    pub fn hit(&self, r: &ray::Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        match self.key {
            HittableKey::HittableList => self
                .hittable_list
                .as_ref()
                .unwrap()
                .hit(r, t_min, t_max, rec),
            HittableKey::BvhNode => self.bvh_node.as_ref().unwrap().hit(r, t_min, t_max, rec),
            HittableKey::Sphere => self.sphere.as_ref().unwrap().hit(r, t_min, t_max, rec),
            HittableKey::MovingSphere => self
                .moving_sphere
                .as_ref()
                .unwrap()
                .hit(r, t_min, t_max, rec),
            HittableKey::XYRect => self.xy_rect.as_ref().unwrap().hit(r, t_min, t_max, rec),
            HittableKey::XZRect => self.xz_rect.as_ref().unwrap().hit(r, t_min, t_max, rec),
            HittableKey::YZRect => self.yz_rect.as_ref().unwrap().hit(r, t_min, t_max, rec),
            HittableKey::Cuboid => self.cuboid.as_ref().unwrap().hit(r, t_min, t_max, rec),
            HittableKey::Translation => {
                self.translation.as_ref().unwrap().hit(r, t_min, t_max, rec)
            }
            HittableKey::RotateX => self.rotate_x.as_ref().unwrap().hit(r, t_min, t_max, rec),
            HittableKey::RotateY => self.rotate_y.as_ref().unwrap().hit(r, t_min, t_max, rec),
            HittableKey::RotateZ => self.rotate_z.as_ref().unwrap().hit(r, t_min, t_max, rec),
            HittableKey::ConstantMedium => self
                .constant_medium
                .as_ref()
                .unwrap()
                .hit(r, t_min, t_max, rec),
        }
    }

    pub fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut AABB) -> bool {
        match self.key {
            HittableKey::HittableList => self
                .hittable_list
                .as_ref()
                .unwrap()
                .bounding_box(time0, time1, output_box),
            HittableKey::BvhNode => self
                .bvh_node
                .as_ref()
                .unwrap()
                .bounding_box(time0, time1, output_box),
            HittableKey::Sphere => self
                .sphere
                .as_ref()
                .unwrap()
                .bounding_box(time0, time1, output_box),
            HittableKey::MovingSphere => self
                .moving_sphere
                .as_ref()
                .unwrap()
                .bounding_box(time0, time1, output_box),
            HittableKey::XYRect => self
                .xy_rect
                .as_ref()
                .unwrap()
                .bounding_box(time0, time1, output_box),
            HittableKey::XZRect => self
                .xz_rect
                .as_ref()
                .unwrap()
                .bounding_box(time0, time1, output_box),
            HittableKey::YZRect => self
                .yz_rect
                .as_ref()
                .unwrap()
                .bounding_box(time0, time1, output_box),
            HittableKey::Cuboid => self
                .cuboid
                .as_ref()
                .unwrap()
                .bounding_box(time0, time1, output_box),
            HittableKey::Translation => self
                .translation
                .as_ref()
                .unwrap()
                .bounding_box(time0, time1, output_box),
            HittableKey::RotateX => self
                .rotate_x
                .as_ref()
                .unwrap()
                .bounding_box(time0, time1, output_box),
            HittableKey::RotateY => self
                .rotate_y
                .as_ref()
                .unwrap()
                .bounding_box(time0, time1, output_box),
            HittableKey::RotateZ => self
                .rotate_z
                .as_ref()
                .unwrap()
                .bounding_box(time0, time1, output_box),
            HittableKey::ConstantMedium => self
                .constant_medium
                .as_ref()
                .unwrap()
                .bounding_box(time0, time1, output_box),
        }
    }
}

fn surrounding_box(box0: &AABB, box1: &AABB) -> AABB {
    let small = vec3::Point3::new(
        box0.min.x().min(box1.min.x()),
        box0.min.y().min(box1.min.y()),
        box0.min.z().min(box1.min.z()),
    );
    let big = vec3::Point3::new(
        box0.max.x().max(box1.max.x()),
        box0.max.y().max(box1.max.y()),
        box0.max.z().max(box1.max.z()),
    );
    AABB::new(&small, &big)
}
