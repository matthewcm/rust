use macroquad::prelude::*;

pub fn resolve_collision(a: &mut Rect, b: &Rect, vel: &mut Vec2) -> bool {
    let intersection = match a.intersect(*b) {
        Some(intersection) => intersection,
        None => return false,
    };

    let a_center = a.point() + a.point() * 0.5f32;
    let b_center = b.point() + b.point() * 0.5f32;

    let to = b_center - a_center;
    let to_signum = to.signum();

    match intersection.w > intersection.h {
        true => {
            a.y -= to_signum.y * intersection.h;
            vel.y = -to_signum.y * vel.y.abs();
        }
        false => {
            a.x -= to_signum.x * intersection.h;
            vel.x = -to_signum.x * vel.x.abs();
        }
    }
    true
}
