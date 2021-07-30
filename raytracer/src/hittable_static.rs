use crate::aabb::AABB;
use crate::material_static::{Lambertian, Material};
use crate::ray::Ray;
use crate::texture_static::SolidColor;
use crate::vec3::Vec3;
use crate::{degrees_to_radians, random_0_1, INF};
use std::sync::Arc;

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;

    fn bounding_box(&self, time0: f64, time1: f64) -> Option<AABB>;

    fn pdf_value(&self, o: &Vec3, v: &Vec3) -> f64 {
        0.0
    }

    fn random(&self, o: &Vec3) -> Vec3 {
        Vec3::new(1.0, 0.0, 0.0)
    }
}

#[derive(Clone)]
pub struct HitRecord {
    pub p: Vec3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
    pub mat_ptr: Arc<dyn Material>,
    pub u: f64,
    pub v: f64,
}

impl HitRecord {
    pub fn new() -> Self {
        Self {
            p: Vec3::zero(),
            normal: Vec3::zero(),
            t: 0.0,
            front_face: false,
            mat_ptr: Arc::new(Lambertian::new(SolidColor::new_with_vec(Vec3::zero()))),
            u: 0.0,
            v: 0.0,
        }
    }

    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3) {
        self.front_face = r.dir * (*outward_normal) < 0.0;
        if self.front_face {
            self.normal = *outward_normal;
        } else {
            self.normal = -*outward_normal;
        }
    }
}

#[derive(Clone)]
pub struct Translate<T: Hittable> {
    pub ptr: T,
    pub offset: Vec3,
}

impl<T: Clone + Hittable> Translate<T> {
    pub fn new(p: T, displacement: Vec3) -> Self {
        Self {
            ptr: p.clone(),
            offset: displacement,
        }
    }
}

impl<T: Clone + Hittable> Hittable for Translate<T> {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let moved_r = Ray::new(r.orig - self.offset, r.dir, r.tm);
        if let Some(temp_rec) = self.ptr.hit(&moved_r, t_min, t_max) {
            let mut rec = temp_rec.clone();
            rec.p += self.offset;
            rec.set_face_normal(&moved_r, &temp_rec.normal);
            Some(rec)
        } else {
            None
        }
    }

    fn bounding_box(&self, time0: f64, time1: f64) -> Option<AABB> {
        if let Some(temp_box) = self.ptr.bounding_box(time0, time1) {
            let output_box = AABB::new(
                temp_box.minimum + self.offset,
                temp_box.maximum + self.offset,
            );
            Some(output_box)
        } else {
            None
        }
    }
}

#[derive(Clone)]
pub struct RotateY<T: Hittable> {
    pub ptr: T,
    pub sin_theta: f64,
    pub cos_theta: f64,
    pub has_box: bool,
    pub bbox: AABB,
}

impl<T: Clone + Hittable> RotateY<T> {
    pub fn new(p: T, angle: f64) -> Self {
        let radians = degrees_to_radians(angle);
        let sin = radians.sin();
        let cos = radians.cos();
        let mut _hasbox = false;
        let mut _bbox = AABB::new(Vec3::zero(), Vec3::zero());
        if let Some(temp_bbox) = p.clone().bounding_box(0.0, 1.0) {
            _hasbox = true;
            _bbox = temp_bbox;
        }
        let mut min = Vec3::new(INF, INF, INF);
        let mut max = Vec3::new(-INF, -INF, -INF);
        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let x = i as f64 * _bbox.maximum.x + (1.0 - i as f64) * _bbox.minimum.x;
                    let y = j as f64 * _bbox.maximum.y + (1.0 - j as f64) * _bbox.minimum.y;
                    let z = k as f64 * _bbox.maximum.z + (1.0 - k as f64) * _bbox.minimum.z;
                    let newx = cos * x + sin * z;
                    let newz = -sin * x + cos * z;
                    let tester = Vec3::new(newx, y, newz);
                    min.x = min.x.min(tester.x);
                    max.x = max.x.max(tester.x);
                    min.y = min.y.min(tester.y);
                    max.y = max.y.max(tester.y);
                    min.z = min.z.min(tester.z);
                    max.z = max.z.max(tester.z);
                }
            }
        }
        Self {
            ptr: p.clone(),
            sin_theta: sin,
            cos_theta: cos,
            has_box: _hasbox,
            bbox: AABB::new(min, max),
        }
    }
}

