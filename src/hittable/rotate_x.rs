use super::{HitRecord, Hittable, HittableEnum, AABB};
use crate::degrees_to_radians;
use crate::ray::Ray;
use crate::vec3::Vec3;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RotateX {
    ptr: HittableEnum,
    sin_theta: f64,
    cos_theta: f64,
    hasbox: bool,
    bbox: AABB,
}

impl RotateX {
    pub fn new(ptr: HittableEnum, angle: f64) -> Self {
        let radians = degrees_to_radians(angle);
        let sin_theta = f64::sin(radians);
        let cos_theta = f64::cos(radians);
        let mut bbox = AABB::new(&Vec3::zero(), &Vec3::zero());
        let hasbox = ptr.bounding_box(0.0, 1.0, &mut bbox);
        let mut min = Vec3::new(f64::INFINITY, f64::INFINITY, f64::INFINITY);
        let mut max = Vec3::new(f64::NEG_INFINITY, f64::NEG_INFINITY, f64::NEG_INFINITY);

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let x = i as f64 * bbox.max.x() + (1 - i) as f64 * bbox.min.x();
                    let y = j as f64 * bbox.max.y() + (1 - j) as f64 * bbox.min.y();
                    let z = k as f64 * bbox.max.z() + (1 - k) as f64 * bbox.min.z();
                    let newy = cos_theta * y - sin_theta * z;
                    let newz = sin_theta * y + cos_theta * z;

                    let tester = Vec3::new(x, newy, newz);
                    for c in 0..3 {
                        min[c] = min[c].min(tester[c]);
                        max[c] = max[c].max(tester[c]);
                    }
                }
            }
        }
        let bbox = AABB::new(&min, &max);

        Self {
            ptr,
            sin_theta,
            cos_theta,
            hasbox,
            bbox,
        }
    }
}

impl Hittable for RotateX {
    fn hit(&self, r: &crate::ray::Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let origin = r.origin;
        let direction = r.direction;
        let origin = Vec3::new(
            origin.x(),
            self.cos_theta * origin.y() + self.sin_theta * origin.z(),
            -self.sin_theta * origin.y() + self.cos_theta * origin.z(),
        );
        let direction = Vec3::new(
            direction.x(),
            self.cos_theta * direction.y() + self.sin_theta * direction.z(),
            -self.sin_theta * direction.y() + self.cos_theta * direction.z(),
        );
        let rotated_r = Ray::new(&origin, &direction, r.time);
        if !self.ptr.hit(&rotated_r, t_min, t_max, rec) {
            return false;
        }
        let mut p = rec.p;
        let mut normal = rec.normal;

        p[1] = self.cos_theta * rec.p[1] - self.sin_theta * rec.p[2];
        p[2] = self.sin_theta * rec.p[1] + self.cos_theta * rec.p[2];
        normal[1] = self.cos_theta * rec.normal[1] - self.sin_theta * rec.normal[2];
        normal[2] = self.sin_theta * rec.normal[1] + self.cos_theta * rec.normal[2];

        rec.p = p;
        self.set_front_face(&rotated_r, &normal, rec);

        true
    }

    fn bounding_box(&self, _time0: f64, _time1: f64, output_box: &mut AABB) -> bool {
        *output_box = self.bbox.clone();
        self.hasbox
    }
}
