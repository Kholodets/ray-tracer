use crate::ray::Ray;
use crate::vec3::Vec3;
use crate::HitRecord;
use crate::Object;
use crate::Material;

#[derive(Copy, Clone)]
pub struct Triangle<'c> {
    pub vec0: Vec3,
    pub vec1: Vec3,
    pub vec2: Vec3,
    pub norm: Vec3,
    pub mat: &'c dyn Material
}



impl Object for Triangle<'_> {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let d = self.norm.dot(&self.vec0);
        let t =(self.norm.dot(&ray.orig())+d) / self.norm.dot(&ray.dir());
        let p = ray.orig()+ray.dir()*t;
        
        if (t > t_max || t < t_min) {
            return None;
        }

        if (self.norm.dot(&((self.vec1-self.vec0).cross(&(p-self.vec0)))) <= 0.0 ||
            self.norm.dot(&((self.vec2-self.vec1).cross(&(p-self.vec1)))) <= 0.0 ||
            self.norm.dot(&((self.vec0-self.vec2).cross(&(p-self.vec2)))) <= 0.0) {
            return None;
        }

        Some(HitRecord {
            mat: self.mat,
            t: t,
            p: p,
            norm: self.norm,
            //ff: face,
        })
    }
}

impl Triangle<'_> {
    pub fn new(vec0: Vec3, vec1: Vec3, vec2: Vec3, mat: &dyn Material) -> Triangle {
        let n = (vec1-vec0).cross(&(vec2-vec0));
        Triangle {
            vec0: vec0,
            vec1: vec1,
            vec2: vec2,
            norm: n,
            mat: mat
        }
    }
}
