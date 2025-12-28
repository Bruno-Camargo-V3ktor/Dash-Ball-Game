use bevy::prelude::*;

mod components;
mod messages;
mod plugins;
mod resources;
mod states;
mod systems;

use plugins::*;
use states::*;

pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<SimulationState>()
            .add_plugins(GameStatePlugin)
            .add_plugins(TimersPlugin)
            .add_plugins(CameraPlugin)
            .add_plugins(PlayerPlugin)
            .add_plugins(EnemyPlugin)
            .add_plugins(StarPlugin);
    }
}
