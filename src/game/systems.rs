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
    use crate::game::resources::score::Score;

    use super::super::messages::game_states::GameOver;
    use super::super::resources::score::HighScores;
    use super::super::states::SimulationState;
    use bevy::prelude::*;

    pub fn exit_game(keyboard_input: Res<ButtonInput<KeyCode>>, mut commands: Commands) {
        if keyboard_input.just_pressed(KeyCode::Escape) {
            commands.write_message(AppExit::Success);
        }
    }

    pub fn handle_game_over(
        mut gameover_reader: MessageReader<GameOver>,
        mut change_app_state: ResMut<NextState<AppState>>,
    ) {
        for game_over in gameover_reader.read() {
            println!("Game Over: {}", game_over.score);
            change_app_state.set(AppState::GameOver);
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
        app_state: Res<State<AppState>>,
        mut change_app_state: ResMut<NextState<AppState>>,
        mut change_game_state: ResMut<NextState<SimulationState>>,
        keyboard_input: Res<ButtonInput<KeyCode>>,
    ) {
        if keyboard_input.just_pressed(KeyCode::Backspace) && *app_state.get() != AppState::MainMenu
        {
            change_app_state.set(AppState::MainMenu);
            change_game_state.set(SimulationState::GameRunning);
        }
    }

    pub fn insert_score(mut commands: Commands) {
        commands.insert_resource(Score::default());
    }

    pub fn remove_score(mut commands: Commands) {
        commands.remove_resource::<Score>();
    }
}

pub mod ui {
    pub mod hud {
        use super::super::super::components::ui::hud::*;
        use super::super::super::components::*;
        use super::super::super::resources;
        use bevy::prelude::*;

        pub fn spawn_hud(mut commands: Commands, asset_server: Res<AssetServer>) {
            commands
                .spawn((
                    HUD,
                    Node {
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        flex_direction: FlexDirection::Row,
                        justify_content: JustifyContent::SpaceBetween,

                        ..Default::default()
                    },
                ))
                .with_children(|p| {
                    p.spawn((
                        Node {
                            width: Val::Px(150.0),
                            height: Val::Px(80.0),

                            margin: UiRect::all(Val::Px(32.0)),

                            flex_direction: FlexDirection::Row,
                            justify_content: JustifyContent::SpaceAround,
                            align_items: AlignItems::Center,

                            ..Default::default()
                        },
                        BackgroundColor(Color::linear_rgba(0.15, 0.15, 0.15, 0.5)),
                    ))
                    .with_children(|p| {
                        p.spawn(Node {
                            width: Val::Px(32.0),
                            height: Val::Px(32.0),
                            ..Default::default()
                        })
                        .with_child((ImageNode {
                            image: asset_server.load("sprites/star.png"),
                            ..Default::default()
                        },));

                        p.spawn((
                            Score,
                            Text("0".into()),
                            TextLayout {
                                justify: Justify::Center,
                                linebreak: LineBreak::NoWrap,
                            },
                            TextFont {
                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                font_size: 48.0,
                                ..Default::default()
                            },
                            TextColor::WHITE,
                        ));
                    });

                    p.spawn((
                        Node {
                            width: Val::Px(150.0),
                            height: Val::Px(80.0),

                            margin: UiRect::all(Val::Px(32.0)),

                            flex_direction: FlexDirection::Row,
                            justify_content: JustifyContent::SpaceAround,
                            align_items: AlignItems::Center,

                            ..Default::default()
                        },
                        BackgroundColor(Color::linear_rgba(0.15, 0.15, 0.15, 0.5)),
                    ))
                    .with_children(|p| {
                        p.spawn(Node {
                            width: Val::Px(32.0),
                            height: Val::Px(32.0),
                            ..Default::default()
                        })
                        .with_child((ImageNode {
                            image: asset_server.load("sprites/ball_red_large.png"),
                            ..Default::default()
                        },));

                        p.spawn((
                            Enemy,
                            Text("0".into()),
                            TextLayout {
                                justify: Justify::Center,
                                linebreak: LineBreak::NoWrap,
                            },
                            TextFont {
                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                font_size: 48.0,
                                ..Default::default()
                            },
                            TextColor::WHITE,
                        ));
                    });
                });
        }

