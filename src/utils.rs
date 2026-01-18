use bevy::prelude::Vec3;
use bevy::prelude::Vec2;
pub fn polar_to_cartesian(distance: f32, angle: Vec2) -> Vec3 {
    let x: f32 = distance * angle.x.cos() * angle.y.cos();
    let y: f32 = distance * angle.y.cos();
    let z: f32 = distance * angle.x.sin() * angle.y.sin();
    Vec3::new(x, y, z)
}
//rotate in multiples of 90 degree steps. This should be the fastest way to do it.
pub fn rotate_in_square_steps(vec: Vec3, _steps: i32) -> Vec3 {
    let steps:u8;
    if _steps > 0 {
        steps = (_steps % 4) as u8;
    }else{
        steps = (4 + (_steps % 4)) as u8;
    }
    match steps {
        1 => Vec3::new(vec.z, vec.y, -vec.x),   // 90 deg
        2 => Vec3::new(-vec.x, vec.y, -vec.z), // 180 deg
        3 => Vec3::new(-vec.z, vec.y, vec.x),  // 270 deg
        _ => vec,                              // 0 deg
    }
}