impl<T: Clone + Hittable> Hittable for RotateY<T> {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut origin = r.orig;
        let mut direction = r.dir;
        origin.x = self.cos_theta * r.orig.x - self.sin_theta * r.orig.z;
        origin.z = self.sin_theta * r.orig.x + self.cos_theta * r.orig.z;
        direction.x = self.cos_theta * r.dir.x - self.sin_theta * r.dir.z;
        direction.z = self.sin_theta * r.dir.x + self.cos_theta * r.dir.z;
        let rotate_r = Ray::new(origin, direction, r.tm);
        if let Some(temp_rec) = self.ptr.hit(&rotate_r, t_min, t_max) {
            let mut rec = temp_rec.clone();
            let mut p = temp_rec.p;
            let mut normal = temp_rec.normal;
            p.x = self.cos_theta * temp_rec.p.x + self.sin_theta * temp_rec.p.z;
            p.z = -self.sin_theta * temp_rec.p.x + self.cos_theta * temp_rec.p.z;
            normal.x = self.cos_theta * temp_rec.normal.x + self.sin_theta * temp_rec.normal.z;
            normal.z = -self.sin_theta * temp_rec.normal.x + self.cos_theta * temp_rec.normal.z;
            rec.p = p;
            rec.set_face_normal(&rotate_r, &normal);
            Some(rec)
        } else {
            None
        }
    }

    fn bounding_box(&self, time0: f64, time1: f64) -> Option<AABB> {
        if self.has_box {
            Some(self.bbox.clone())
        } else {
            None
        }
    }
}

#[derive(Clone)]
pub struct ConstantMedium<B: Hittable, P: Material> {
    pub boundary: B,
    pub phase_function: P,
    pub neg_inv_density: f64,
}

impl<B: Hittable, P: Material> ConstantMedium<B, P> {
    // pub fn new_with_arc<T: Texture>(b: B, d: f64, a: T) -> Self {
    //     Self {
    //         boundary: b,
    //         neg_inv_density: -1.0 / d,
    //         phase_function: Isotropic::<T>::new_with_arc(a),
    //     }
    // }
    //
    // pub fn new_with_col(b: B, d: f64, c: Vec3) -> Self {
    //     Self {
    //         boundary: b,
    //         neg_inv_density: -1.0 / d,
    //         phase_function: Isotropic::<SolidColor>::new_with_col(c),
    //     }
    // }

    pub fn new(b: B, d: f64, p: P) -> Self {
        Self {
            boundary: b,
            neg_inv_density: -1.0 / d,
            phase_function: p,
        }
    }
}

impl<B: Clone + Hittable, P: 'static + Clone + Material> Hittable for ConstantMedium<B, P> {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        if let Some(mut rec1) = self.boundary.hit(r, -INF, INF) {
            if let Some(mut rec2) = self.boundary.hit(r, rec1.t + 0.0001, INF) {
                if rec1.t < t_min {
                    rec1.t = t_min;
                }
                if rec2.t > t_max {
                    rec2.t = t_max;
                }
                if rec1.t >= rec2.t {
                    return None;
                }
                if rec1.t < 0.0 {
                    rec1.t = 0.0;
                }
                let ray_length = r.dir.length();
                let distance_inside_boundary = (rec2.t - rec1.t) * ray_length;
                let hit_distance = self.neg_inv_density * random_0_1().ln();
                if hit_distance > distance_inside_boundary {
                    return None;
                }
                let rec = HitRecord {
                    p: r.at(rec1.t + hit_distance / ray_length),
                    normal: Vec3::new(1.0, 0.0, 0.0),
                    t: rec1.t + hit_distance / ray_length,
                    front_face: true,
                    mat_ptr: Arc::new(self.phase_function.clone()),
                    u: 0.0,
                    v: 0.0,
                };
                Some(rec)
            } else {
                None
            }
        } else {
            None
        }
    }

    fn bounding_box(&self, time0: f64, time1: f64) -> Option<AABB> {
        self.boundary.bounding_box(time0, time1)
    }
}

#[derive(Clone)]
pub struct FlipFace<T: Hittable> {
    pub ptr: T,
}

impl<T: Hittable> FlipFace<T> {
    pub fn new(p: T) -> Self {
        Self { ptr: p }
    }
}

impl<T: Clone + Hittable> Hittable for FlipFace<T> {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        if let Some(mut rec) = self.ptr.hit(r, t_min, t_max) {
            rec.front_face = !rec.front_face;
            Some(rec)
        } else {
            None
        }
    }

    fn bounding_box(&self, time0: f64, time1: f64) -> Option<AABB> {
        self.ptr.bounding_box(time0, time1)
    }
}
