use std::ops::{Add, Sub, Mul, Div, Neg};
use std::f64;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Vec3 {
    pub v: [f64; 3],
}

pub static ORIGIN: Vec3 = Vec3 { v: [0.0, 0.0, 0.0] };

pub static POINTS_ON_SPHERE: [Vec3; 42] = [
    Vec3 { v: [0.000000, -0.000000, -1.000000] },
    Vec3 { v: [0.723608, -0.525725, -0.447219] },
    Vec3 { v: [-0.276388, -0.850649, -0.447219] },
    Vec3 { v: [-0.894426, -0.000000, -0.447216] },
    Vec3 { v: [-0.276388, 0.850649, -0.447220] },
    Vec3 { v: [0.723608, 0.525725, -0.447219] },
    Vec3 { v: [0.276388, -0.850649, 0.447220] },
    Vec3 { v: [-0.723608, -0.525725, 0.447219] },
    Vec3 { v: [-0.723608, 0.525725, 0.447219] },
    Vec3 { v: [0.276388, 0.850649, 0.447219] },
    Vec3 { v: [0.894426, 0.000000, 0.447216] },
    Vec3 { v: [-0.000000, 0.000000, 1.000000] },
    Vec3 { v: [0.425323, -0.309011, -0.850654] },
    Vec3 { v: [-0.162456, -0.499995, -0.850654] },
    Vec3 { v: [0.262869, -0.809012, -0.525738] },
    Vec3 { v: [0.425323, 0.309011, -0.850654] },
    Vec3 { v: [0.850648, -0.000000, -0.525736] },
    Vec3 { v: [-0.525730, -0.000000, -0.850652] },
    Vec3 { v: [-0.688190, -0.499997, -0.525736] },
    Vec3 { v: [-0.162456, 0.499995, -0.850654] },
    Vec3 { v: [-0.688190, 0.499997, -0.525736] },
    Vec3 { v: [0.262869, 0.809012, -0.525738] },
    Vec3 { v: [0.951058, 0.309013, 0.000000] },
    Vec3 { v: [0.951058, -0.309013, 0.000000] },
    Vec3 { v: [0.587786, -0.809017, 0.000000] },
    Vec3 { v: [0.000000, -1.000000, 0.000000] },
    Vec3 { v: [-0.587786, -0.809017, 0.000000] },
    Vec3 { v: [-0.951058, -0.309013, -0.000000] },
    Vec3 { v: [-0.951058, 0.309013, -0.000000] },
    Vec3 { v: [-0.587786, 0.809017, -0.000000] },
    Vec3 { v: [-0.000000, 1.000000, -0.000000] },
    Vec3 { v: [0.587786, 0.809017, -0.000000] },
    Vec3 { v: [0.688190, -0.499997, 0.525736] },
    Vec3 { v: [-0.262869, -0.809012, 0.525738] },
    Vec3 { v: [-0.850648, 0.000000, 0.525736] },
    Vec3 { v: [-0.262869, 0.809012, 0.525738] },
    Vec3 { v: [0.688190, 0.499997, 0.525736] },
    Vec3 { v: [0.525730, 0.000000, 0.850652] },
    Vec3 { v: [0.162456, -0.499995, 0.850654] },
    Vec3 { v: [-0.425323, -0.309011, 0.850654] },
    Vec3 { v: [-0.425323, 0.309011, 0.850654] },
    Vec3 { v: [0.162456, 0.499995, 0.850654] },
];

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { v: [x, y, z] }
    }

    pub fn x(&self) -> f64 {
        self.v[0]
    }

    pub fn y(&self) -> f64 {
        self.v[1]
    }

    pub fn z(&self) -> f64 {
        self.v[2]
    }

    pub fn len2(&self) -> f64 {
        self.dot(self)
    }

    pub fn len(&self) -> f64 {
        self.len2().sqrt()
    }

    pub fn dist2(&self, other: &Vec3) -> f64 {
        (*self - *other).len2()
    }

    pub fn dist(&self, other: &Vec3) -> f64 {
        (*self - *other).len()
    }

    pub fn dot(&self, other: &Vec3) -> f64 {
        self.v[0] * other.v[0] + self.v[1] * other.v[1] + self.v[2] * other.v[2]
    }

    pub fn cross(&self, other: &Vec3) -> Vec3 {
        Vec3 {
            v: [
                self.v[1] * other.v[2] - self.v[2] * other.v[1],
                self.v[2] * other.v[0] - self.v[0] * other.v[2],
                self.v[0] * other.v[1] - self.v[1] * other.v[0],
            ],
        }
    }

    pub fn normalize(&mut self) {
        let k = 1.0 / self.len();
        self.v[0] *= k;
        self.v[1] *= k;
        self.v[2] *= k;
    }

    pub fn normalized(&self) -> Vec3 {
        let mut v = *self;
        v.normalize();
        v
    }

    pub fn scale(&mut self, k: f64) {
        self.v[0] *= k;
        self.v[1] *= k;
        self.v[2] *= k;
    }

    pub fn scaled(&self, k: f64) -> Vec3 {
        Vec3 {
            v: [self.v[0] * k, self.v[1] * k, self.v[2] * k],
        }
    }

    pub fn is_zero(&self) -> bool {
        self.len2() < f64::EPSILON
    }

    pub fn eq(&self, other: &Vec3) -> bool {
        self.x().eq_eps(other.x()) && self.y().eq_eps(other.y()) && self.z().eq_eps(other.z())
    }
}

