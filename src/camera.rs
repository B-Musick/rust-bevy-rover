/**
 * Remember - need a camera otherwise world wont show
 */
use bevy::prelude::*;

const CAMERA_DISTANCE: f32 = 30.0;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera);
    }
}

fn spawn_camera(mut commands: Commands) {
    // Spawn Camera3dBundle entity
    // https://bevy-cheatbook.github.io/3d/camera.html#creating-a-3d-camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 6.0, -16.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}