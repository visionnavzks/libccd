# libccd-rs Tutorial

This document provides a tutorial on the algorithms implemented in `libccd-rs` (a pure Rust port of `libccd`), namely GJK (Gilbert-Johnson-Keerthi) and MPR (Minkowski Portal Refinement), and how to use them.

## 1. The GJK Algorithm

The Gilbert-Johnson-Keerthi (GJK) algorithm determines whether two convex shapes intersect.

### How it works:
Instead of checking for intersection between Shape A and Shape B directly, GJK computes their **Minkowski Difference** ($A \ominus B$).
The intersection of Shape A and Shape B occurs if and only if their Minkowski Difference contains the origin $(0,0,0)$.

GJK does not construct the entire Minkowski Difference (which would be computationally expensive). Instead, it iteratively builds a **simplex** (a point, line segment, triangle, or tetrahedron) inside the Minkowski Difference, trying to enclose the origin.
It uses a **Support Function** to search for extreme points in given directions. If the support point in the direction of the origin does not go past the origin, the shapes do not intersect. Otherwise, it updates the simplex and continues until it either encloses the origin (Intersection) or proves it cannot (No Intersection).

### Usage in libccd-rs:
To use GJK in this library, you need to implement a support function for your shapes.

```rust
use libccd_rs::ccd::{ccd_t, ccdGJKIntersect, ccd_vec3_t, ccdFirstDirDefault};
use std::ffi::c_void;

// Define your shape
struct Circle { pos: [f32; 3], radius: f32 }

// Implement the support function
unsafe extern "C" fn support_circle(obj: *const c_void, dir: *const ccd_vec3_t, vec: *mut ccd_vec3_t) {
    // ... Calculate extreme point of circle in `dir` and store in `vec` ...
}

// Set up the ccd_t struct and test
let mut ccd = ccd_t {
    first_dir: Some(ccdFirstDirDefault),
    support1: Some(support_circle),
    support2: Some(support_circle),
    center1: None, // Only needed for MPR
    center2: None,
    max_iterations: 100,
    epa_tolerance: 0.0001,
    mpr_tolerance: 0.0001,
    dist_tolerance: 1e-6,
};

let intersects = unsafe {
    ccdGJKIntersect(&circle1 as *const _ as _, &circle2 as *const _ as _, &ccd as *const _)
};
```

## 2. The MPR Algorithm (Minkowski Portal Refinement)

MPR (also known as XenoCollide) is an alternative to GJK and EPA (Expanding Polytope Algorithm). Like GJK, it uses the Minkowski Difference, but its approach to finding the origin is different.

### How it works:
MPR starts by finding the center of the Minkowski Difference (by subtracting the geometric centers of Shape B and Shape A). It creates a ray from this center to the origin $(0,0,0)$.
It then builds a "portal" (a triangle) on the surface of the Minkowski Difference that the ray passes through. MPR refines this portal iteratively, pulling it closer to the surface of the Minkowski shape.
If the origin ends up inside the shape (behind the portal), it's an intersection.

MPR is highly robust, especially in 3D, and generally simpler to implement than GJK+EPA when calculating penetration depth, as it does not suffer from the same numerical instability edge cases as EPA.

### Usage in libccd-rs:
Using MPR is similar to GJK, but requires providing a function to compute the geometric center of your shapes.

```rust
use libccd_rs::ccd::ccdMPRIntersect;

unsafe extern "C" fn center_circle(obj: *const c_void, center: *mut ccd_vec3_t) {
    let circle = &*(obj as *const Circle);
    (*center).v = circle.pos;
}

// In the ccd_t initialization, ensure `center1` and `center2` are populated:
ccd.center1 = Some(center_circle);
ccd.center2 = Some(center_circle);

let intersects = unsafe {
    ccdMPRIntersect(&circle1 as *const _ as _, &circle2 as *const _ as _, &ccd as *const _)
};
```

### Penetration Depth
You can also get the penetration depth vector and position using `ccdGJKPenetration` (GJK + EPA) or `ccdMPRPenetration` (MPR) by supplying output variable pointers.
