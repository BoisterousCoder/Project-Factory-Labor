//! A simple 3D scene with light shining over a cube sitting on a plane.

mod chunk;
mod block;
mod world;
mod plugins;
mod utils;

use crate::plugins::camera::CameraPlugin;
use bevy::prelude::*;
use world::WORLD;


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(CameraPlugin)
        .add_systems(Startup, setup)
        .run();
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) { 
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

    commands.spawn_batch(spawn_initial_chunks(asset_server));
}

fn spawn_initial_chunks(asset_server: Res<AssetServer>) -> Vec<(SceneRoot, Transform)>{
    let mut bundles: Vec<(SceneRoot, Transform)> = Vec::new();
    WORLD.iter_mut().for_each(|mut chunk_data| {
        let x = chunk_data.key().0;
        let y = chunk_data.key().1;
        let chunk = chunk_data.value_mut();
        bundles.extend(chunk.get_scene_bundle(&asset_server, x, y));
    });
    bundles
}