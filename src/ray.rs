use crate::vec3::Vec3;

pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub fn orig(&self) -> Vec3 {
        self.origin
    }

    pub fn dir(&self) -> Vec3 {
        self.direction
    }

    pub fn at(&self, t: f64) -> Vec3 {
        self.origin + self.direction * t
    }
}
