use bevy::prelude::*;

use crate::PlayerName;

use super::{
    components::{
        ExitButton, GameBackButton, GamePage, GamePauseButton, GameScoreText, HomePage, PlayButton,
        PopupBackButton, PopupContinueButton, PopupPage, ScoresBackButton, ScoresButton,
        ScoresPage,
    },
    functions, NODE_BACKGROUND,
};

pub fn spawn_home(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            NodeBundle {
                background_color: NODE_BACKGROUND,
                style: Style {
                    flex_direction: FlexDirection::Column,
                    width: Val::Percent(100.),
                    height: Val::Percent(100.),
                    justify_content: JustifyContent::SpaceAround,
                    align_items: AlignItems::Center,
                    ..default()
                },
                ..default()
            },
            HomePage,
        ))
        .with_children(|parent| {
            functions::home_title(parent, &asset_server);
            functions::home_button(parent, &asset_server, "Play", PlayButton);
            functions::home_button(parent, &asset_server, "Scores", ScoresButton);
            functions::home_button(parent, &asset_server, "Exit", ExitButton);
        });
}

pub fn despawn_home(mut commands: Commands, home_query: Query<Entity, With<HomePage>>) {
    if let Ok(home_entity) = home_query.get_single() {
        commands.entity(home_entity).despawn_recursive();
    }
}

pub fn spawn_game(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            NodeBundle {
                background_color: BackgroundColor(Color::rgba_u8(0x0, 0x0, 0x0, 0x0)),
                style: Style {
                    flex_direction: FlexDirection::Column,
                    width: Val::Percent(100.),
                    height: Val::Percent(100.),
                    justify_content: JustifyContent::FlexStart,
                    align_items: AlignItems::FlexStart,
                    ..default()
                },
                ..default()
            },
            GamePage,
        ))
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    background_color: Color::rgb_u8(0x49, 0xaa, 0xee).into(),
                    style: Style {
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        width: Val::Percent(100.),
                        height: Val::Percent(10.),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                width: Val::Auto,
                                height: Val::Auto,
                                position_type: PositionType::Absolute,
                                left: Val::Px(20.),
                                top: Val::Px(5.),
                                ..default()
                            },
                            ..default()
                        })
                        .with_children(|parent| {
                            functions::other_button(
                                parent,
                                &asset_server,
                                "Back (B)",
                                Color::rgb_u8(0xee, 0x37, 0x37),
                                GameBackButton,
                            );
                            functions::other_button(
                                parent,
                                &asset_server,
                                "Pause (P)",
                                Color::rgb_u8(0xdf, 0xe0, 0x56),
                                GamePauseButton,
                            );
                        });

                    functions::scores_and_game_title(
                        parent,
                        &asset_server,
                        "Score: 0",
                        Some(GameScoreText),
                    );
                });
        });
}

pub fn despawn_game(mut commands: Commands, game_entity: Query<Entity, With<GamePage>>) {
    if let Ok(game_entity) = game_entity.get_single() {
        commands.entity(game_entity).despawn_recursive();
    }
}

pub fn spawn_scores(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            NodeBundle {
                background_color: NODE_BACKGROUND,
                style: Style {
                    flex_direction: FlexDirection::Column,
                    width: Val::Percent(100.),
                    height: Val::Percent(100.),
                    justify_content: JustifyContent::FlexStart,
                    align_items: AlignItems::FlexStart,
                    ..default()
                },
                ..default()
            },
            ScoresPage,
        ))
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    background_color: Color::rgb_u8(0xff, 0xfa, 0xcd).into(),
                    style: Style {
                        width: Val::Percent(100.),
                        height: Val::Percent(15.),
                        justify_content: JustifyContent::FlexStart,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    functions::other_button(
                        parent,
                        &asset_server,
                        "Back",
                        Color::RED,
                        ScoresBackButton,
                    );
                    functions::scores_and_game_title::<ScoresBackButton>(
                        parent,
                        &asset_server,
                        "Scores",
                        None,
                    );
                });
            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(100.),
                        height: Val::Percent(85.),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                width: Val::Percent(100.),
                                height: Val::Percent(100.),
                                flex_direction: FlexDirection::Column,
                                justify_content: JustifyContent::Start,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            ..default()
                        })
                        .with_children(|parent| {
                            // list item
                            functions::list_item(parent, &asset_server, "Test Item");
                        });
                });
        });
}

pub fn despawn_scores(mut commands: Commands, scores_query: Query<Entity, With<ScoresPage>>) {
    if let Ok(scores_entity) = scores_query.get_single() {
        commands.entity(scores_entity).despawn_recursive();
    }
}

pub fn spawn_popup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut player_name: ResMut<PlayerName>,
) {
    player_name.0.clear();
    commands
        .spawn((
            NodeBundle {
                background_color: NODE_BACKGROUND,
                style: Style {
                    width: Val::Percent(100.),
                    height: Val::Percent(100.),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                ..default()
            },
            PopupPage,
        ))
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    background_color: Color::rgb_u8(0xae, 0xee, 0xee).into(),
                    style: Style {
                        width: Val::Percent(50.),
                        height: Val::Percent(50.),
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::SpaceAround,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                width: Val::Percent(100.),
                                height: Val::Percent(50.),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            ..default()
                        })
                        .with_children(|parent| {
                            functions::popup_name(parent, &asset_server);
                            functions::popup_input_border(parent, &asset_server);
                        });
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                width: Val::Percent(100.),
                                height: Val::Percent(50.),
                                justify_content: JustifyContent::SpaceAround,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            ..default()
                        })
                        .with_children(|parent| {
                            functions::other_button(
                                parent,
                                &asset_server,
                                "Back",
                                Color::RED,
                                PopupBackButton,
                            );
                            functions::other_button(
                                parent,
                                &asset_server,
                                "Continue",
                                Color::MIDNIGHT_BLUE,
                                PopupContinueButton,
                            );
                        });
                });
        });
}

pub fn despawn_popup(mut commands: Commands, popup_query: Query<Entity, With<PopupPage>>) {
    if let Ok(popup_entity) = popup_query.get_single() {
        commands.entity(popup_entity).despawn_recursive();
    }
}
