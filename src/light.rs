use bevy::prelude::*;

const BACKGROUND_COLOR: Color = Color::rgb(0.1, 0.0, 0.15);
const LIGHT_BRIGHTNESS: f32 = 0.75;

pub struct LightPlugin;

impl Plugin for LightPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(BACKGROUND_COLOR))
            .insert_resource(AmbientLight {
                color: Color::default(),
                brightness: LIGHT_BRIGHTNESS,
            });
    }
}
