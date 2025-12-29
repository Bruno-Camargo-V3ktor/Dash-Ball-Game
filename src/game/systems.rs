pub mod player {
    use super::super::{
        components::{
            player::{PLAYER_SIZE, PLAYER_SPEED, Player},
            star::{CollectStarSound, STAR_SIZE, Star},
        },
        resources::score::Score,
    };
    use bevy::{prelude::*, window::PrimaryWindow};

    #[derive(SystemSet, Debug, Clone, Hash, PartialEq, Eq)]
    pub enum PlayerStateSet {
        Movement,
        Confine,
    }

    pub fn spawn_player(
        mut commands: Commands,
        window_query: Query<&Window, With<PrimaryWindow>>,
        asset_server: Res<AssetServer>,
    ) {
        let window = window_query.single().unwrap();

        commands.spawn((
            Player {},
            Sprite {
                image: asset_server.load("sprites/ball_blue_large.png"),
                ..Default::default()
            },
            Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        ));
    }

    pub fn despawn_player(mut commands: Commands, player_query: Query<Entity, With<Player>>) {
        for entity in player_query {
            commands.entity(entity).despawn();
        }
    }

    pub fn player_movement(
        keyboard_input: Res<ButtonInput<KeyCode>>,
        mut player_query: Query<&mut Transform, With<Player>>,
        time: Res<Time>,
    ) {
        if let Ok(mut transform) = player_query.single_mut() {
            let mut dir = Vec3::ZERO;

            if keyboard_input.pressed(KeyCode::ArrowLeft) || keyboard_input.pressed(KeyCode::KeyA) {
                dir += Vec3::new(-1.0, 0.0, 0.0);
            }

            if keyboard_input.pressed(KeyCode::ArrowRight) || keyboard_input.pressed(KeyCode::KeyD)
            {
                dir += Vec3::new(1.0, 0.0, 0.0);
            }

            if keyboard_input.pressed(KeyCode::ArrowDown) || keyboard_input.pressed(KeyCode::KeyS) {
                dir += Vec3::new(0.0, -1.0, 0.0);
            }

            if keyboard_input.pressed(KeyCode::ArrowUp) || keyboard_input.pressed(KeyCode::KeyW) {
                dir += Vec3::new(0.0, 1.0, 0.0);
            }

            if dir.length() > 0.0 {
                dir = dir.normalize();
            }

            transform.translation += dir * (PLAYER_SPEED * time.delta_secs());
        }
    }

    pub fn confine_player(
        mut player_query: Query<&mut Transform, With<Player>>,
        window_query: Query<&Window, With<PrimaryWindow>>,
    ) {
        if let (Ok(mut player_transform), Ok(window)) =
            (player_query.single_mut(), window_query.single())
        {
            let half_player_size: f32 = PLAYER_SIZE / 2.0;
            let x_min = 0.0 + half_player_size;
            let x_max = window.width() - half_player_size;
            let y_min = 0.0 + half_player_size;
            let y_max = window.height() - half_player_size;

            let mut translation = player_transform.translation;
            translation.x = translation.x.clamp(x_min, x_max);
            translation.y = translation.y.clamp(y_min, y_max);
            player_transform.translation = translation;
        }
    }

    pub fn player_hit_star(
        mut commands: Commands,
        player_query: Query<&Transform, With<Player>>,
        stars_query: Query<(Entity, &Transform), With<Star>>,
        asset_serve: Res<AssetServer>,
        mut score: ResMut<Score>,
    ) {
        if let Ok(player_transform) = player_query.single() {
            for (star_entity, star_transform) in stars_query {
                let distance = player_transform
                    .translation
                    .distance(star_transform.translation);

                let player_radius = PLAYER_SIZE / 2.0;
                let star_radius = STAR_SIZE / 2.0;

                if distance < player_radius + star_radius {
                    commands.entity(star_entity).despawn();
                    commands.spawn(CollectStarSound {
                        audio: AudioPlayer(asset_serve.load("audio/laserLarge_000.ogg")),
                        settings: PlaybackSettings::DESPAWN,
                    });
                    score.value += 1;
                }
            }
        }
    }
}

pub mod star {
    use super::super::{
        components::star::{NUMBER_OF_STARS, STAR_SIZE, Star},
        resources::timers::StarSpawnTimer,
    };
    use bevy::{prelude::*, window::PrimaryWindow};
    use rand::random;

    pub fn spawn_stars(
        mut commands: Commands,
        window_query: Query<&Window, With<PrimaryWindow>>,
        asset_server: Res<AssetServer>,
    ) {
        if let Ok(window) = window_query.single() {
            for _ in 0..NUMBER_OF_STARS {
                let pos_x =
                    random::<f32>() * window.width().clamp(STAR_SIZE, window.width() - STAR_SIZE);
                let pos_y = random::<f32>()
                    * window
                        .height()
                        .clamp(STAR_SIZE, window.height() - STAR_SIZE);

                commands.spawn((
                    Star {},
                    Sprite::from_image(asset_server.load("sprites/star.png")),
                    Transform::from_xyz(pos_x, pos_y, 0.0),
                ));
            }
        }
    }

