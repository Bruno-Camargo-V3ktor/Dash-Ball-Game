use super::systems::main_menu_state::*;
use crate::AppState;
use bevy::prelude::*;

pub struct MainMenuStatePlugin;
impl Plugin for MainMenuStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            transition_to_game.run_if(in_state(AppState::MainMenu)),
        );
    }
}
