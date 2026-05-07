use macroquad::prelude::*;
use ccd::{Vec3, CCD, gjk_intersect};

struct Ball {
    pos: Vec2,
    radius: f32,
}

fn ball_support(ball: &Ball, dir: &Vec3) -> Vec3 {
    let mut d = Vec3::new(dir.x(), dir.y(), 0.0);
    d.normalize();
    Vec3::new(
        ball.pos.x as f64 + d.x() * ball.radius as f64,
        ball.pos.y as f64 + d.y() * ball.radius as f64,
        0.0,
    )
}

fn ball_center(ball: &Ball) -> Vec3 {
    Vec3::new(ball.pos.x as f64, ball.pos.y as f64, 0.0)
}

#[macroquad::main("CCD Collision Detection Demo")]
async fn main() {
    let mut balls: Vec<Ball> = vec![
        Ball { pos: Vec2::new(200.0, 300.0), radius: 60.0 },
        Ball { pos: Vec2::new(400.0, 300.0), radius: 60.0 },
    ];
    let mut dragging_idx: Option<usize> = None;
    let mut collision_result = false;

    let mut ccd: CCD<Ball> = CCD::default();
    ccd.support1 = ball_support;
    ccd.support2 = ball_support;
    ccd.center1 = ball_center;
    ccd.center2 = ball_center;

    loop {
        let mouse = mouse_position();
        let ball1 = &balls[0];
        let ball2 = &balls[1];

        if is_mouse_button_down(MouseButton::Left) {
            let b1x = ball1.pos.x;
            let b1y = ball1.pos.y;
            let b1r = ball1.radius;
            let b2x = ball2.pos.x;
            let b2y = ball2.pos.y;
            let b2r = ball2.radius;

            let d1 = (mouse.0 - b1x).powi(2) + (mouse.1 - b1y).powi(2);
            let d2 = (mouse.0 - b2x).powi(2) + (mouse.1 - b2y).powi(2);

            if dragging_idx.is_none() {
                if d1 < b1r.powi(2) {
                    dragging_idx = Some(0);
                } else if d2 < b2r.powi(2) {
                    dragging_idx = Some(1);
                }
            }
        } else {
            dragging_idx = None;
        }

        if let Some(idx) = dragging_idx {
            balls[idx].pos.x = mouse.0;
            balls[idx].pos.y = mouse.1;
        }

        collision_result = gjk_intersect(&balls[0], &balls[1], &ccd);

        clear_background(Color::from_hex(0x1a1a2e));

        draw_circle(balls[0].pos.x, balls[0].pos.y, balls[0].radius, Color::from_hex(0x4a90d9));
        draw_circle_lines(balls[0].pos.x, balls[0].pos.y, balls[0].radius, 2.0, WHITE);
        
        draw_circle(balls[1].pos.x, balls[1].pos.y, balls[1].radius, Color::from_hex(0xd94a4a));
        draw_circle_lines(balls[1].pos.x, balls[1].pos.y, balls[1].radius, 2.0, WHITE);

        if collision_result {
            let mid_x = (balls[0].pos.x + balls[1].pos.x) / 2.0;
            let mid_y = (balls[0].pos.y + balls[1].pos.y) / 2.0;
            draw_circle(mid_x, mid_y, 15.0, Color::from_hex(0xffd700));
        }

        let status_text = if collision_result { "COLLISION" } else { "NO COLLISION" };
        let status_color = if collision_result {
            Color::from_hex(0x00ff00)
        } else {
            Color::from_hex(0xff4444)
        };

        draw_text(status_text, 300.0, 50.0, 32.0, status_color);

        draw_text("Drag balls with mouse", 250.0, 550.0, 18.0, Color::from_hex(0x888888));

        next_frame().await;
    }
}
