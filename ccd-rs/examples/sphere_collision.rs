use ccd::{Vec3, CCD, gjk_intersect, mpr_intersect};

struct Sphere {
    center: Vec3,
    radius: f64,
}

fn sphere_support(sphere: &Sphere, dir: &Vec3) -> Vec3 {
    let mut d = *dir;
    d.normalize();
    sphere.center + d * sphere.radius
}

fn sphere_center(sphere: &Sphere) -> Vec3 {
    sphere.center
}

fn main() {
    let sphere1 = Sphere {
        center: Vec3::new(0.0, 0.0, 0.0),
        radius: 1.0,
    };
    
    let sphere2 = Sphere {
        center: Vec3::new(1.5, 0.0, 0.0),
        radius: 1.0,
    };
    
    let sphere3 = Sphere {
        center: Vec3::new(5.0, 0.0, 0.0),
        radius: 1.0,
    };

    let mut ccd: CCD<Sphere> = CCD::default();
    ccd.support1 = sphere_support;
    ccd.support2 = sphere_support;
    ccd.center1 = sphere_center;
    ccd.center2 = sphere_center;

    let gjk_intersecting = gjk_intersect(&sphere1, &sphere2, &ccd);
    let gjk_separated = gjk_intersect(&sphere1, &sphere3, &ccd);
    
    let mpr_intersecting = mpr_intersect(&sphere1, &sphere2, &ccd);
    let mpr_separated = mpr_intersect(&sphere1, &sphere3, &ccd);

    println!("GJK - Sphere1 & Sphere2 (intersecting): {}", gjk_intersecting);
    println!("GJK - Sphere1 & Sphere3 (separated): {}", gjk_separated);
    println!("MPR - Sphere1 & Sphere2 (intersecting): {}", mpr_intersecting);
    println!("MPR - Sphere1 & Sphere3 (separated): {}", mpr_separated);
}
