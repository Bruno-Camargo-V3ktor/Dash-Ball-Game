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
