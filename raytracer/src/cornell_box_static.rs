use crate::aabb::AABB;
use crate::hittable_static::{HitRecord, Hittable};
use crate::hittable_list::HittableList;
use crate::material_static::Material;
use crate::ray::Ray;
use crate::rectangle_static::{xyRect, xzRect, yzRect};
use crate::vec3::Vec3;
use std::sync::Arc;

#[derive(Clone)]
pub struct Box {
    pub box_min: Vec3,
    pub box_max: Vec3,
    pub sides: HittableList,
}

impl Box {
    pub fn new<T: 'static + Material + Clone>(p0: Vec3, p1: Vec3, ptr: T) -> Self {
        let mut _box = Self {
            box_min: p0,
            box_max: p1,
            sides: HittableList::new(),
        };
        _box.sides.add(Arc::new(xyRect::new(
            p0.x,
            p1.x,
            p0.y,
            p1.y,
            p1.z,
            ptr.clone(),
        )));
        _box.sides.add(Arc::new(xyRect::new(
            p0.x,
            p1.x,
            p0.y,
            p1.y,
            p0.z,
            ptr.clone(),
        )));
        _box.sides.add(Arc::new(xzRect::new(
            p0.x,
            p1.x,
            p0.z,
            p1.z,
            p1.y,
            ptr.clone(),
        )));
        _box.sides.add(Arc::new(xzRect::new(
            p0.x,
            p1.x,
            p0.z,
            p1.z,
            p0.y,
            ptr.clone(),
        )));
        _box.sides.add(Arc::new(yzRect::new(
            p0.y,
            p1.y,
            p0.z,
            p1.z,
            p1.x,
            ptr.clone(),
        )));
        _box.sides.add(Arc::new(yzRect::new(
            p0.y,
            p1.y,
            p0.z,
            p1.z,
            p0.x,
            ptr.clone(),
        )));
        _box
    }
}

impl Hittable for Box {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        self.sides.hit(r, t_min, t_max)
    }

    fn bounding_box(&self, time0: f64, time1: f64) -> Option<AABB> {
        let output_box = AABB::new(self.box_min, self.box_max);
        Some(output_box)
    }
}
