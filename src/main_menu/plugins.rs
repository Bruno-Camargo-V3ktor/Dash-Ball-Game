use super::systems::main_menu_state::*;
use crate::{
    AppState,
    main_menu::systems::{
        interaction::{interact_with_play_button, interact_with_quit_button},
        layout::{despawn_main_menu, spawn_main_menu},
    },
};
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

pub struct LayoutPlugin;
impl Plugin for LayoutPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::MainMenu), spawn_main_menu)
            .add_systems(OnExit(AppState::MainMenu), despawn_main_menu);
    }
}

pub struct InteractionPlugin;
impl Plugin for InteractionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (interact_with_play_button, interact_with_quit_button)
                .run_if(in_state(AppState::MainMenu)),
        );
    }
}
