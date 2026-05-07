use crate::vec3::Vec3;
use crate::ccd::CCD;

#[derive(Copy, Clone, Debug)]
pub struct Support {
    pub v: Vec3,
    pub v1: Vec3,
    pub v2: Vec3,
}

impl Support {
    pub fn copy_from(&mut self, other: &Support) {
        self.v = other.v;
        self.v1 = other.v1;
        self.v2 = other.v2;
    }
}

pub fn support<T>(obj1: &T, obj2: &T, dir: &Vec3, ccd: &CCD<T>) -> Support {
    let mut d = *dir;
    
    let v1 = (ccd.support1)(obj1, &d);
    
    d.scale(-1.0);
    let v2 = (ccd.support2)(obj2, &d);
    
    let v = v1 - v2;
    
    Support { v, v1, v2 }
}
