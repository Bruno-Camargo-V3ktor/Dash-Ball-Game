pub mod game_states {
    use bevy::prelude::*;

    #[derive(Message)]
    pub struct GameOver {
        pub score: u32,
    }
}