        pub fn despawn_hud(mut commands: Commands, hud_query: Query<Entity, With<HUD>>) {
            let hud = hud_query.single().unwrap();
            commands.entity(hud).despawn();
        }

        pub fn update_score_text(
            mut text_score: Query<&mut Text, With<Score>>,
            score: Res<resources::score::Score>,
        ) {
            if let Ok(mut text) = text_score.single_mut() {
                let score = score.value;
                text.0 = format!("{score}");
            }
        }

        pub fn update_enemys_text(
            mut text_enemy: Query<&mut Text, With<Enemy>>,
            enemys_query: Query<Entity, With<enemy::Enemy>>,
        ) {
            if let Ok(mut text) = text_enemy.single_mut() {
                let len = enemys_query.iter().len();
                text.0 = format!("{len}");
            }
        }
    }

    pub mod pause_menu {
        pub const NORMAL_BUTTON_COLOR: Color = Color::linear_rgb(0.15, 0.15, 0.15);
        pub const HOVERED_BUTTON_COLOR: Color = Color::linear_rgb(0.25, 0.25, 0.25);
        pub const PRESSED_BUTTON_COLOR: Color = Color::linear_rgb(0.35, 0.75, 0.35);

        use crate::{AppState, game::states::SimulationState};

        use super::super::super::components::ui::pause_menu::*;
        use bevy::prelude::*;

        pub fn spawn_pause_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
            commands
                .spawn((
                    PauseMenuContainer,
                    Node {
                        padding: UiRect::axes(Val::Px(64.0), Val::Px(64.0)),
                        width: Val::Vw(100.0),
                        height: Val::Vh(100.0),

                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,

                        ..Default::default()
                    },
                ))
                .with_children(|p| {
                    let _container = p
                        .spawn((
                            Node {
                                padding: UiRect::axes(Val::Px(64.0), Val::Px(64.0)),
                                width: Val::Auto,
                                height: Val::Auto,

                                flex_direction: FlexDirection::Column,
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,

                                row_gap: Val::Px(16.0),

                                ..Default::default()
                            },
                            BackgroundColor(Color::linear_rgba(0.15, 0.15, 0.15, 0.5)),
                        ))
                        .with_children(|p| {
                            // Title
                            p.spawn((
                                Text::new("Pause"),
                                TextLayout {
                                    justify: Justify::Center,
                                    linebreak: LineBreak::NoWrap,
                                },
                                TextFont {
                                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                    font_size: 64.0,
                                    ..Default::default()
                                },
                                TextColor::WHITE,
                            ));

                            // Button Resume
                            p.spawn(create_button(ButtonResume))
                                .with_child(create_text_button("Resume", &asset_server));

                            // Button Main Menu
                            p.spawn(create_button(ButtonMainMenu))
                                .with_child(create_text_button("Main Menu", &asset_server));

                            // Button Quit
                            p.spawn(create_button(ButtonQuit))
                                .with_child(create_text_button("Quit", &asset_server));
                        });
                });
        }

        pub fn despawn_pause_menu(
            mut commands: Commands,
            pause_menu: Query<Entity, With<PauseMenuContainer>>,
        ) {
            if let Ok(entity) = pause_menu.single() {
                commands.entity(entity).despawn();
            }
        }

