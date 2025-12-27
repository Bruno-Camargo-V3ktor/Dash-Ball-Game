use crate::{
    resources::timers::EnemySpawnTimer,
    systems::enemy::{
        confine_enemy, enemy_hit_player, enemy_movement, spawn_enemies, spawn_enemys_over_time,
        update_enemy_direction,
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
