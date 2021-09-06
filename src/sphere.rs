use crate::ray::Ray;
use crate::vec3::Vec3;
use crate::HitRecord;
use crate::Object;
use crate::Material;

#[derive(Copy, Clone)]
pub struct Sphere<'c> {
    pub center: Vec3,
    pub radius: f64,
    pub mat: &'c Material
}

impl Object for Sphere<'_> {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = ray.orig() - self.center;
        let a = ray.dir().length_squared();
        let half_b = oc.dot(&ray.dir());
        let c = oc.length_squared() - self.radius*self.radius;

        let discriminant = half_b*half_b - a*c;
        if discriminant < 0.0 {
            return None;
        }

        let sqrtd = discriminant.sqrt();
        let root = (half_b*-1.0 - sqrtd) / a;
        let root = if root < t_min || root > t_max { (half_b*-1.0 + sqrtd) / a } else { root };
        if root < t_min || root > t_max {
            return None;
        }

        let point = ray.at(root);

        let n = (point - self.center) / self.radius;

        let face = ray.dir().dot(&n) < 0.0;

        Some(HitRecord {
            mat: self.mat,
            t: root,
            p: point,
            norm: n,
            ff: face,
        })
    }
}
