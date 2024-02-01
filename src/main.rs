mod asset_loader;
mod asteroids;
mod camera;
mod collision_detection;
mod debug;
mod despawn;
mod light;
mod movement;
mod spaceship;

use asset_loader::AssetLoaderPlugin;
use asteroids::AsteroidsPlugin;
use bevy::{app::App, DefaultPlugins};

use camera::CameraPlugin;
use collision_detection::CollisionDetectionPlugin;
use debug::DebugPlugin;
use despawn::DespawnPlugin;
use light::LightPlugin;
use movement::MovementPlugin;
use spaceship::SpaceshipPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(AssetLoaderPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(LightPlugin)
        .add_plugins(SpaceshipPlugin)
        .add_plugins(AsteroidsPlugin)
        .add_plugins(MovementPlugin)
        .add_plugins(CollisionDetectionPlugin)
        .add_plugins(DespawnPlugin)
        .add_plugins(DebugPlugin)
        .run()
}
