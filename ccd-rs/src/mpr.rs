use crate::vec3::{Vec3, ORIGIN, ZeroCheck, EpsEq};
use crate::ccd::CCD;
use crate::support::Support;
use crate::support;

struct Simplex {
    ps: [Support; 4],
    last: i32,
}

impl Simplex {
    fn new() -> Simplex {
        Simplex {
            ps: [Support { v: ORIGIN, v1: ORIGIN, v2: ORIGIN }; 4],
            last: -1,
        }
    }

    fn size(&self) -> i32 {
        self.last + 1
    }

    fn point(&self, idx: i32) -> &Support {
        &self.ps[idx as usize]
    }

    fn point_mut(&mut self, idx: i32) -> &mut Support {
        &mut self.ps[idx as usize]
    }

    fn add(&mut self, s: &Support) {
        self.last += 1;
        self.ps[self.last as usize] = *s;
    }

    fn set(&mut self, pos: usize, s: &Support) {
        self.ps[pos] = *s;
    }

    fn set_size(&mut self, size: i32) {
        self.last = size - 1;
    }

    fn swap(&mut self, pos1: usize, pos2: usize) {
        self.ps.swap(pos1, pos2);
    }
}

fn find_origin<T>(obj1: &T, obj2: &T, ccd: &CCD<T>) -> Support {
    let v1 = (ccd.center1)(obj1);
    let v2 = (ccd.center2)(obj2);
    let v = v1 - v2;
    Support { v, v1, v2 }
}

fn portal_dir(portal: &Simplex) -> Vec3 {
    let v2v1 = portal.point(2).v - portal.point(1).v;
    let v3v1 = portal.point(3).v - portal.point(1).v;
    v2v1.cross(&v3v1).normalized()
}

fn portal_encapsules_origin(portal: &Simplex, dir: &Vec3) -> bool {
    let dot = dir.dot(&portal.point(1).v);
    dot.is_zero_eps() || dot > 0.0
}

fn portal_reach_tolerance<T>(portal: &Simplex, v4: &Support, dir: &Vec3, ccd: &CCD<T>) -> bool {
    let dv1 = portal.point(1).v.dot(dir);
    let dv2 = portal.point(2).v.dot(dir);
    let dv3 = portal.point(3).v.dot(dir);
    let dv4 = v4.v.dot(dir);
    
    let dot1 = dv4 - dv1;
    let dot2 = dv4 - dv2;
    let dot3 = dv4 - dv3;
    
    let min_dot = dot1.min(dot2).min(dot3);
    
    min_dot < ccd.mpr_tolerance || min_dot.eq_eps(ccd.mpr_tolerance)
}

fn portal_can_encapsule_origin(_portal: &Simplex, v4: &Support, dir: &Vec3) -> bool {
    let dot = v4.v.dot(dir);
    dot.is_zero_eps() || dot > 0.0
}

fn expand_portal(portal: &mut Simplex, v4: &Support) {
    let v4v0 = v4.v.cross(&portal.point(0).v);
    
    let dot = portal.point(1).v.dot(&v4v0);
    if dot > 0.0 {
        let dot = portal.point(2).v.dot(&v4v0);
        if dot > 0.0 {
            portal.set(1, v4);
        } else {
            portal.set(3, v4);
        }
    } else {
        let dot = portal.point(3).v.dot(&v4v0);
        if dot > 0.0 {
            portal.set(2, v4);
        } else {
            portal.set(1, v4);
        }
    }
}

fn discover_portal<T>(obj1: &T, obj2: &T, ccd: &CCD<T>, portal: &mut Simplex) -> i32 {
    let mut dir: Vec3;
    let mut va: Vec3;
    let mut vb: Vec3;
    let mut dot: f64;
    let mut cont: i32;
    
    *portal.point_mut(0) = find_origin(obj1, obj2, ccd);
    portal.set_size(1);
    
    if portal.point(0).v.eq(&ORIGIN) {
        let va = Vec3::new(1e-5, 0.0, 0.0);
        portal.point_mut(0).v = portal.point(0).v + va;
    }
    
    dir = portal.point(0).v;
    dir.scale(-1.0);
    dir.normalize();
    
    *portal.point_mut(1) = support::support(obj1, obj2, &dir, ccd);
    portal.set_size(2);
    
    dot = portal.point(1).v.dot(&dir);
    if dot.is_zero_eps() || dot < 0.0 {
        return -1;
    }
    
    dir = portal.point(0).v.cross(&portal.point(1).v);
    if dir.len2().is_zero_eps() {
        if portal.point(1).v.eq(&ORIGIN) {
            return 1;
        } else {
            return 2;
        }
    }
    
    dir.normalize();
    *portal.point_mut(2) = support::support(obj1, obj2, &dir, ccd);
    dot = portal.point(2).v.dot(&dir);
    if dot.is_zero_eps() || dot < 0.0 {
        return -1;
    }
    
    portal.set_size(3);
    
    va = portal.point(1).v - portal.point(0).v;
    vb = portal.point(2).v - portal.point(0).v;
    dir = va.cross(&vb);
    dir.normalize();
    
    dot = dir.dot(&portal.point(0).v);
    if dot > 0.0 {
        portal.swap(1, 2);
        dir.scale(-1.0);
    }
    
    while portal.size() < 4 {
        *portal.point_mut(3) = support::support(obj1, obj2, &dir, ccd);
        dot = portal.point(3).v.dot(&dir);
        if dot.is_zero_eps() || dot < 0.0 {
            return -1;
        }
        
        cont = 0;
        
        let p1 = *portal.point(1);
        let p3 = *portal.point(3);
        va = p1.v.cross(&p3.v);
        dot = va.dot(&portal.point(0).v);
        if dot < 0.0 && !dot.is_zero_eps() {
            portal.set(2, &p3);
            cont = 1;
        }
        
        if cont == 0 {
            let p3 = *portal.point(3);
            let p2 = *portal.point(2);
            va = p3.v.cross(&p2.v);
            dot = va.dot(&portal.point(0).v);
            if dot < 0.0 && !dot.is_zero_eps() {
                portal.set(1, &p3);
                cont = 1;
            }
        }
        
        if cont == 1 {
            va = portal.point(1).v - portal.point(0).v;
            vb = portal.point(2).v - portal.point(0).v;
            dir = va.cross(&vb);
            dir.normalize();
        } else {
            portal.set_size(4);
        }
    }
    
    0
}

fn refine_portal<T>(obj1: &T, obj2: &T, ccd: &CCD<T>, portal: &mut Simplex) -> i32 {
    let mut dir: Vec3;
    let mut v4: Support;
    
    loop {
        dir = portal_dir(portal);
        
        if portal_encapsules_origin(portal, &dir) {
            return 0;
        }
        
        v4 = support::support(obj1, obj2, &dir, ccd);
        
        if !portal_can_encapsule_origin(portal, &v4, &dir) || portal_reach_tolerance(portal, &v4, &dir, ccd) {
            return -1;
        }
        
        expand_portal(portal, &v4);
    }
}

pub fn mpr_intersect<T>(obj1: &T, obj2: &T, ccd: &CCD<T>) -> bool {
    let mut portal = Simplex::new();
    let res = discover_portal(obj1, obj2, ccd, &mut portal);
    
    if res < 0 {
        return false;
    }
    if res > 0 {
        return true;
    }
    
    let res = refine_portal(obj1, obj2, ccd, &mut portal);
    res == 0
}