        pub fn create_button(component: impl Component) -> impl Bundle {
            (
                component,
                Node {
                    width: Val::Px(300.0),
                    height: Val::Px(100.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                BackgroundColor(NORMAL_BUTTON_COLOR),
                Button,
            )
        }

        pub fn create_text_button(
            text: impl Into<String>,
            asset_server: &Res<AssetServer>,
        ) -> impl Bundle {
            (
                Text::new(text),
                TextLayout::new_with_justify(Justify::Center),
                TextFont {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 32.0,
                    ..Default::default()
                },
                TextColor::WHITE,
            )
        }

        pub fn interact_with_resume_button(
            mut button_query: Query<
                (&Interaction, &mut BackgroundColor),
                (Changed<Interaction>, With<ButtonResume>),
            >,
            mut changed_state: ResMut<NextState<SimulationState>>,
        ) {
            if let Ok((interaction, mut background)) = button_query.single_mut() {
                match *interaction {
                    Interaction::Hovered => {
                        background.0 = HOVERED_BUTTON_COLOR;
                    }
                    Interaction::Pressed => {
                        background.0 = PRESSED_BUTTON_COLOR;
                        changed_state.set(SimulationState::GameRunning);
                    }
                    Interaction::None => {
                        background.0 = NORMAL_BUTTON_COLOR;
                    }
                }
            }
        }

        pub fn interact_with_main_menu_button(
            mut button_query: Query<
                (&Interaction, &mut BackgroundColor),
                (Changed<Interaction>, With<ButtonMainMenu>),
            >,
            mut changed_app_state: ResMut<NextState<AppState>>,
            mut changed_game_state: ResMut<NextState<SimulationState>>,
        ) {
            if let Ok((interaction, mut background)) = button_query.single_mut() {
                match *interaction {
                    Interaction::Hovered => {
                        background.0 = HOVERED_BUTTON_COLOR;
                    }
                    Interaction::Pressed => {
                        background.0 = PRESSED_BUTTON_COLOR;
                        changed_game_state.set(SimulationState::GameRunning);
                        changed_app_state.set(AppState::MainMenu);
                    }
                    Interaction::None => {
                        background.0 = NORMAL_BUTTON_COLOR;
                    }
                }
            }
        }

        pub fn interact_with_quit_button(
            mut button_query: Query<
                (&Interaction, &mut BackgroundColor),
                (Changed<Interaction>, With<ButtonQuit>),
            >,
            mut commands: Commands,
        ) {
            if let Ok((interaction, mut background)) = button_query.single_mut() {
                match *interaction {
                    Interaction::Hovered => {
                        background.0 = HOVERED_BUTTON_COLOR;
                    }
                    Interaction::Pressed => {
                        background.0 = PRESSED_BUTTON_COLOR;
                        commands.write_message(AppExit::Success);
                    }
                    Interaction::None => {
                        background.0 = NORMAL_BUTTON_COLOR;
                    }
                }
            }
        }
    }

    pub mod gameover {
        pub const NORMAL_BUTTON_COLOR: Color = Color::linear_rgb(0.15, 0.15, 0.15);
        pub const HOVERED_BUTTON_COLOR: Color = Color::linear_rgb(0.25, 0.25, 0.25);
        pub const PRESSED_BUTTON_COLOR: Color = Color::linear_rgb(0.35, 0.75, 0.35);

        use crate::{AppState, game::states::SimulationState};

        use super::super::super::components::ui::gameover::*;
        use bevy::prelude::*;

        pub fn spawn_gameover_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
            commands
                .spawn((
                    GameOverContainer,
                    Node {
                        padding: UiRect::axes(Val::Px(64.0), Val::Px(64.0)),
                        width: Val::Vw(100.0),
                        height: Val::Vh(100.0),

                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,

                        ..Default::default()
                    },
                ))
                .with_children(|p| {
                    let _container = p
                        .spawn((
                            Node {
                                padding: UiRect::axes(Val::Px(64.0), Val::Px(64.0)),
                                width: Val::Auto,
                                height: Val::Auto,

                                flex_direction: FlexDirection::Column,
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,

                                row_gap: Val::Px(16.0),

                                ..Default::default()
                            },
                            BackgroundColor(Color::linear_rgba(0.15, 0.15, 0.15, 0.5)),
                        ))
                        .with_children(|p| {
                            // Title
                            p.spawn((
                                Text::new("GameOver"),
                                TextLayout {
                                    justify: Justify::Center,
                                    linebreak: LineBreak::NoWrap,
                                },
                                TextFont {
                                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                    font_size: 64.0,
                                    ..Default::default()
                                },
                                TextColor::WHITE,
                            ));

                            // Button Resume
                            p.spawn(create_button(ButtonRestart))
                                .with_child(create_text_button("Restart", &asset_server));

                            // Button Main Menu
                            p.spawn(create_button(ButtonMainMenu))
                                .with_child(create_text_button("Main Menu", &asset_server));

                            // Button Quit
                            p.spawn(create_button(ButtonQuit))
                                .with_child(create_text_button("Quit", &asset_server));
                        });
                });
        }

        pub fn despawn_gameover_menu(
            mut commands: Commands,
            pause_menu: Query<Entity, With<GameOverContainer>>,
        ) {
            if let Ok(entity) = pause_menu.single() {
                commands.entity(entity).despawn();
            }
        }

        pub fn create_button(component: impl Component) -> impl Bundle {
            (
                component,
                Node {
                    width: Val::Px(300.0),
                    height: Val::Px(100.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                BackgroundColor(NORMAL_BUTTON_COLOR),
                Button,
            )
        }

        pub fn create_text_button(
            text: impl Into<String>,
            asset_server: &Res<AssetServer>,
        ) -> impl Bundle {
            (
                Text::new(text),
                TextLayout::new_with_justify(Justify::Center),
                TextFont {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 32.0,
                    ..Default::default()
                },
                TextColor::WHITE,
            )
        }

        pub fn interact_with_restart_button(
            mut button_query: Query<
                (&Interaction, &mut BackgroundColor),
                (Changed<Interaction>, With<ButtonRestart>),
            >,
            mut changed_app_state: ResMut<NextState<AppState>>,
            mut changed_game_state: ResMut<NextState<SimulationState>>,
        ) {
            if let Ok((interaction, mut background)) = button_query.single_mut() {
                match *interaction {
                    Interaction::Hovered => {
                        background.0 = HOVERED_BUTTON_COLOR;
                    }
                    Interaction::Pressed => {
                        background.0 = PRESSED_BUTTON_COLOR;
                        changed_game_state.set(SimulationState::GameRunning);
                        changed_app_state.set(AppState::Game);
                    }
                    Interaction::None => {
                        background.0 = NORMAL_BUTTON_COLOR;
                    }
                }
            }
        }

        pub fn interact_with_main_menu_button(
            mut button_query: Query<
                (&Interaction, &mut BackgroundColor),
                (Changed<Interaction>, With<ButtonMainMenu>),
            >,
            mut changed_app_state: ResMut<NextState<AppState>>,
            mut changed_game_state: ResMut<NextState<SimulationState>>,
        ) {
            if let Ok((interaction, mut background)) = button_query.single_mut() {
                match *interaction {
                    Interaction::Hovered => {
                        background.0 = HOVERED_BUTTON_COLOR;
                    }
                    Interaction::Pressed => {
                        background.0 = PRESSED_BUTTON_COLOR;
                        changed_game_state.set(SimulationState::GameRunning);
                        changed_app_state.set(AppState::MainMenu);
                    }
                    Interaction::None => {
                        background.0 = NORMAL_BUTTON_COLOR;
                    }
                }
            }
        }

        pub fn interact_with_quit_button(
            mut button_query: Query<
                (&Interaction, &mut BackgroundColor),
                (Changed<Interaction>, With<ButtonQuit>),
            >,
            mut commands: Commands,
        ) {
            if let Ok((interaction, mut background)) = button_query.single_mut() {
                match *interaction {
                    Interaction::Hovered => {
                        background.0 = HOVERED_BUTTON_COLOR;
                    }
                    Interaction::Pressed => {
                        background.0 = PRESSED_BUTTON_COLOR;
                        commands.write_message(AppExit::Success);
                    }
                    Interaction::None => {
                        background.0 = NORMAL_BUTTON_COLOR;
                    }
                }
            }
        }
    }
}
