use libccd_rs::ccd::{ccd_t, ccdGJKIntersect, ccd_vec3_t, ccdFirstDirDefault};
use macroquad::prelude::*;
use std::ffi::c_void;

#[repr(C)]
struct Circle {
    pos: [f32; 3],
    radius: f32,
}

unsafe extern "C" fn support_circle(obj: *const c_void, dir: *const ccd_vec3_t, vec: *mut ccd_vec3_t) { unsafe {
    let circle = &*(obj as *const Circle);
    let dir_vec = &*dir;

    let len = (dir_vec.v[0]*dir_vec.v[0] + dir_vec.v[1]*dir_vec.v[1] + dir_vec.v[2]*dir_vec.v[2]).sqrt();

    if len > 0.0 {
        let n0 = dir_vec.v[0] / len;
        let n1 = dir_vec.v[1] / len;
        let n2 = dir_vec.v[2] / len;

        (*vec).v[0] = circle.pos[0] + n0 * circle.radius;
        (*vec).v[1] = circle.pos[1] + n1 * circle.radius;
        (*vec).v[2] = circle.pos[2] + n2 * circle.radius;
    } else {
        (*vec).v[0] = circle.pos[0];
        (*vec).v[1] = circle.pos[1];
        (*vec).v[2] = circle.pos[2];
    }
}}

unsafe extern "C" fn center_circle(obj: *const c_void, center: *mut ccd_vec3_t) { unsafe {
    let circle = &*(obj as *const Circle);
    (*center).v[0] = circle.pos[0];
    (*center).v[1] = circle.pos[1];
    (*center).v[2] = circle.pos[2];
}}

#[macroquad::main("LibCCD-RS Demo")]
async fn main() {
    let mut circle1 = Circle {
        pos: [200.0, 200.0, 0.0],
        radius: 50.0,
    };

    let circle2 = Circle {
        pos: [400.0, 200.0, 0.0],
        radius: 80.0,
    };

    loop {
        clear_background(LIGHTGRAY);

        if is_mouse_button_down(MouseButton::Left) {
            let (mx, my) = mouse_position();
            circle1.pos[0] = mx;
            circle1.pos[1] = my;
        }

        let ccd = ccd_t {
            first_dir: Some(ccdFirstDirDefault),
            support1: Some(support_circle),
            support2: Some(support_circle),
            center1: Some(center_circle),
            center2: Some(center_circle),
            max_iterations: 100,
            epa_tolerance: 0.0001,
            mpr_tolerance: 0.0001,
            dist_tolerance: 1e-6,
        };

        let intersect = unsafe {
            ccdGJKIntersect(
                &circle1 as *const _ as *const c_void,
                &circle2 as *const _ as *const c_void,
                &ccd as *const _,
            )
        };

        let color1 = if intersect != 0 { RED } else { BLUE };
        let color2 = if intersect != 0 { RED } else { GREEN };

        draw_circle(circle1.pos[0], circle1.pos[1], circle1.radius, color1);
        draw_circle(circle2.pos[0], circle2.pos[1], circle2.radius, color2);

        draw_text(
            "Drag circle with left mouse button",
            10.0,
            20.0,
            20.0,
            BLACK,
        );

        draw_text(
            &format!("Intersecting: {}", intersect != 0),
            10.0,
            50.0,
            20.0,
            if intersect != 0 { RED } else { BLACK },
        );

        next_frame().await
    }
}
