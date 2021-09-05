use crate::HitRecord;
use crate::Object;
use crate::ray::Ray;
use crate::vec3::Vec3;
use std::cell::RefCell;

pub struct Scene {
    v: RefCell<Vec<dyn Object>>
}

impl Scene {
    

    pub fn add(&self, o: &Object) {
        (*self).v.push(o);
    }
}

impl Object for Scene {
    fn hit(&self, ray: &Ray, t_min, t_max) -> Option<HitRecord> {
        let &mut chr: HitRecord;
        let &mut smallest_t: f64 = f64::INFINITY;
        let h = false;
        for o in self.v {
            let hr = o.hit(ray, t_min, t_max);
            if hr.t < smallest_t {
                h = true;
                smallest_t = hr.t;
                chr = hr;
            }
        }

        if h {
            Some(chr)
        } else {
            None
        }
    }
}
