use bevy::{ prelude::*, window::{ PrimaryWindow, WindowResolution } };
use rand::random;

pub const PLAYER_SPEED: f32 = 500.0;
pub const PLAYER_SIZE: f32 = 64.0;
#[derive(Component)]
pub struct Player {}

#[derive(Component)]
pub struct ExplosionSoundPlayer {}
impl ExplosionSoundPlayer {
    pub fn new(
        asset_serve: &Res<'_, AssetServer>
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

pub const NUMBER_OF_ENEMIES: usize = 4;
pub const ENEMY_SPEED: f32 = 200.0;
pub const ENEMY_SIZE: f32 = 64.0;
#[derive(Component)]
pub struct Enemy {
    pub direction: Vec2,
}

#[derive(Component)]
pub struct BouncEnemySound {}
impl BouncEnemySound {
    pub fn new(
        asset_server: &Res<'_, AssetServer>
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

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (spawn_camera, spawn_player, spawn_enemies).chain())
        .add_systems(
            Update,
            (
                camera_position,
                player_movement,
                confine_player,
                confine_enemy,
                update_enemy_direction,
                enemy_movement,
                enemy_hit_player,
            ).chain()
        )
        .run();
}

pub fn spawn_player(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>
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

pub fn spawn_camera(
    mut commands: Commands,
    mut window_query: Query<&mut Window, With<PrimaryWindow>>
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
    window_query: Query<&Window, With<PrimaryWindow>>
) {
    if let (Ok(mut camera), Ok(window)) = (camera_query.single_mut(), window_query.single()) {
        camera.translation.x = window.width() / 2.0;
        camera.translation.y = window.height() / 2.0;
    }
}

pub fn spawn_enemies(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>
) {
    let window = window_query.single().unwrap();

    for _ in 0..NUMBER_OF_ENEMIES {
        let random_x = random::<f32>() * window.width();
        let random_y = random::<f32>() * window.height();

        commands.spawn((
            Enemy { direction: Vec2::new(random::<f32>(), random::<f32>()).normalize() },
            Sprite {
                image: asset_server.load("sprites/ball_red_large.png"),
                ..Default::default()
            },
            Transform::from_xyz(random_x, random_y, 0.0),
        ));
    }
}

pub fn player_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<&mut Transform, With<Player>>,
    time: Res<Time>
) {
    if let Ok(mut transform) = player_query.single_mut() {
        let mut dir = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::ArrowLeft) || keyboard_input.pressed(KeyCode::KeyA) {
            dir += Vec3::new(-1.0, 0.0, 0.0);
        }

        if keyboard_input.pressed(KeyCode::ArrowRight) || keyboard_input.pressed(KeyCode::KeyD) {
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
    window_query: Query<&Window, With<PrimaryWindow>>
) {
    if
        let (Ok(mut player_transform), Ok(window)) = (
            player_query.single_mut(),
            window_query.single(),
        )
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
    mut commands: Commands
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
    window_query: Query<&Window, With<PrimaryWindow>>
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
    mut player_query: Query<(Entity, &Transform), With<Player>>,
    enemy_query: Query<&Transform, With<Enemy>>,
    asset_server: Res<AssetServer>
) {
    if let Ok((player_entity, player_transform)) = player_query.single_mut() {
        for enemy_transform in enemy_query {
            let distance = player_transform.translation.distance(enemy_transform.translation);

            let player_radius = PLAYER_SIZE / 2.0;
            let enemy_radius = ENEMY_SIZE / 2.0;

            if distance < player_radius + enemy_radius {
                commands.entity(player_entity).despawn();
                commands.spawn(ExplosionSoundPlayer::new(&asset_server));
            }
        }
    }
}
