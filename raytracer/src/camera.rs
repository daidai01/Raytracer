use crate::vec3::Vec3;
use crate::Ray;
use crate::{degrees_to_radians, random_min_max};

#[derive(Clone,Copy)]
pub struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    lens_radius: f64,
    time0: f64,
    time1: f64,
}

impl Camera {
    pub fn new(
        lookfrom: Vec3,
        lookat: Vec3,
        vup: Vec3,
        vfov: f64,
        aspect_ratio: f64,
        aperture: f64,
        focus_dist: f64,
        _time0: f64,
        _time1: f64,
    ) -> Self {
        let theta = degrees_to_radians(vfov);
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (lookfrom - lookat).unit();
        let u = Vec3::cross(vup, w).unit();
        let v = Vec3::cross(w, u);

        let mut cam = Self {
            origin: lookfrom,
            horizontal: u * viewport_width * focus_dist,
            vertical: v * viewport_height * focus_dist,
            lower_left_corner: Vec3::zero(),
            u,
            v,
            w,
            lens_radius: aperture / 2.0,
            time0: _time0,
            time1: _time1,
        };
        cam.lower_left_corner =
            cam.origin - cam.horizontal / 2.0 - cam.vertical / 2.0 - w * focus_dist;
        cam
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let rd = Vec3::random_in_unit_disk() * self.lens_radius;
        let offset = self.u * rd.x + self.v * rd.y;
        Ray {
            orig: self.origin + offset,
            dir: self.lower_left_corner + self.horizontal * s + self.vertical * t
                - self.origin
                - offset,
            tm: random_min_max(self.time0, self.time1),
        }
    }
}
