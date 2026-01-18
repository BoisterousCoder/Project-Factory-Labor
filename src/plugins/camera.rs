use bevy::input::mouse::AccumulatedMouseMotion;
use bevy::input::mouse::AccumulatedMouseScroll;
use bevy::prelude::*;

use crate::utils::{polar_to_cartesian, rotate_in_square_steps};
use std::f32::consts::PI;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_camera)
            .add_systems(Update, camera_controls);
    }
}

#[derive(Component)]
struct MainCamera;

#[derive(Component)]
struct Cursor;

#[derive(Resource)]
struct CameraSettings {
    pub distance: f32,
    pub looking_at: Vec3,
    pub angle: Vec2,
    pub sensitivity: f32,
    pub move_speed: f32,
    pub smoothness: f32,
    pub scroll_sensitivity: f32
}
impl CameraSettings {
    pub fn get_position(&self) -> Vec3 {
        polar_to_cartesian(self.distance, self.angle) + self.looking_at
    }
}

fn setup_camera(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    let init_camera_settings = CameraSettings {
        distance: 10.0,
        looking_at: Vec3::new(0.0, 8.0, 0.0),
        angle: Vec2::new(-PI / 4.0, PI / 4.0), // Initial angles (yaw, pitch)
        sensitivity: 0.06,
        move_speed: 5.0,
        smoothness: 5.0,
        scroll_sensitivity: 1.5
    };

    let cam_pos = init_camera_settings.get_position();
    let looking_at = init_camera_settings.looking_at;

    // camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(cam_pos.x, cam_pos.y, cam_pos.z)
            .looking_at(looking_at, Vec3::Y),
        MainCamera,
    ));

    commands.spawn((
        SceneRoot(asset_server.load("Cursor.gltf#Scene0")),
        Transform::from_translation(looking_at),
        Cursor,
    ));

    commands.insert_resource(init_camera_settings);
}

fn camera_controls(
    mut camera_query: Query<&mut Transform, (With<MainCamera>, Without<Cursor>)>,
    mut cursor_query: Query<&mut Transform, (With<Cursor>, Without<MainCamera>)>,
    mouse_motion: Res<AccumulatedMouseMotion>,
    mouse_button: Res<ButtonInput<MouseButton>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut camera_settings: ResMut<CameraSettings>,
    accumulated_scroll: Res<AccumulatedMouseScroll>,
    time: Res<Time>,
) {
    let mut camera_transform: Mut<'_, Transform>;
    let mut cursor_transform: Mut<'_, Transform>;

    //Get the transformations for the camera and cursor
    if let Ok(_camera_transform) = camera_query.single_mut(){
        camera_transform = _camera_transform;
    }else{
        panic!("No camera found! Send Help!");
    }
    if let Ok(_cursor_transform) = cursor_query.single_mut(){
        cursor_transform = _cursor_transform;
    }else{
        panic!("No cursor found! Send Help!");
    }

    // Rotate Camera around point
    let sensitivity_step = camera_settings.sensitivity * time.delta_secs();
    if mouse_button.pressed(MouseButton::Middle) {
        let delta = mouse_motion.delta;
        let sensitivity = 0.002;

        // Yaw (Horizontal rotation around the global Y axis)
        camera_settings.angle.x += delta.x * sensitivity_step;

        // Pitch (Vertical rotation around the local X axis)
        camera_settings.angle.y += delta.y * sensitivity_step;
    }

    // Move target Point
    let move_step = camera_settings.move_speed * time.delta_secs();
    let mut move_dir = Vec3::ZERO;
    if keyboard_input.pressed(KeyCode::KeyW) { move_dir.z -= 1.0; }
    if keyboard_input.pressed(KeyCode::KeyS) { move_dir.z += 1.0; }
    if keyboard_input.pressed(KeyCode::KeyA) { move_dir.x -= 1.0; }
    if keyboard_input.pressed(KeyCode::KeyD) { move_dir.x += 1.0; }
    if keyboard_input.pressed(KeyCode::KeyQ) { move_dir.y += 1.0; }
    if keyboard_input.pressed(KeyCode::KeyE) { move_dir.y -= 1.0; }

    move_dir *= move_step;

    //Get the direction the camera is looking in to adjust the movement direction accordingly
    let camera_rotate_steps = ((camera_settings.angle.x % (PI*2.0))/(PI*0.25)).round() as i32;
    move_dir = rotate_in_square_steps(move_dir, camera_rotate_steps);

    //move the look at point
    camera_settings.looking_at += move_dir;

    // Zoom in/out
    camera_settings.distance -= accumulated_scroll.delta.y * camera_settings.scroll_sensitivity * time.delta_secs();

    // Update Camera based on new settings
    let lerp_factor = (time.delta_secs() * camera_settings.smoothness).min(1.0);
    camera_transform.translation = camera_transform.translation.lerp(camera_settings.get_position(), lerp_factor);

    let target_rotation = camera_transform.looking_at(camera_settings.looking_at, Vec3::Y).rotation;
    camera_transform.rotation = camera_transform.rotation.slerp(target_rotation, lerp_factor);
    
    cursor_transform.translation = camera_settings.looking_at;
}