pub trait EpsEq {
    fn eq_eps(&self, other: Self) -> bool;
}

impl EpsEq for f64 {
    fn eq_eps(&self, other: f64) -> bool {
        let ab = (*self - other).abs();
        if ab < f64::EPSILON {
            return true;
        }
        let a = self.abs();
        let b = other.abs();
        if b > a {
            ab < f64::EPSILON * b
        } else {
            ab < f64::EPSILON * a
        }
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Vec3 {
        Vec3 {
            v: [
                self.v[0] + other.v[0],
                self.v[1] + other.v[1],
                self.v[2] + other.v[2],
            ],
        }
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Vec3 {
        Vec3 {
            v: [
                self.v[0] - other.v[0],
                self.v[1] - other.v[1],
                self.v[2] - other.v[2],
            ],
        }
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, k: f64) -> Vec3 {
        self.scaled(k)
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, k: f64) -> Vec3 {
        self.scaled(1.0 / k)
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        Vec3 {
            v: [-self.v[0], -self.v[1], -self.v[2]],
        }
    }
}

pub fn sign(val: f64) -> i32 {
    if val.is_zero_eps() {
        0
    } else if val < 0.0 {
        -1
    } else {
        1
    }
}

pub trait ZeroCheck {
    fn is_zero_eps(&self) -> bool;
}

impl ZeroCheck for f64 {
    fn is_zero_eps(&self) -> bool {
        self.abs() < f64::EPSILON
    }
}

pub fn point_segment_dist2(p: &Vec3, x0: &Vec3, b: &Vec3, witness: &mut Option<Vec3>) -> f64 {
    let d = *b - *x0;
    let a = *x0 - *p;
    
    let t = -a.dot(&d) / d.len2();
    
    if t < 0.0 || t.is_zero_eps() {
        let dist = x0.dist2(p);
        if let Some(w) = witness {
            *w = *x0;
        }
        dist
    } else if t > 1.0 || t.eq_eps(1.0) {
        let dist = b.dist2(p);
        if let Some(w) = witness {
            *w = *b;
        }
        dist
    } else {
        if let Some(w) = witness {
            *w = d.scaled(t) + *x0;
            w.dist2(p)
        } else {
            (d.scaled(t) + a).len2()
        }
    }
}

pub fn point_tri_dist2(p: &Vec3, x0: &Vec3, b: &Vec3, c: &Vec3, witness: &mut Option<Vec3>) -> f64 {
    let d1 = *b - *x0;
    let d2 = *c - *x0;
    let a = *x0 - *p;
    
    let u = a.dot(&a);
    let v = d1.dot(&d1);
    let w = d2.dot(&d2);
    let p_dot = a.dot(&d1);
    let q = a.dot(&d2);
    let r = d1.dot(&d2);
    
    let denom = w * v - r * r;
    
    let (s, t) = if denom.is_zero_eps() {
        (-1.0, -1.0)
    } else {
        let s_val = (q * r - w * p_dot) / denom;
        let t_val = (-s_val * r - q) / w;
        (s_val, t_val)
    };
    
    if (s > 0.0 || s.is_zero_eps()) 
        && (s < 1.0 || s.eq_eps(1.0)) 
        && (t > 0.0 || t.is_zero_eps()) 
        && (t < 1.0 || t.eq_eps(1.0)) 
        && (t + s < 1.0 || (t + s).eq_eps(1.0)) 
    {
        if let Some(w) = witness {
            *w = d1.scaled(s) + d2.scaled(t) + *x0;
            w.dist2(p)
        } else {
            s * s * v + t * t * w + 2.0 * s * t * r + 2.0 * s * p_dot + 2.0 * t * q + u
        }
    } else {
        let mut w1 = None;
        let dist = point_segment_dist2(p, x0, b, &mut w1);
        
        let mut w2 = None;
        let dist2 = point_segment_dist2(p, x0, c, &mut w2);
        
        let dist = if dist2 < dist {
            if let Some(w) = witness {
                *w = w2.unwrap();
            }
            dist2
        } else {
            if let Some(w) = witness {
                *w = w1.unwrap();
            }
            dist
        };
        
        let mut w3 = None;
        let dist3 = point_segment_dist2(p, b, c, &mut w3);
        
        if dist3 < dist {
            if let Some(w) = witness {
                *w = w3.unwrap();
            }
            dist3
        } else {
            dist
        }
    }
}
