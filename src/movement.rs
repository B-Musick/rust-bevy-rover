// use bevy::prelude::*;

// #[derive(Component, Debug)]
// pub struct Velocity {
//     pub value: Vec3,
// }

// impl Velocity {
//     // Common pattern of a constructor using new
//     // Velocity is the speed in a direction
//     pub fn new(value: Vec3) -> Self {
//         Self { value }
//     }
// }
// // Derive the component trait
// // Rate at which velocity changes = Accelleration
// #[derive(Component, Debug)]
// pub struct Acceleration {
//     pub value: Vec3,
// }

// impl Acceleration {
//     pub fn new(value: Vec3) -> Self {
//         Self { value }
//     }
// }
// // Name it moving object bundle so can use accross all objects (asteroids, spaceship, etc)
// #[derive(Bundle)]
// pub struct MovingObjectBundle {
//     // MAke public so can use across multiple files
//     pub velocity: Velocity,
//     pub acceleration: Acceleration,
//     pub model: SceneBundle,
// }

// pub struct MovementPlugin;

// impl Plugin for MovementPlugin {
//     fn build(&self, app: &mut App) {
//         // Add this to plugin so app runs every frame
//         app.add_systems(Update, (update_velocity, update_position));
//     }
// }
// // Get time resourece so can know the time delta
// // update velocity value by adding accelleration and multiply by delta so fps doesnt affect it. 
// fn update_velocity(mut query: Query<(&Acceleration, &mut Velocity)>, time: Res<Time>) {
//     for (acceleration, mut velocity) in query.iter_mut() {
//         velocity.value += acceleration.value * time.delta_seconds();
//     }
// }

// fn update_position(mut query: Query<(&Velocity, &mut Transform)>, time: Res<Time>) {
//     for (velocity, mut transform) in query.iter_mut() {
//         transform.translation += velocity.value * time.delta_seconds();
//     }
// }

// Delete after
use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct Velocity {
    pub value: Vec3,
}

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_position);
    }
}

fn update_position(mut query: Query<(&Velocity, &mut Transform)>, time: Res<Time>) {
    for (velocity, mut transform) in query.iter_mut() {
        transform.translation += velocity.value * time.delta_seconds();
    }
}