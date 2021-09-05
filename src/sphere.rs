#[derive(Debug, Copy, Clone)]
pub struct Sphere {
    pub center: Vec3,
    pub radius: Vec3,
    pub material: Material
}

impl Object for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> HitRecord {
        let oc = ray.orig() - self.center;
        let a = ray.dir().length_squared();
        let half_b = oc.dot(ray.dir());
        let c = oc.length_squared() - self.radius*self.radius;

        let discriminant = half_b*half_b - a*c;
        let h = discriminant >= 0;
        let sqrtd = if h { discriminant.sqrt() } else { 0 }
        let root = if h {
            (half_b*-1 + sqrtd) / a
        } else {
            -1.0
        }
        
