use crate::HitRecord;
use crate::Object;
use crate::ray::Ray;
use crate::vec3::Vec3;
use crate::lambertian::Lambertian;

pub struct Scene<'a> {
    pub list: Vec<&'a dyn Object>
}

impl Scene<'_> {
    pub fn add(&mut self, o: &'static dyn Object) {
        self.list.push(o);
    }
}

impl Object for Scene<'_> {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut chr: HitRecord = HitRecord {
            t:0.0,p:Vec3{e:[0.0,0.0,0.0]},norm:Vec3{e:[0.0,0.0,0.0]},/*ff:false,*/
            mat: &(Lambertian {color: Vec3 {e: [0.0, 0.0, 0.0]}})
        };
        let mut smallest_t: f64 = f64::INFINITY;
        let mut h = false;
        for o in &self.list {
            let res = o.hit(ray, t_min, t_max);
            match res {
                Some(hr) =>{
                    if hr.t < smallest_t {
                        h = true;
                        smallest_t = hr.t;
                        chr = hr;
                    }
                },

                None => { }
            }
        }

        if h {
            Some(chr)
        } else {
            None
        }
    }
}
