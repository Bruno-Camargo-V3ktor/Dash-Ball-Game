use bevy::prelude::*;

mod game;
mod main_menu;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(game::GamePlugin)
        .add_plugins(main_menu::MainMenuPlugin)
        .run();
}
