use crate::aabb::AABB;
use crate::hittable_list::HittableList;
use crate::hittable_static::{HitRecord, Hittable};
use crate::material_static::Lambertian;
use crate::random_int;
use crate::ray::Ray;
use crate::sphere_static::Sphere;
use crate::texture_static::SolidColor;
use crate::vec3::Vec3;
use std::sync::Arc;

#[derive(Clone)]
pub struct bvhNode {
    pub left: Arc<dyn Hittable>,
    pub right: Arc<dyn Hittable>,
    pub my_box: AABB,
}

impl bvhNode {
    pub fn new_with_list(list: &mut HittableList, time0: f64, time1: f64) -> Self {
        let len = list.objects.len();
        Self::new_with_vec(&mut list.objects, 0, len, time0, time1)
    }

    pub fn new_with_vec(
        objects: &mut Vec<Arc<dyn Hittable>>,
        start: usize,
        end: usize,
        time0: f64,
        time1: f64,
    ) -> Self {
        let mut node = bvhNode {
            left: Arc::new(Sphere::new(
                Vec3::zero(),
                0.0,
                Lambertian::new(SolidColor::new_with_vec(Vec3::zero())),
            )),
            right: Arc::new(Sphere::new(
                Vec3::zero(),
                0.0,
                Lambertian::new(SolidColor::new_with_vec(Vec3::zero())),
            )),
            my_box: AABB::new(Vec3::zero(), Vec3::zero()),
        };
        let axis = random_int(0, 2);
        let object_span = end - start;
        if object_span == 1 {
            node.left = objects[start].clone();
            node.right = objects[start].clone();
        } else if object_span == 2 {
            if Self::box_compare(&objects[start], &objects[start + 1], axis).0 {
                node.left = objects[start].clone();
                node.right = objects[start + 1].clone();
            } else {
                node.left = objects[start + 1].clone();
                node.right = objects[start].clone();
            }
        } else {
            objects.sort_by(|a, b| {
                (Self::box_compare(&a, &b, axis).1)
                    .partial_cmp(&Self::box_compare(&a, &b, axis).2)
                    .unwrap()
            });
            let mid = start + object_span / 2;
            node.left = Arc::new(bvhNode::new_with_vec(objects, start, mid, time0, time1));
            node.right = Arc::new(bvhNode::new_with_vec(objects, mid, end, time0, time1));
        }
        if let Some(box_left) = node.left.bounding_box(time0, time1) {
            if let Some(box_right) = node.right.bounding_box(time0, time1) {
                node.my_box = AABB::surrounding_box(&box_left, &box_right);
                node
            } else {
                panic!("NO BOUNDING BOX IN bvhNode CONSTRUCTOR");
            }
        } else {
            panic!("NO BOUNDING BOX IN bvhNode CONSTRUCTOR");
        }
    }

    pub fn box_compare(
        a: &Arc<dyn Hittable>,
        b: &Arc<dyn Hittable>,
        axis: i32,
    ) -> (bool, f64, f64) {
        let mut temp_a = 0.0;
        let mut temp_b = 0.0;
        if let Some(box_a) = a.bounding_box(0.0, 0.0) {
            temp_a = box_a.minimum.at(axis);
        } else {
            panic!("NO BOUNDING BOX IN bvhNode CONSTRUCTOR");
        }
        if let Some(box_b) = b.bounding_box(0.0, 0.0) {
            temp_b = box_b.minimum.at(axis);
        } else {
            panic!("NO BOUNDING BOX IN bvhNode CONSTRUCTOR");
        }
        (temp_a < temp_b, temp_a, temp_b)
    }
}

impl Hittable for bvhNode {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        if !self.my_box.hit(r, t_min, t_max) {
            return None;
        }
        return if let Some(hit_left) = self.left.hit(r, t_min, t_max) {
            if let Some(hit_right) = self.right.hit(r, t_min, hit_left.t) {
                Some(hit_right)
            } else {
                Some(hit_left)
            }
        } else {
            if let Some(hit_right) = self.right.hit(r, t_min, t_max) {
                Some(hit_right)
            } else {
                None
            }
        };
    }

    fn bounding_box(&self, time0: f64, time1: f64) -> Option<AABB> {
        Some(self.my_box.clone())
    }
}
