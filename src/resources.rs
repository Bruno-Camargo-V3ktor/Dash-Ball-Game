pub mod score {
    use bevy::prelude::*;

    #[derive(Resource, Default)]
    pub struct Score {
        pub value: u32,
    }

    #[derive(Resource, Default)]
    pub struct HighScores {
        pub scores: Vec<(String, u32)>,
    }
}

pub mod timers {
    use crate::components::{enemy::ENEMY_SPAWN_TIME, star::STAR_SPAWN_TIME};
    use bevy::prelude::*;

    #[derive(Resource)]
    pub struct StarSpawnTimer {
        pub timer: Timer,
    }

    impl Default for StarSpawnTimer {
        fn default() -> Self {
            Self {
                timer: Timer::from_seconds(STAR_SPAWN_TIME, TimerMode::Repeating),
            }
        }
    }

    #[derive(Resource)]
    pub struct EnemySpawnTimer {
        pub timer: Timer,
    }

    impl Default for EnemySpawnTimer {
        fn default() -> Self {
            Self {
                timer: Timer::from_seconds(ENEMY_SPAWN_TIME, TimerMode::Repeating),
            }
        }
    }
}
