use crate::vec3::Vec3;

pub struct ONB {
    pub u: Vec3,
    pub v: Vec3,
    pub w: Vec3,
}

impl ONB {
    pub fn at(&self, i: usize) -> Vec3 {
        if i == 0 {
            self.u
        } else if i == 1 {
            self.v
        } else if i == 2 {
            self.w
        } else {
            panic!("INVALID INDEX")
        }
    }

    pub fn new() -> Self {
        Self {
            u: Vec3::zero(),
            v: Vec3::zero(),
            w: Vec3::zero(),
        }
    }

    pub fn local_with_f64(&self, a: f64, b: f64, c: f64) -> Vec3 {
        self.u * a + self.v * b + self.w * c
    }

    pub fn local_with_vec(&self, a: Vec3) -> Vec3 {
        self.u * a.x + self.v * a.y + self.w * a.z
    }

    pub fn build_from_w(&mut self, n: Vec3) {
        self.w = n.unit();
        let a = if self.w.x.abs() > 0.9 {
            Vec3::new(0.0, 1.0, 0.0)
        } else {
            Vec3::new(1.0, 0.0, 0.0)
        };
        self.v = Vec3::cross(self.w, a).unit();
        self.u = Vec3::cross(self.w, self.v);
    }
}
