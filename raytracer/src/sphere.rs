#![allow(clippy::suspicious_operation_groupings)]

use crate::aabb::AABB;
use crate::hittable::{HitRecord, Hittable};
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::Vec3;
use crate::{INF, ONB, PI};
use std::sync::Arc;

pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
    pub mat_ptr: Arc<dyn Material>,
}

impl Sphere {
    pub fn new(cen: Vec3, r: f64, m: Arc<dyn Material>) -> Self {
        Self {
            center: Vec3 {
                x: cen.x,
                y: cen.y,
                z: cen.z,
            },
            radius: r,
            mat_ptr: m,
        }
    }

    fn get_sphere_uv(p: &Vec3) -> (f64, f64) {
        let theta = (-p.y).acos();
        let phi = (-p.z).atan2(p.x) + PI;
        (phi / (2.0 * PI), theta / PI)
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = r.orig - self.center;
        let a = r.dir.squared_length();
        let half_b = oc * r.dir;
        let c = oc.squared_length() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        } else {
            let sqrtd = discriminant.sqrt();
            let mut root = (-half_b - sqrtd) / a;
            if root < t_min || root > t_max {
                root = (-half_b + sqrtd) / a;
                if root < t_min || root > t_max {
                    return None;
                }
            }
            let mut rec = HitRecord::new();
            rec.t = root;
            rec.p = r.at(root);
            let outward_normal = (rec.p - self.center) / self.radius;
            rec.set_face_normal(r, &outward_normal);
            let tuple = Self::get_sphere_uv(&outward_normal);
            rec.u = tuple.0;
            rec.v = tuple.1;
            rec.mat_ptr = self.mat_ptr.clone();
            Some(rec)
        }
    }

    fn bounding_box(&self, time0: f64, time1: f64) -> Option<AABB> {
        let output_box = AABB {
            minimum: self.center - Vec3::new(self.radius, self.radius, self.radius),
            maximum: self.center + Vec3::new(self.radius, self.radius, self.radius),
        };
        Some(output_box)
    }

    fn pdf_value(&self, o: &Vec3, v: &Vec3) -> f64 {
        if let Some(rec) = self.hit(&Ray::new(o.clone(), v.clone(), 0.0), 0.001, INF) {
            let cos_theta_max = (1.0
                - self.radius * self.radius / (self.center - o.clone()).squared_length())
            .sqrt();
            let solid_angle = 2.0 * PI * (1.0 - cos_theta_max);
            1.0 / solid_angle
        } else {
            0.0
        }
    }

    fn random(&self, o: &Vec3) -> Vec3 {
        let direction = self.center - o.clone();
        let distance_squared = direction.squared_length();
        let mut uvw = ONB::new();
        uvw.build_from_w(direction);
        uvw.local_with_vec(Vec3::random_to_sphere(self.radius, distance_squared))
    }
}

pub struct MovingSphere {
    pub center0: Vec3,
    pub center1: Vec3,
    pub time0: f64,
    pub time1: f64,
    pub radius: f64,
    pub mat_ptr: Arc<dyn Material>,
}

impl MovingSphere {
    pub fn new(
        cen0: Vec3,
        cen1: Vec3,
        _time0: f64,
        _time1: f64,
        r: f64,
        m: Arc<dyn Material>,
    ) -> Self {
        Self {
            center0: cen0,
            center1: cen1,
            time0: _time0,
            time1: _time1,
            radius: r,
            mat_ptr: m,
        }
    }

    pub fn center(&self, time: f64) -> Vec3 {
        self.center0
            + (self.center1 - self.center0) * ((time - self.time0) / (self.time1 - self.time0))
    }
}

impl Hittable for MovingSphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = r.orig - self.center(r.tm);
        let a = r.dir.squared_length();
        let half_b = oc * r.dir;
        let c = oc.squared_length() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        } else {
            let sqrtd = discriminant.sqrt();
            let mut root = (-half_b - sqrtd) / a;
            if root < t_min || root > t_max {
                root = (-half_b + sqrtd) / a;
                if root < t_min || root > t_max {
                    return None;
                }
            }
            let mut rec = HitRecord::new();
            rec.t = root;
            rec.p = r.at(root);
            let outward_normal = (rec.p - self.center(r.tm)) / self.radius;
            rec.set_face_normal(r, &outward_normal);
            rec.mat_ptr = self.mat_ptr.clone();
            Some(rec)
        }
    }

    fn bounding_box(&self, time0: f64, time1: f64) -> Option<AABB> {
        let box0 = AABB {
            minimum: self.center(time0) - Vec3::new(self.radius, self.radius, self.radius),
            maximum: self.center(time0) + Vec3::new(self.radius, self.radius, self.radius),
        };
        let box1 = AABB {
            minimum: self.center(time1) - Vec3::new(self.radius, self.radius, self.radius),
            maximum: self.center(time1) + Vec3::new(self.radius, self.radius, self.radius),
        };
        let output_box = AABB::surrounding_box(&box0, &box1);
        Some(output_box)
    }
}
