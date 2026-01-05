pub mod player {
    use bevy::prelude::*;

    pub const PLAYER_SPEED: f32 = 500.0;
    pub const PLAYER_SIZE: f32 = 64.0;
    #[derive(Component)]
    pub struct Player {}

    #[derive(Component)]
    pub struct ExplosionSoundPlayer {}
    impl ExplosionSoundPlayer {
        pub fn new(
            asset_serve: &Res<'_, AssetServer>,
        ) -> (ExplosionSoundPlayer, AudioPlayer, PlaybackSettings) {
            (
                ExplosionSoundPlayer {},
                AudioPlayer::new(asset_serve.load("audio/assets_audio_explosionCrunch_000.oga")),
                PlaybackSettings {
                    mode: bevy::audio::PlaybackMode::Despawn,
                    ..Default::default()
                },
            )
        }
    }
}

pub mod enemy {
    use bevy::prelude::*;
    use rand::random;

    pub const NUMBER_OF_ENEMIES: usize = 2;
    pub const ENEMY_SPEED: f32 = 200.0;
    pub const ENEMY_SIZE: f32 = 64.0;
    pub const ENEMY_SPAWN_TIME: f32 = 5.0;

    #[derive(Component)]
    pub struct Enemy {
        pub direction: Vec2,
    }

    #[derive(Component)]
    pub struct BouncEnemySound {}
    impl BouncEnemySound {
        pub fn new(
            asset_server: &Res<'_, AssetServer>,
        ) -> (BouncEnemySound, AudioPlayer, PlaybackSettings) {
            let audio = if random::<f32>() < 0.5 {
                asset_server.load("audio/pluck_001.ogg")
            } else {
                asset_server.load("audio/pluck_002.ogg")
            };

            (
                BouncEnemySound {},
                AudioPlayer::new(audio),
                PlaybackSettings {
                    mode: bevy::audio::PlaybackMode::Despawn,
                    ..Default::default()
                },
            )
        }
    }
}

pub mod star {
    use bevy::prelude::*;

    pub const NUMBER_OF_STARS: usize = 3;
    pub const STAR_SIZE: f32 = 30.0;
    pub const STAR_SPAWN_TIME: f32 = 3.0;

    #[derive(Component)]
    pub struct Star {}

    #[derive(Bundle)]
    pub struct CollectStarSound {
        pub audio: AudioPlayer,
        pub settings: PlaybackSettings,
    }
}

pub mod ui {
    pub mod hud {
        use bevy::prelude::*;

        #[derive(Component)]
        pub struct HUD;

        #[derive(Component)]
        pub struct Score;

        #[derive(Component)]
        pub struct Enemy;
    }

    pub mod pause_menu {
        use bevy::prelude::*;

        #[derive(Component)]
        pub struct PauseMenuContainer;

        #[derive(Component)]
        pub struct ButtonResume;

        #[derive(Component)]
        pub struct ButtonMainMenu;

        #[derive(Component)]
        pub struct ButtonQuit;
    }
}
