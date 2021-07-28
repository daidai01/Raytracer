use crate::ray::Ray;
use crate::vec3::Vec3;
use std::mem;

#[derive(Clone)]
pub struct AABB {
    pub minimum: Vec3,
    pub maximum: Vec3,
}

impl AABB {
    pub fn new(a: Vec3, b: Vec3) -> Self {
        Self {
            minimum: a,
            maximum: b,
        }
    }

    pub fn hit(&self, r: &Ray, mut t_min: f64, mut t_max: f64) -> bool {
        for i in 0..3 {
            let invD = 1.0 / r.dir.at(i);
            let mut t0 = (self.minimum.at(i) - r.orig.at(i)) * invD;
            let mut t1 = (self.maximum.at(i) - r.orig.at(i)) * invD;
            if invD < 0.0 {
                mem::swap(&mut t0, &mut t1);
            }
            t_min = if t0 > t_min { t0 } else { t_min };
            t_max = if t1 < t_max { t1 } else { t_max };
            if t_max <= t_min {
                return false;
            }
        }
        true
    }

    pub fn surrounding_box(box0: &AABB, box1: &AABB) -> Self {
        let small = Vec3 {
            x: if box0.minimum.x < box1.minimum.x {
                box0.minimum.x
            } else {
                box1.minimum.x
            },
            y: if box0.minimum.y < box1.minimum.y {
                box0.minimum.y
            } else {
                box1.minimum.y
            },
            z: if box0.minimum.z < box1.minimum.z {
                box0.minimum.z
            } else {
                box1.minimum.z
            },
        };
        let big = Vec3 {
            x: if box0.maximum.x > box1.maximum.x {
                box0.maximum.x
            } else {
                box1.maximum.x
            },
            y: if box0.maximum.y > box1.maximum.y {
                box0.maximum.y
            } else {
                box1.maximum.y
            },
            z: if box0.maximum.z > box1.maximum.z {
                box0.maximum.z
            } else {
                box1.maximum.z
            },
        };
        Self {
            minimum: small,
            maximum: big,
        }
    }
}
