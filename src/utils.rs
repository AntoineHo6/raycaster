use std::f32::consts::PI;

pub fn rot_angle(angle: f32, rotation: f32) -> f32 {
    let mut new_angle: f32 = angle + rotation;

    if new_angle > 2. * PI {
        new_angle -= 2. * PI;
    } else if new_angle < 0. {
        new_angle += 2. * PI;
    }

    return new_angle;
}