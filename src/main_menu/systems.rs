pub mod main_menu_state {
    use crate::AppState;
    use bevy::prelude::*;

    pub fn transition_to_game(
        state: Res<State<AppState>>,
        mut changed_state: ResMut<NextState<AppState>>,
        keyboard_input: Res<ButtonInput<KeyCode>>,
    ) {
        if keyboard_input.just_pressed(KeyCode::Enter) && *state.get() == AppState::MainMenu {
            changed_state.set(AppState::Game);
        }
    }
}

pub mod layout {
    use super::super::components::layout::{MainMenu, PlayButton, QuitButton};
    use super::super::styles::*;
    use bevy::prelude::*;

    pub fn spawn_main_menu(mut commands: Commands, assert_server: Res<AssetServer>) {
        let main_menu_entity = build_main_menu(&mut commands, &assert_server);
    }

    pub fn despawn_main_menu(
        mut commands: Commands,
        main_menu_query: Query<Entity, With<MainMenu>>,
    ) {
        let entity = main_menu_query.single().unwrap();
        commands.entity(entity).despawn();
    }

    pub fn build_main_menu(commands: &mut Commands, asset_server: &Res<AssetServer>) -> Entity {
        let main_menu_entity = commands
            .spawn((
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    row_gap: Val::Px(12.0),
                    column_gap: Val::Px(12.0),
                    ..Default::default()
                },
                //BackgroundColor(Color::linear_rgb(1.0, 0.0, 0.0)),
                MainMenu,
            ))
            .with_children(|parent| {
                // === TITLE ===
                parent
                    .spawn(Node {
                        flex_direction: FlexDirection::Row,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        width: Val::Px(300.0),
                        height: Val::Px(120.0),
                        margin: UiRect::all(Val::Px(8.0)),
                        row_gap: Val::Px(8.0),
                        column_gap: Val::Px(8.0),
                        ..Default::default()
                    })
                    .with_children(|p| {
                        // Image
                        p.spawn(ImageNode {
                            image: asset_server.load("sprites/star.png"),
                            ..default()
                        });

                        // Text
                        p.spawn(text_title_style("Dash Ball", &asset_server));

                        //Image
                        p.spawn(ImageNode {
                            image: asset_server.load("sprites/star.png"),
                            ..default()
                        });
                    });

                // === Play Button ===
                parent
                    .spawn((
                        Button,
                        button_style(),
                        BackgroundColor(NORMAL_BUTTON_COLOR),
                        PlayButton,
                    ))
                    .with_children(|p| {
                        p.spawn(text_button_style("Play", &asset_server));
                    });

                // === Quit Button ===
                parent
                    .spawn((
                        Button,
                        button_style(),
                        BackgroundColor(NORMAL_BUTTON_COLOR),
                        QuitButton,
                    ))
                    .with_children(|p| {
                        p.spawn(text_button_style("Quit", &asset_server));
                    });
            })
            .id();

        main_menu_entity
    }
}

pub mod interaction {
    use bevy::prelude::*;
}
