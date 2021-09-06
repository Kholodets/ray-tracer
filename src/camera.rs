use crate::vec3::Vec3;
use crate::ray::Ray;

pub struct Camera {
    origin: Vec3,
    llc: Vec3,
    hor: Vec3,
    vert: Vec3
}

impl Camera {
    pub fn new(vph: f64, vpw: f64, fcl: f64) -> Camera {
        let o = Vec3 {e: [0.0, 0.0, 0.0]};
        let h = Vec3 {e: [vpw, 0.0, 0.0]};
        let v = Vec3 {e: [0.0, vph, 0.0]};
        let l = o - h/2.0 - v/2.0 - Vec3 {e: [0.0, 0.0, fcl]};

        Camera {
            origin: o,
            llc: l,
            hor: h,
            vert: v
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(self.origin, self.llc + self.hor*u + self.vert*v - self.origin)
    }
}
