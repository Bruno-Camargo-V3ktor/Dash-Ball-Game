use bevy::prelude::*;
use plugins::*;

mod plugins;
mod systems;

pub struct MainMenuPlugin;
impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MainMenuStatePlugin)
            .add_systems(Startup, hello_main_menu);
    }
}

pub fn hello_main_menu() {
    println!("Hi, i am menu");
}
