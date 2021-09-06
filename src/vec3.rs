use rand::Rng;

#[derive(Debug, Copy, Clone)]
pub struct Vec3 {
    pub e: [f64; 3],
}

impl Vec3 {
    pub fn random_unit_vector() -> Vec3 {
        let mut rng =  rand::thread_rng();
        Vec3 {e:[
            rng.gen::<f64>(),
            rng.gen::<f64>(),
            rng.gen::<f64>()
        ]}
    }

    pub fn near_zero(&self) -> bool {
        let s = 1e-8;
        self.x().abs() < s && self.y().abs() < s && self.z().abs() < s
    }

    pub fn x(&self) -> f64 {
        self.e[0]
    }
    
    pub fn y(&self) -> f64 {
        self.e[1]
    }
    
    pub fn z(&self) -> f64 {
        self.e[2]
    }

    pub fn length(&self) -> f64 {
        (self.length_squared()).sqrt()
    }

    pub fn length_squared(&self) -> f64 {
        self.e[0]*self.e[0] + self.e[1]*self.e[1] + self.e[2]*self.e[2]
    }

    pub fn dot(&self, v: &Vec3) -> f64 {
        self.x()*v.x() + self.y()*v.y() + self.z()*v.z()
    }

    pub fn cross(&self, v: &Vec3) -> Vec3 {
        Vec3 {e: [self.y() * v.z() - self.z() * v.y(),
               self.z() * v.x() - self.x() * v.z(),
               self.x() * v.y() - self.y() * v.x()]}
    }

    pub fn unit_vector(&self) -> Vec3 {
        *self / self.length()
    }
}

impl std::ops::Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, v: Vec3) -> Vec3 {
        Vec3 {e: [self.x() + v.x(), self.y() + v.y(), self.z() + v.z()]}
    }
}

impl std::ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, v: Vec3) -> Vec3 {
        Vec3{e: [self.x() - v.x(), self.y() - v.y(), self.z() - v.z()]}
    }
}

impl std::ops::Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, v: Vec3) -> Vec3 {
        Vec3 {e: [self.x() * v.x(), self.y() * v.y(), self.z() * v.z()]}
    }
}

impl std::ops::Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, t: f64) -> Vec3 {
        Vec3 {e: [self.x() * t, self.y() * t, self.z() * t]}
    }
}

impl std::ops::Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, t: f64) -> Vec3 {
        Vec3 {e: [self.x() / t, self.y() / t, self.z() / t]}
    }
}
