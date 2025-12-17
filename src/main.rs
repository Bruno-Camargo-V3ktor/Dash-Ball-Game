use bevy::{
    prelude::*,
    window::{PrimaryWindow, WindowResolution},
};
use rand::random;

pub const PLAYER_SPEED: f32 = 500.0;
pub const PLAYER_SIZE: f32 = 64.0;
#[derive(Component)]
pub struct Player {}

pub const NUMBER_OF_ENEMIES: usize = 4;
#[derive(Component)]
struct Enemy {}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (spawn_camera, spawn_player, spawn_enemies).chain())
        .add_systems(Update, (player_movement, confine_player).chain())
        .run();
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

pub fn spawn_camera(
    mut commands: Commands,
    mut window_query: Query<&mut Window, With<PrimaryWindow>>,
) {
    let mut window = window_query.single_mut().unwrap();

    window.resolution = WindowResolution::new(1280, 720);
    window.resizable = false;

    commands.spawn((
        Camera2d::default(),
        Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
    ));
}

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
            Enemy {},
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
    time: Res<Time>,
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

        translation.x = if translation.x < x_min {
            x_min
        } else if translation.x > x_max {
            x_max
        } else {
            translation.x
        };

        translation.y = if translation.y < y_min {
            y_min
        } else if translation.y > y_max {
            y_max
        } else {
            translation.y
        };

        player_transform.translation = translation;
    }
}
