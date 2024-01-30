mod camera;
mod debug;
mod light;
mod movement;
mod spaceship;

use bevy::{app::App, DefaultPlugins};

use camera::CameraPlugin;
use debug::DebugPlugin;
use light::LightPlugin;
use movement::MovementPlugin;
use spaceship::SpaceshipPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(CameraPlugin)
        .add_plugins(LightPlugin)
        .add_plugins(SpaceshipPlugin)
        .add_plugins(MovementPlugin)
        .add_plugins(DebugPlugin)
        .run()
}
