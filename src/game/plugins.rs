use super::{
    messages::game_states::GameOver,
    resources::{
        score::{HighScores, Score},
        timers::{EnemySpawnTimer, StarSpawnTimer},
    },
    systems::{camera::*, enemy::*, game_state::*, player::*, star::*, timers::*, ui::*},
};
use crate::{AppState, game::states::SimulationState};
use bevy::prelude::*;

pub struct EnemyPlugin;
impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<EnemySpawnTimer>()
            .add_systems(OnEnter(AppState::Game), spawn_enemies)
            .add_systems(
                Update,
                (
                    confine_enemy,
                    enemy_movement,
                    update_enemy_direction,
                    enemy_hit_player,
                    spawn_enemys_over_time,
                )
                    .chain()
                    .run_if(in_state(AppState::Game))
                    .run_if(in_state(SimulationState::GameRunning)),
            )
            .add_systems(OnExit(AppState::Game), despawn_enemys);
    }
}

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(
            Update,
            (PlayerStateSet::Movement, PlayerStateSet::Confine).chain(),
        )
        .add_systems(OnEnter(AppState::Game), spawn_player)
        .add_systems(
            Update,
            (
                player_hit_star,
                player_movement.in_set(PlayerStateSet::Movement),
                confine_player.in_set(PlayerStateSet::Confine),
            )
                .run_if(in_state(AppState::Game))
                .run_if(in_state(SimulationState::GameRunning)),
        )
        .add_systems(OnExit(AppState::Game), despawn_player);
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
            .add_systems(OnEnter(AppState::Game), spawn_stars)
            .add_systems(
                Update,
                spawn_stars_over_time
                    .run_if(in_state(AppState::Game))
                    .run_if(in_state(SimulationState::GameRunning)),
            )
            .add_systems(OnExit(AppState::Game), despawn_stars);
    }
}

pub struct TimersPlugin;
impl Plugin for TimersPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (tick_star_spawn_timer, tick_enemy_spawn_timer)
                .chain()
                .run_if(in_state(AppState::Game))
                .run_if(in_state(SimulationState::GameRunning)),
        );
    }
}

pub struct GameStatePlugin;
impl Plugin for GameStatePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Score>()
            .init_resource::<HighScores>()
            .add_message::<GameOver>()
            .add_systems(OnEnter(AppState::Game), insert_score)
            .add_systems(
                Update,
                (
                    exit_game,
                    handle_game_over,
                    update_high_scores,
                    toggle_game_simulation,
                )
                    .run_if(in_state(AppState::Game)),
            )
            .add_systems(Update, transition_to_main_menu)
            .add_systems(OnExit(AppState::Game), remove_score);
    }
}

pub struct UIPlugin;
impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Game), hud::spawn_hud)
            .add_systems(OnExit(AppState::Game), hud::despawn_hud)
            .add_systems(
                OnEnter(SimulationState::GamePaused),
                pause_menu::spawn_pause_menu,
            )
            .add_systems(
                OnExit(SimulationState::GamePaused),
                pause_menu::despawn_pause_menu,
            )
            .add_systems(OnEnter(AppState::GameOver), gameover::spawn_gameover_menu)
            .add_systems(OnExit(AppState::GameOver), gameover::despawn_gameover_menu)
            .add_systems(
                Update,
                (hud::update_score_text, hud::update_enemys_text).run_if(in_state(AppState::Game)),
            )
            .add_systems(
                Update,
                (
                    pause_menu::interact_with_resume_button,
                    pause_menu::interact_with_main_menu_button,
                    pause_menu::interact_with_quit_button,
                )
                    .run_if(in_state(SimulationState::GamePaused)),
            )
            .add_systems(
                Update,
                (
                    gameover::interact_with_restart_button,
                    gameover::interact_with_main_menu_button,
                    gameover::interact_with_quit_button,
                )
                    .run_if(in_state(AppState::GameOver)),
            );
    }
}
