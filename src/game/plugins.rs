use crate::AppState;

use super::{
    messages::game_states::GameOver,
    resources::{
        score::{HighScores, Score},
        timers::{EnemySpawnTimer, StarSpawnTimer},
    },
    systems::{camera::*, enemy::*, game_state::*, player::*, star::*, timers::*},
};
use bevy::prelude::*;

pub struct EnemyPlugin;
impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<EnemySpawnTimer>()
            .add_systems(Startup, spawn_enemies)
            .add_systems(
                Update,
                (
                    confine_enemy,
                    enemy_movement,
                    update_enemy_direction,
                    enemy_hit_player,
                    spawn_enemys_over_time,
                )
                    .chain(),
            );
    }
}

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(Update, player_hit_star)
            .configure_sets(
                Update,
                (PlayerStateSet::Movement, PlayerStateSet::Confine).chain(),
            )
            .add_systems(Update, player_movement.in_set(PlayerStateSet::Movement))
            .add_systems(Update, confine_player.in_set(PlayerStateSet::Confine));
    }
}

pub struct CameraPlugin;
impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera)
            .add_systems(Update, camera_position);
    }
}

pub struct StarPlugin;
impl Plugin for StarPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<StarSpawnTimer>()
            .add_systems(Startup, spawn_stars)
            .add_systems(Update, spawn_stars_over_time);
    }
}

pub struct TimersPlugin;
impl Plugin for TimersPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (tick_star_spawn_timer, tick_enemy_spawn_timer).chain(),
        );
    }
}

pub struct GameStatePlugin;
impl Plugin for GameStatePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Score>()
            .init_resource::<HighScores>()
            .add_message::<GameOver>()
            .add_systems(
                Update,
                (
                    exit_game,
                    handle_game_over,
                    update_high_scores,
                    toggle_game_simulation.run_if(in_state(AppState::Game)),
                    transition_to_main_menu.run_if(in_state(AppState::Game)),
                ),
            );
    }
}