    pub fn despawn_stars(mut commands: Commands, stars_query: Query<Entity, With<Star>>) {
        for entity in stars_query {
            commands.entity(entity).despawn();
        }
    }

    pub fn spawn_stars_over_time(
        mut commands: Commands,
        window_query: Query<&Window, With<PrimaryWindow>>,
        asset_server: Res<AssetServer>,
        star_spawn_timer: Res<StarSpawnTimer>,
    ) {
        if star_spawn_timer.timer.is_finished() {
            if let Ok(window) = window_query.single() {
                let pos_x =
                    random::<f32>() * window.width().clamp(STAR_SIZE, window.width() - STAR_SIZE);
                let pos_y = random::<f32>()
                    * window
                        .height()
                        .clamp(STAR_SIZE, window.height() - STAR_SIZE);

                commands.spawn((
                    Star {},
                    Sprite::from_image(asset_server.load("sprites/star.png")),
                    Transform::from_xyz(pos_x, pos_y, 0.0),
                ));
            }
        }
    }
}

pub mod enemy {
    use super::super::{
        components::{
            enemy::*,
            player::{ExplosionSoundPlayer, PLAYER_SIZE, Player},
        },
        messages::game_states::GameOver,
        resources::{score::Score, timers::EnemySpawnTimer},
    };
    use bevy::{prelude::*, window::PrimaryWindow};
    use rand::random;

    pub fn spawn_enemies(
        mut commands: Commands,
        window_query: Query<&Window, With<PrimaryWindow>>,
        asset_server: Res<AssetServer>,
    ) {
        let window = window_query.single().unwrap();

        for _ in 0..NUMBER_OF_ENEMIES {
            let random_x = random::<f32>() * window.width();
            let random_y = random::<f32>() * window.height();

            commands.spawn((
                Enemy {
                    direction: Vec2::new(random::<f32>(), random::<f32>()).normalize(),
                },
                Sprite {
                    image: asset_server.load("sprites/ball_red_large.png"),
                    ..Default::default()
                },
                Transform::from_xyz(random_x, random_y, 0.0),
            ));
        }
    }

    pub fn despawn_enemys(mut commands: Commands, enemys_query: Query<Entity, With<Enemy>>) {
        for entity in enemys_query {
            commands.entity(entity).despawn();
        }
    }

    pub fn enemy_movement(enemy_query: Query<(&mut Transform, &Enemy)>, time: Res<Time>) {
        for (mut transform, enemy) in enemy_query {
            let dir = Vec3::new(enemy.direction.x, enemy.direction.y, 0.0);
            transform.translation += dir * (ENEMY_SPEED * time.delta_secs());
        }
    }

    pub fn update_enemy_direction(
        enemy_query: Query<(&Transform, &mut Enemy)>,
        window_query: Query<&Window, With<PrimaryWindow>>,
        asset_server: Res<AssetServer>,
        mut commands: Commands,
    ) {
        if let Ok(window) = window_query.single() {
            let half_enemy_size: f32 = ENEMY_SIZE / 2.0;
            let x_min = 0.0 + half_enemy_size;
            let x_max = window.width() - half_enemy_size;
            let y_min = 0.0 + half_enemy_size;
            let y_max = window.height() - half_enemy_size;

            for (transform, mut enemy) in enemy_query {
                let translation = transform.translation;

                if translation.x <= x_min || translation.x >= x_max {
                    enemy.direction.x *= -1.0;
                    commands.spawn(BouncEnemySound::new(&asset_server));
                }

                if translation.y <= y_min || translation.y >= y_max {
                    enemy.direction.y *= -1.0;
                    commands.spawn(BouncEnemySound::new(&asset_server));
                }
            }
        }
    }

    pub fn confine_enemy(
        enemy_query: Query<&mut Transform, With<Enemy>>,
        window_query: Query<&Window, With<PrimaryWindow>>,
    ) {
        if let Ok(window) = window_query.single() {
            let half_enemy_size: f32 = ENEMY_SIZE / 2.0;
            let x_min = 0.0 + half_enemy_size;
            let x_max = window.width() - half_enemy_size;
            let y_min = 0.0 + half_enemy_size;
            let y_max = window.height() - half_enemy_size;

            for mut transform in enemy_query {
                let mut translation = transform.translation;

                translation.x = translation.x.clamp(x_min, x_max);
                translation.y = translation.y.clamp(y_min, y_max);
                transform.translation = translation;
            }
        }
    }

