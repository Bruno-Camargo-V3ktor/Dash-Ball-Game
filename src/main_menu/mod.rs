use bevy::prelude::*;
use plugins::*;

mod components;
mod plugins;
mod styles;
mod systems;

pub struct MainMenuPlugin;
impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MainMenuStatePlugin)
            .add_plugins(LayoutPlugin)
            .add_plugins(plugins::InteractionPlugin);
    }
}
