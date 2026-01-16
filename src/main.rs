//! A simple 3D scene with light shining over a cube sitting on a plane.

mod chunk;
mod block;
mod world;
mod thread_manager;

use std::f32::consts::PI;

use bevy::prelude::*;
use chunk::Chunk;
use world::WORLD;


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // circular base
    commands.spawn((
        Mesh3d(meshes.add(Circle::new(4.0))),
        MeshMaterial3d(materials.add(Color::WHITE)),
        Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
    ));

    // a person
    commands.spawn((
        SceneRoot(asset_server.load("people/woman.gltf#Scene0")),
        Transform::from_xyz(0.0, 8.0, 0.0),
        //Transform::from_rotation(Quat::from_rotation_y(PI)),
    ));
    // light
    commands.spawn((
        PointLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 14.0, 4.0),
    ));
    // camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(-2.5, 12.0, 9.0).looking_at(Vec3::new(0.0, 7.0, 0.0), Vec3::Y),
    ));
    commands.spawn_batch(spawn_initial_chunks(asset_server));
}

fn spawn_initial_chunks(asset_server: Res<AssetServer>) -> Vec<(SceneRoot, Transform)>{
    let mut bundles: Vec<(SceneRoot, Transform)> = Vec::new();
    WORLD.iter_mut().for_each(|mut chunkData| {
        let x = chunkData.key().0;
        let y = chunkData.key().1;
        let chunk = chunkData.value_mut();
        bundles.extend(chunk.get_scene_bundle(&asset_server, x, y));
    });
    bundles
}
