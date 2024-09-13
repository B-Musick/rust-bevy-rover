mod rover;
mod movement;
mod camera;
mod debug;

use bevy::prelude::*;

use movement::MovementPlugin;
use camera::CameraPlugin;
use debug::DebugPlugin;
use rover::RoverPlugin;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::srgb(0.1, 0.0, 0.15)))
        .insert_resource(AmbientLight {
            color: Color::default(),
            brightness: 500.75,
        })
        .add_plugins(DefaultPlugins)
        // User defined plugins.
        .add_plugins(MovementPlugin)
        .add_plugins(DebugPlugin)
        .add_plugins(RoverPlugin)
        .add_plugins(CameraPlugin)
        .run();
}