    pub fn enemy_hit_player(
        mut commands: Commands,
        mut gameover_writer: MessageWriter<GameOver>,
        mut player_query: Query<(Entity, &Transform), With<Player>>,
        enemy_query: Query<&Transform, With<Enemy>>,
        asset_server: Res<AssetServer>,
        score: Res<Score>,
    ) {
        if let Ok((player_entity, player_transform)) = player_query.single_mut() {
            for enemy_transform in enemy_query {
                let distance = player_transform
                    .translation
                    .distance(enemy_transform.translation);

                let player_radius = PLAYER_SIZE / 2.0;
                let enemy_radius = ENEMY_SIZE / 2.0;

                if distance < player_radius + enemy_radius {
                    commands.entity(player_entity).despawn();
                    commands.spawn(ExplosionSoundPlayer::new(&asset_server));
                    gameover_writer.write(GameOver { score: score.value });
                }
            }
        }
    }

    pub fn spawn_enemys_over_time(
        mut commands: Commands,
        window_query: Query<&Window, With<PrimaryWindow>>,
        asset_server: Res<AssetServer>,
        enemy_spawn_timer: Res<EnemySpawnTimer>,
    ) {
        if enemy_spawn_timer.timer.is_finished() {
            let window = window_query.single().unwrap();

            let random_x = random::<f32>() * window.width();
            let random_y = random::<f32>() * window.height();

            commands.spawn((
                Enemy {
                    direction: Vec2::new(random::<f32>(), random::<f32>()).normalize(),
                },
                Sprite {
                    image: asset_server.load("sprites/ball_red_large.png"),
                    ..Default::default()
                },
                Transform::from_xyz(random_x, random_y, 0.0),
            ));
        }
    }
}

pub mod camera {
    use bevy::{
        prelude::*,
        window::{PrimaryWindow, WindowResolution},
    };

    pub fn spawn_camera(
        mut commands: Commands,
        mut window_query: Query<&mut Window, With<PrimaryWindow>>,
    ) {
        let mut window = window_query.single_mut().unwrap();

        window.resolution = WindowResolution::new(1280, 720);
        //window.resizable = false;

        commands.spawn((
            Camera2d::default(),
            Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        ));
    }

    pub fn camera_position(
        mut camera_query: Query<&mut Transform, With<Camera2d>>,
        window_query: Query<&Window, With<PrimaryWindow>>,
    ) {
        if let (Ok(mut camera), Ok(window)) = (camera_query.single_mut(), window_query.single()) {
            camera.translation.x = window.width() / 2.0;
            camera.translation.y = window.height() / 2.0;
        }
    }
}

pub mod timers {
    use super::super::resources::timers::{EnemySpawnTimer, StarSpawnTimer};
    use bevy::prelude::*;

    pub fn tick_star_spawn_timer(mut star_spawn_timer: ResMut<StarSpawnTimer>, time: Res<Time>) {
        star_spawn_timer.timer.tick(time.delta());
    }

    pub fn tick_enemy_spawn_timer(mut enemy_spawn_timer: ResMut<EnemySpawnTimer>, time: Res<Time>) {
        enemy_spawn_timer.timer.tick(time.delta());
    }
}

pub mod game_state {
    use crate::AppState;

    use super::super::messages::game_states::GameOver;
    use super::super::resources::score::HighScores;
    use super::super::states::SimulationState;
    use bevy::prelude::*;

    pub fn exit_game(keyboard_input: Res<ButtonInput<KeyCode>>, mut commands: Commands) {
        if keyboard_input.just_pressed(KeyCode::Escape) {
            commands.write_message(AppExit::Success);
        }
    }

    pub fn handle_game_over(mut gameover_reader: MessageReader<GameOver>) {
        for game_over in gameover_reader.read() {
            println!("Game Over: {}", game_over.score);
        }
    }

    pub fn update_high_scores(
        mut gameover_reader: MessageReader<GameOver>,
        mut high_scores: ResMut<HighScores>,
    ) {
        for game_over in gameover_reader.read() {
            high_scores
                .scores
                .push(("Player".to_string(), game_over.score));
            println!("{:?}", ("Player".to_string(), game_over.score));
        }
    }

    pub fn toggle_game_simulation(
        state: Res<State<SimulationState>>,
        mut change_state: ResMut<NextState<SimulationState>>,
        keyboard_input: Res<ButtonInput<KeyCode>>,
    ) {
        if keyboard_input.just_pressed(KeyCode::Space) {
            match state.get() {
                SimulationState::GamePaused => {
                    change_state.set(SimulationState::GameRunning);
                }
                SimulationState::GameRunning => {
                    change_state.set(SimulationState::GamePaused);
                }
            }
        }
    }

    pub fn transition_to_main_menu(
        state: Res<State<AppState>>,
        mut change_state: ResMut<NextState<AppState>>,
        keyboard_input: Res<ButtonInput<KeyCode>>,
    ) {
        if keyboard_input.just_pressed(KeyCode::Backspace) && *state.get() == AppState::Game {
            change_state.set(AppState::MainMenu);
        }
    }
}
