use crate::vec3::{Vec3, ORIGIN, point_tri_dist2, ZeroCheck};
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

    fn last(&self) -> &Support {
        &self.ps[self.last as usize]
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

fn triple_cross(a: &Vec3, b: &Vec3, c: &Vec3) -> Vec3 {
    let e = a.cross(b);
    e.cross(c)
}

fn do_simplex2(simplex: &mut Simplex, dir: &mut Vec3) -> i32 {
    let a = *simplex.last();
    let b = *simplex.point(0);
    
    let ab = b.v - a.v;
    let mut ao = a.v;
    ao.scale(-1.0);
    
    let dot = ab.dot(&ao);
    
    let tmp = ab.cross(&ao);
    if tmp.len2().is_zero_eps() && dot > 0.0 {
        return 1;
    }
    
    if dot.is_zero_eps() || dot < 0.0 {
        simplex.set(0, &a);
        simplex.set_size(1);
        *dir = ao;
    } else {
        *dir = triple_cross(&ab, &ao, &ab);
    }
    
    0
}

fn do_simplex3(simplex: &mut Simplex, dir: &mut Vec3) -> i32 {
    let a = *simplex.last();
    let b = *simplex.point(1);
    let c = *simplex.point(0);
    
    let dist = point_tri_dist2(&ORIGIN, &a.v, &b.v, &c.v, &mut None);
    if dist.is_zero_eps() {
        return 1;
    }
    
    if a.v.eq(&b.v) || a.v.eq(&c.v) {
        return -1;
    }
    
    let mut ao = a.v;
    ao.scale(-1.0);
    
    let ab = b.v - a.v;
    let ac = c.v - a.v;
    let abc = ab.cross(&ac);
    
    let tmp = abc.cross(&ac);
    let dot = tmp.dot(&ao);
    
    if dot.is_zero_eps() || dot > 0.0 {
        let dot = ac.dot(&ao);
        if dot.is_zero_eps() || dot > 0.0 {
            simplex.set(1, &a);
            simplex.set_size(2);
            *dir = triple_cross(&ac, &ao, &ac);
        } else {
            let ab_val = ab;
            return do_simplex3_part2(simplex, a, b, ao, ab_val, dir);
        }
    } else {
        let tmp = ab.cross(&abc);
        let dot = tmp.dot(&ao);
        if dot.is_zero_eps() || dot > 0.0 {
            let ab_val = ab;
            return do_simplex3_part2(simplex, a, b, ao, ab_val, dir);
        } else {
            let dot = abc.dot(&ao);
            if dot.is_zero_eps() || dot > 0.0 {
                *dir = abc;
            } else {
                simplex.set(0, &b);
                simplex.set(1, &c);
                *dir = abc;
                dir.scale(-1.0);
            }
        }
    }
    
    0
}

fn do_simplex3_part2(simplex: &mut Simplex, a: Support, b: Support, ao: Vec3, ab: Vec3, dir: &mut Vec3) -> i32 {
    let dot = ab.dot(&ao);
    if dot.is_zero_eps() || dot > 0.0 {
        simplex.set(0, &b);
        simplex.set(1, &a);
        simplex.set_size(2);
        *dir = triple_cross(&ab, &ao, &ab);
    } else {
        simplex.set(0, &a);
        simplex.set_size(1);
        *dir = ao;
    }
    0
}

fn do_simplex4(simplex: &mut Simplex, dir: &mut Vec3) -> i32 {
    let a = *simplex.last();
    let b = *simplex.point(2);
    let c = *simplex.point(1);
    let d = *simplex.point(0);
    
    let dist = point_tri_dist2(&a.v, &b.v, &c.v, &d.v, &mut None);
    if dist.is_zero_eps() {
        return -1;
    }
    
    let dist = point_tri_dist2(&ORIGIN, &a.v, &b.v, &c.v, &mut None);
    if dist.is_zero_eps() {
        return 1;
    }
    let dist = point_tri_dist2(&ORIGIN, &a.v, &c.v, &d.v, &mut None);
    if dist.is_zero_eps() {
        return 1;
    }
    let dist = point_tri_dist2(&ORIGIN, &a.v, &b.v, &d.v, &mut None);
    if dist.is_zero_eps() {
        return 1;
    }
    let dist = point_tri_dist2(&ORIGIN, &b.v, &c.v, &d.v, &mut None);
    if dist.is_zero_eps() {
        return 1;
    }
    
    let mut ao = a.v;
    ao.scale(-1.0);
    
    let ab = b.v - a.v;
    let ac = c.v - a.v;
    let ad = d.v - a.v;
    
    let abc = ab.cross(&ac);
    let acd = ac.cross(&ad);
    let adb = ad.cross(&ab);
    
    let b_on_acd = crate::vec3::sign(acd.dot(&ab));
    let c_on_adb = crate::vec3::sign(adb.dot(&ac));
    let d_on_abc = crate::vec3::sign(abc.dot(&ad));
    
    let ab_o = crate::vec3::sign(acd.dot(&ao)) == b_on_acd;
    let ac_o = crate::vec3::sign(adb.dot(&ao)) == c_on_adb;
    let ad_o = crate::vec3::sign(abc.dot(&ao)) == d_on_abc;
    
    if ab_o && ac_o && ad_o {
        return 1;
    }
    
    if !ab_o {
        simplex.set(2, &a);
        simplex.set_size(3);
    } else if !ac_o {
        simplex.set(1, &d);
        simplex.set(0, &b);
        simplex.set(2, &a);
        simplex.set_size(3);
    } else {
        simplex.set(0, &c);
        simplex.set(1, &b);
        simplex.set(2, &a);
        simplex.set_size(3);
    }
    
    do_simplex3(simplex, dir)
}

fn do_simplex(simplex: &mut Simplex, dir: &mut Vec3) -> i32 {
    match simplex.size() {
        2 => do_simplex2(simplex, dir),
        3 => do_simplex3(simplex, dir),
        _ => do_simplex4(simplex, dir),
    }
}

fn gjk<T>(obj1: &T, obj2: &T, ccd: &CCD<T>, simplex: &mut Simplex) -> i32 {
    let mut dir: Vec3;
    let mut last: Support;
    let mut do_simplex_res: i32;
    
    *simplex = Simplex::new();
    
    dir = (ccd.first_dir)(obj1, obj2);
    last = support::support(obj1, obj2, &dir, ccd);
    simplex.add(&last);
    
    dir = last.v;
    dir.scale(-1.0);
    
    for _ in 0..ccd.max_iterations {
        last = support::support(obj1, obj2, &dir, ccd);
        
        if last.v.dot(&dir) < 0.0 {
            return -1;
        }
        
        simplex.add(&last);
        
        do_simplex_res = do_simplex(simplex, &mut dir);
        if do_simplex_res == 1 {
            return 0;
        } else if do_simplex_res == -1 {
            return -1;
        }
        
        if dir.len2().is_zero_eps() {
            return -1;
        }
    }
    
    -1
}

pub fn gjk_intersect<T>(obj1: &T, obj2: &T, ccd: &CCD<T>) -> bool {
    let mut simplex = Simplex::new();
    gjk(obj1, obj2, ccd, &mut simplex) == 0
}
