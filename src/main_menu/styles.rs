use bevy::prelude::*;

pub const NORMAL_BUTTON_COLOR: Color = Color::linear_rgb(0.15, 0.15, 0.15);
pub const HOVERED_BUTTON_COLOR: Color = Color::linear_rgb(0.25, 0.25, 0.25);
pub const PRESSED_BUTTON_COLOR: Color = Color::linear_rgb(0.35, 0.75, 0.35);

pub fn button_style() -> Node {
    Node {
        width: Val::Px(200.0),
        height: Val::Px(80.0),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    }
}

pub fn text_button_style(text: impl Into<String>, asset_server: &Res<AssetServer>) -> impl Bundle {
    (
        Text(text.into()),
        TextLayout::new_with_justify(Justify::Center),
        TextFont {
            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
            font_size: 32.0,
            ..Default::default()
        },
        TextColor::WHITE,
    )
}

pub fn text_title_style(text: impl Into<String>, asset_server: &Res<AssetServer>) -> impl Bundle {
    (
        Text(text.into()),
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
    )
}
