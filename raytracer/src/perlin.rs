use crate::random_int;
use crate::vec3::Vec3;

pub struct Perlin {
    ranvec: Vec<Vec3>,
    perm_x: Vec<i32>,
    perm_y: Vec<i32>,
    perm_z: Vec<i32>,
}

impl Perlin {
    const POINT_COUNT: usize = 256;

    fn perlin_generate_perm() -> Vec<i32> {
        let mut p: Vec<i32> = vec![];
        for i in 0..Self::POINT_COUNT {
            p.push(i as i32);
        }
        Self::permute(&mut p, Self::POINT_COUNT);
        p
    }

    fn permute(p: &mut Vec<i32>, n: usize) {
        for i in (0..n - 1).rev() {
            let target = random_int(0, i as i32) as usize;
            let tmp = p[i];
            p[i] = p[target];
            p[target] = tmp;
        }
    }

    pub fn new() -> Self {
        let mut _ranvec: Vec<Vec3> = vec![];
        for i in 0..Self::POINT_COUNT {
            _ranvec.push(Vec3::random_min_max(-1.0, 1.0));
        }
        Self {
            ranvec: _ranvec,
            perm_x: Self::perlin_generate_perm(),
            perm_y: Self::perlin_generate_perm(),
            perm_z: Self::perlin_generate_perm(),
        }
    }

    pub fn noise(&self, p: &Vec3) -> f64 {
        let u = p.x - p.x.floor();
        let v = p.y - p.y.floor();
        let w = p.z - p.z.floor();
        let i = (p.x.floor()) as i32;
        let j = (p.y.floor()) as i32;
        let k = (p.z.floor()) as i32;
        let mut c: [[[Vec3; 2]; 2]; 2] = [[[Vec3::zero(); 2]; 2]; 2];
        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    c[di as usize][dj as usize][dk as usize] = self.ranvec[(self.perm_x
                        [((i + di) & 255) as usize]
                        ^ self.perm_y[((j + dj) & 255) as usize]
                        ^ self.perm_z[((k + dk) & 255) as usize])
                        as usize];
                }
            }
        }
        Self::trilinear_interp(&c, u, v, w)
    }

    fn trilinear_interp(c: &[[[Vec3; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
        let uu = u * u * (3.0 - 2.0 * u);
        let vv = v * v * (3.0 - 2.0 * v);
        let ww = w * w * (3.0 - 2.0 * w);
        let mut accum = 0.0;
        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let weight_v = Vec3::new(u - i as f64, v - j as f64, w - k as f64);
                    accum += (i as f64 * uu + (1.0 - i as f64) * (1.0 - uu))
                        * (j as f64 * vv + (1.0 - j as f64) * (1.0 - vv))
                        * (k as f64 * ww + (1.0 - k as f64) * (1.0 - ww))
                        * (c[i as usize][j as usize][k as usize] * weight_v);
                }
            }
        }
        accum
    }

    pub fn turb(&self, p: &Vec3, depth: i32) -> f64 {
        let mut accum = 0.0;
        let mut temp_p = p.clone();
        let mut weight = 1.0;
        for i in 0..depth {
            accum += weight * self.noise(&temp_p);
            weight *= 0.5;
            temp_p *= 2.0;
        }
        accum.abs()
    }
}
