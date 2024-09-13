use bevy::prelude::*;

// use crate::movement::{Acceleration, MovingObjectBundle, Velocity};
// Remove
use crate::movement::Velocity;
// To hear
const STARTING_TRANSLATION: Vec3 = Vec3::new(0.0, 0.0, 0.0);

// Remove
const STARTING_VELOCITY: Vec3 = Vec3::new(0.0, 0.0, 1.0);
#[derive(Bundle)]
struct RoverBundle {
    velocity: Velocity,
    model: SceneBundle,
}
// Remove to here

#[derive(Component, Debug)]
pub struct Rover;

pub struct RoverPlugin;

impl Plugin for RoverPlugin {
    // Need to call impl when defining method within
    fn build(&self, app: &mut App) {
        // Need to call &self since we are referencing the Plugin and we dont want to take ownership so we use &
        app.add_systems(Startup, spawn_rover);
    }
}

fn spawn_rover(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(
        RoverBundle {
            velocity: Velocity {
                value: STARTING_VELOCITY,
            },
            model: SceneBundle {
                scene: asset_server.load("Rover.glb#Scene0"),
                transform: Transform::from_translation(STARTING_TRANSLATION),
                ..default()
            },
        }
    );
}