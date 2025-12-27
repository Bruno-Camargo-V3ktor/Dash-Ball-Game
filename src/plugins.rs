use crate::{
    resources::timers::EnemySpawnTimer,
    systems::{
        camera::{camera_position, spawn_camera},
        enemy::{
            confine_enemy, enemy_hit_player, enemy_movement, spawn_enemies, spawn_enemys_over_time,
            update_enemy_direction,
        },
        player::{confine_player, player_hit_star, player_movement, spawn_player},
    },
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
        app.add_systems(Startup, spawn_player).add_systems(
            Update,
            (player_movement, player_hit_star, confine_player).chain(),
        );
    }
}

pub struct CameraPlugin;
impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera)
            .add_systems(Update, camera_position);
    }
}
