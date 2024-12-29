use crate::components::*;
use bevy::prelude::*;

pub fn spawn_scoreboard(mut commands: Commands) {
    commands.spawn((
        TextBundle::from_section(
            "0",
            TextStyle {
                font_size: 64.0,
                color: Color::WHITE,
                ..default()
            },
        )
        .with_text_justify(JustifyText::Center)
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(5.0),
            right: Val::Percent(45.0),
            ..default()
        }),
        Player1Score,
    ));

    commands.spawn((
        TextBundle::from_section(
            "0",
            TextStyle {
                font_size: 64.0,
                color: Color::WHITE,
                ..default()
            },
        )
        .with_text_justify(JustifyText::Center)
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(5.0),
            left: Val::Percent(45.0),
            ..default()
        }),
        Player2Score,
    ));
}
