use bevy::prelude::*;
use plugins::*;

mod components;
mod messages;
mod plugins;
mod resources;
mod systems;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(GameStatePlugin)
        .add_plugins(TimersPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(EnemyPlugin)
        .add_plugins(StarPlugin)
        .run();
}
