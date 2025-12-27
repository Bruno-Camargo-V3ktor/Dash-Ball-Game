use bevy::prelude::*;
use messages::game_states::*;
use resources::{score::*, timers::*};
use systems::{camera::*, enemy::*, game_state::*, player::*, star::*, timers::*};

mod components;
mod messages;
mod resources;
mod systems;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<Score>()
        .init_resource::<HighScores>()
        .init_resource::<StarSpawnTimer>()
        .init_resource::<EnemySpawnTimer>()
        .add_message::<GameOver>()
        .add_systems(
            Startup,
            (spawn_camera, spawn_player, spawn_enemies, spawn_stars).chain(),
        )
        .add_systems(
            Update,
            (
                exit_game,
                handle_game_over,
                tick_star_spawn_timer,
                spawn_stars_over_time,
                camera_position,
                player_movement,
                player_hit_star,
                confine_player,
                confine_enemy,
                update_enemy_direction,
                enemy_movement,
                enemy_hit_player,
                tick_enemy_spawn_timer,
                spawn_enemys_over_time,
                update_high_scores,
            )
                .chain(),
        )
        .run();
}
