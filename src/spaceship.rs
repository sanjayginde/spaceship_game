use bevy::prelude::*;

use crate::movement::Velocity;

const STARTING_TRANSLATION: Vec3 = Vec3::new(1., 1., -20.);
const STARTING_VELOCITY: Vec3 = Vec3::new(0., 0., 1.);

#[derive(Bundle)]
struct SpaceshipBundle {
    velocity: Velocity,
    model: SceneBundle,
}

pub struct SpaceshipPlugin;

impl Plugin for SpaceshipPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_spaceship);
    }
}

fn spawn_spaceship(mut commands: Commands, assert_server: Res<AssetServer>) {
    commands.spawn((SpaceshipBundle {
        velocity: Velocity {
            value: STARTING_VELOCITY,
        },
        model: SceneBundle {
            transform: Transform::from_translation(STARTING_TRANSLATION),
            scene: assert_server.load("Spaceship.glb#Scene0"),
            ..default()
        },
    },));
}
