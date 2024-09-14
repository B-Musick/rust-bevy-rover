use bevy::prelude::*;

use crate::{
    asset_loader::SceneAssets,
    movement::{Acceleration, MovingObjectBundle, Velocity}
};

const STARTING_TRANSLATION: Vec3 = Vec3::new(0.0, 0.0, 0.0);
const ROVER_SPEED: f32 = 25.0;
const ROVER_ROTATION_SPEED: f32 = 2.5;
const ROVER_ROLL_SPEED: f32 = 2.5;

#[derive(Component, Debug)]
pub struct Rover;

pub struct RoverPlugin;

impl Plugin for RoverPlugin {
    // Need to call impl when defining method within
    fn build(&self, app: &mut App) {
        // Need to call &self since we are referencing the Plugin and we dont want to take ownership so we use &
        app.add_systems(PostStartup, spawn_rover).add_systems(
            // HAve to add PostStartup so that the assets load first
            Update,
            rover_movement_controls
        );
    }
}

fn spawn_rover(mut commands: Commands, scene_assets: Res<SceneAssets>) {
    commands.spawn((
        MovingObjectBundle {
            velocity: Velocity::new(Vec3::ZERO),
            acceleration: Acceleration::new(Vec3::ZERO),
            model: SceneBundle {
                scene: scene_assets.rover.clone(),
                transform: Transform::from_translation(STARTING_TRANSLATION),
                ..default()
            },
        },
        Rover
    ));
}

fn rover_movement_controls(
    // Query for rover entity instead of everything with transform or velocity
    // Need to access keyboard controls
    // Query Filter = With argument - useful when want to query component but not the data. Just want to query entity 
    // Query - can put two arguments, each with tuples
    mut query: Query<(&mut Transform, &mut Velocity), With<Rover>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    // single_mut - single entity associated with a component
    // will panic if not one copy
    let (mut transform, mut velocity) = query.single_mut();
    let mut rotation = 0.0;
    let mut roll = 0.0;
    let mut movement = 0.0;

    if keyboard_input.pressed(KeyCode::KeyA) {
        rotation = ROVER_ROTATION_SPEED * time.delta_seconds();
    } else if keyboard_input.pressed(KeyCode::KeyD) {
        rotation = -ROVER_ROTATION_SPEED * time.delta_seconds();
    }

    if keyboard_input.pressed(KeyCode::KeyS) {
        movement = -ROVER_SPEED;
    } else if keyboard_input.pressed(KeyCode::KeyW) {
        movement = ROVER_SPEED;
    }

    // Rotate around the Y-axis.
    // Ignores the Z-axis rotation applied below.
    transform.rotate_y(rotation);

    // Rotate around the local Z-axis.
    // The rotation is relative to the current rotation!
    transform.rotate_local_z(roll);

    // Update the spaceship's velocity based on new direction.
    // Bevy considers forward in -Z direction
    // Can change this in Blender if use
    velocity.value = -transform.forward() * movement;
}