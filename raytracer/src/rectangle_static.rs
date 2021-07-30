use crate::aabb::AABB;
use crate::hittable_static::{HitRecord, Hittable};
use crate::material_static::Material;
use crate::ray::Ray;
use crate::vec3::Vec3;
use crate::{random_min_max, INF};
use std::sync::Arc;

#[derive(Clone)]
pub struct xyRect<T: Material> {
    mp: T,
    x0: f64,
    y0: f64,
    x1: f64,
    y1: f64,
    k: f64,
}

impl<T: Material> xyRect<T> {
    pub fn new(_x0: f64, _x1: f64, _y0: f64, _y1: f64, _k: f64, mat: T) -> Self {
        Self {
            mp: mat,
            x0: _x0,
            y0: _y0,
            x1: _x1,
            y1: _y1,
            k: _k,
        }
    }
}

impl<T: 'static + Clone + Material> Hittable for xyRect<T> {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let t = (self.k - r.orig.z) / r.dir.z;
        if t < t_min || t > t_max {
            return None;
        }
        let x = r.orig.x + t * r.dir.x;
        let y = r.orig.y + t * r.dir.y;
        if x < self.x0 || x > self.x1 || y < self.y0 || y > self.y1 {
            return None;
        }
        let mut rec = HitRecord {
            p: r.at(t),
            normal: Vec3::zero(),
            t,
            front_face: false,
            mat_ptr: Arc::new(self.mp.clone()),
            u: (x - self.x0) / (self.x1 - self.x0),
            v: (y - self.y0) / (self.y1 - self.y0),
        };
        let outward_normal = Vec3::new(0.0, 0.0, 1.0);
        rec.set_face_normal(r, &outward_normal);
        Some(rec)
    }

    fn bounding_box(&self, time0: f64, time1: f64) -> Option<AABB> {
        let output_box = AABB::new(
            Vec3::new(self.x0, self.y0, self.k - 0.0001),
            Vec3::new(self.x1, self.y1, self.k + 0.0001),
        );
        Some(output_box)
    }
}

#[derive(Clone)]
pub struct xzRect<T: Material> {
    mp: T,
    x0: f64,
    z0: f64,
    x1: f64,
    z1: f64,
    k: f64,
}

impl<T: Material> xzRect<T> {
    pub fn new(_x0: f64, _x1: f64, _z0: f64, _z1: f64, _k: f64, mat: T) -> Self {
        Self {
            mp: mat,
            x0: _x0,
            z0: _z0,
            x1: _x1,
            z1: _z1,
            k: _k,
        }
    }
}

impl<T: 'static + Clone + Material> Hittable for xzRect<T> {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let t = (self.k - r.orig.y) / r.dir.y;
        if t < t_min || t > t_max {
            return None;
        }
        let x = r.orig.x + t * r.dir.x;
        let z = r.orig.z + t * r.dir.z;
        if x < self.x0 || x > self.x1 || z < self.z0 || z > self.z1 {
            return None;
        }
        let mut rec = HitRecord {
            p: r.at(t),
            normal: Vec3::zero(),
            t,
            front_face: false,
            mat_ptr: Arc::new(self.mp.clone()),
            u: (x - self.x0) / (self.x1 - self.x0),
            v: (z - self.z0) / (self.z1 - self.z0),
        };
        let outward_normal = Vec3::new(0.0, 1.0, 0.0);
        rec.set_face_normal(r, &outward_normal);
        Some(rec)
    }

    fn bounding_box(&self, time0: f64, time1: f64) -> Option<AABB> {
        let output_box = AABB::new(
            Vec3::new(self.x0, self.k - 0.0001, self.z0),
            Vec3::new(self.x1, self.k + 0.0001, self.z1),
        );
        Some(output_box)
    }

    fn pdf_value(&self, o: &Vec3, v: &Vec3) -> f64 {
        if let Some(rec) = self.hit(&Ray::new(o.clone(), v.clone(), 0.0), 0.001, INF) {
            let area = (self.x1 - self.x0) * (self.z1 - self.z0);
            let distance_squared = rec.t * rec.t * v.squared_length();
            let cosine = (v.clone() * rec.normal / v.length()).abs();
            distance_squared / (cosine * area)
        } else {
            0.0
        }
    }

    fn random(&self, o: &Vec3) -> Vec3 {
        let random_point = Vec3::new(
            random_min_max(self.x0, self.x1),
            self.k,
            random_min_max(self.z0, self.z1),
        );
        random_point - o.clone()
    }
}

#[derive(Clone)]
pub struct yzRect<T: Material> {
    mp: T,
    y0: f64,
    z0: f64,
    y1: f64,
    z1: f64,
    k: f64,
}

impl<T: Material> yzRect<T> {
    pub fn new(_y0: f64, _y1: f64, _z0: f64, _z1: f64, _k: f64, mat: T) -> Self {
        Self {
            mp: mat,
            y0: _y0,
            z0: _z0,
            y1: _y1,
            z1: _z1,
            k: _k,
        }
    }
}

impl<T: 'static + Clone + Material> Hittable for yzRect<T> {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let t = (self.k - r.orig.x) / r.dir.x;
        if t < t_min || t > t_max {
            return None;
        }
        let y = r.orig.y + t * r.dir.y;
        let z = r.orig.z + t * r.dir.z;
        if y < self.y0 || y > self.y1 || z < self.z0 || z > self.z1 {
            return None;
        }
        let mut rec = HitRecord {
            p: r.at(t),
            normal: Vec3::zero(),
            t,
            front_face: false,
            mat_ptr: Arc::new(self.mp.clone()),
            u: (y - self.y0) / (self.y1 - self.y0),
            v: (z - self.z0) / (self.z1 - self.z0),
        };
        let outward_normal = Vec3::new(1.0, 0.0, 0.0);
        rec.set_face_normal(r, &outward_normal);
        Some(rec)
    }

    fn bounding_box(&self, time0: f64, time1: f64) -> Option<AABB> {
        let output_box = AABB::new(
            Vec3::new(self.k - 0.0001, self.y0, self.z0),
            Vec3::new(self.k + 0.0001, self.y1, self.z1),
        );
        Some(output_box)
    }
}
