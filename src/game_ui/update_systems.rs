use bevy::{app::AppExit, prelude::*};

use crate::{GameState, MainState, PlayerName, Score};

use super::{
    components::{
        ExitButton, GameBackButton, GamePauseButton, PlayButton, PopupBackButton,
        PopupContinueButton, PopupUserName, ScoresBackButton, ScoresButton, GameScoreText,
    },
    functions, BACK_HOVERED_BUTTON, BACK_NONE_BUTTON, BACK_PRESSED_BUTTON,
    CONTINUE_AND_PAUSE_HOVERED_BUTTON, CONTINUE_AND_PAUSE_NONE_BUTTON,
    CONTINUE_AND_PAUSE_PRESSED_BUTTON, FONT_PATH, HOME_HOVERED_BUTTON, HOME_NONE_BUTTON,
    HOME_PRESSED_BUTTON,
};

pub fn home_play_button(
    interaction: Query<&Interaction, (With<PlayButton>, Changed<Interaction>)>,
    mut background_color: Query<&mut BackgroundColor, With<PlayButton>>,
    mut game_state: ResMut<NextState<MainState>>,
) {
    if let Ok(interaction) = interaction.get_single() {
        if let Ok(mut background_color) = background_color.get_single_mut() {
            functions::changed_state(
                interaction,
                &mut background_color,
                &mut game_state,
                MainState::Popup,
                HOME_PRESSED_BUTTON,
                HOME_HOVERED_BUTTON,
                HOME_NONE_BUTTON,
            );
        }
    }
}

pub fn home_scores_button(
    interaction: Query<&Interaction, (With<ScoresButton>, Changed<Interaction>)>,
    mut background_color: Query<&mut BackgroundColor, With<ScoresButton>>,
    mut game_state: ResMut<NextState<MainState>>,
) {
    if let Ok(interaction) = interaction.get_single() {
        if let Ok(mut background_color) = background_color.get_single_mut() {
            functions::changed_state(
                interaction,
                &mut background_color,
                &mut game_state,
                MainState::Scores,
                HOME_PRESSED_BUTTON,
                HOME_HOVERED_BUTTON,
                HOME_NONE_BUTTON,
            );
        }
    }
}

pub fn home_exit_button(
    interaction: Query<&Interaction, (With<ExitButton>, Changed<Interaction>)>,
    mut background_color: Query<&mut BackgroundColor, With<ExitButton>>,
    mut exit_event: EventWriter<AppExit>,
) {
    if let Ok(interaction) = interaction.get_single() {
        if let Ok(mut background_color) = background_color.get_single_mut() {
            match interaction {
                Interaction::Pressed => {
                    *background_color = HOME_PRESSED_BUTTON;
                    exit_event.send(AppExit);
                }
                Interaction::Hovered => {
                    *background_color = HOME_HOVERED_BUTTON;
                }
                Interaction::None => {
                    *background_color = HOME_NONE_BUTTON;
                }
            }
        }
    }
}

pub fn scores_back_button(
    interaction: Query<&Interaction, (With<ScoresBackButton>, Changed<Interaction>)>,
    mut background_color: Query<&mut BackgroundColor, With<ScoresBackButton>>,
    mut game_state: ResMut<NextState<MainState>>,
) {
    if let Ok(interaction) = interaction.get_single() {
        if let Ok(mut background_color) = background_color.get_single_mut() {
            functions::changed_state(
                interaction,
                &mut background_color,
                &mut game_state,
                MainState::Home,
                BACK_PRESSED_BUTTON,
                BACK_HOVERED_BUTTON,
                BACK_NONE_BUTTON,
            );
        }
    }
}

pub fn game_back_button(
    interaction: Query<&Interaction, (With<GameBackButton>, Changed<Interaction>)>,
    mut background_color: Query<&mut BackgroundColor, With<GameBackButton>>,
    mut game_state: ResMut<NextState<MainState>>,
) {
    if let Ok(interaction) = interaction.get_single() {
        if let Ok(mut background_color) = background_color.get_single_mut() {
            functions::changed_state(
                interaction,
                &mut background_color,
                &mut game_state,
                MainState::Home,
                BACK_PRESSED_BUTTON,
                BACK_HOVERED_BUTTON,
                BACK_NONE_BUTTON,
            );
        }
    }
}

pub fn game_pause_button(
    mut commands: Commands,
    interaction: Query<&Interaction, (With<GamePauseButton>, Changed<Interaction>)>,
    mut background_color: Query<&mut BackgroundColor, With<GamePauseButton>>,
    text_entities: Query<&Children, With<GamePauseButton>>,
    mut game_state: ResMut<NextState<GameState>>,
    asset_server: Res<AssetServer>,
    game_current_state: Res<State<GameState>>,
) {
    if let Ok(interaction) = interaction.get_single() {
        if let Ok(mut background_color) = background_color.get_single_mut() {
            let (new_state, new_text) = if let GameState::Game = game_current_state.get() {
                (GameState::Pause, "Play (P)")
            } else {
                (GameState::Game, "Pause (P)")
            };
            functions::changed_state(
                interaction,
                &mut background_color,
                &mut game_state,
                new_state,
                CONTINUE_AND_PAUSE_PRESSED_BUTTON,
                CONTINUE_AND_PAUSE_HOVERED_BUTTON,
                CONTINUE_AND_PAUSE_NONE_BUTTON,
            );
            if let Interaction::Pressed = interaction {
                if let Ok(text_entities) = text_entities.get_single() {
                    for text in text_entities.iter() {
                        commands.entity(*text).insert(TextBundle::from_section(
                            new_text,
                            TextStyle {
                                font: asset_server.load(FONT_PATH),
                                font_size: 50.,
                                color: Color::BLACK,
                            },
                        ));
                    }
                }
            }
        }
    }
}

pub fn popup_back_button(
    interaction: Query<&Interaction, (With<PopupBackButton>, Changed<Interaction>)>,
    mut background_color: Query<&mut BackgroundColor, With<PopupBackButton>>,
    mut game_state: ResMut<NextState<MainState>>,
) {
    if let Ok(interaction) = interaction.get_single() {
        if let Ok(mut background_color) = background_color.get_single_mut() {
            functions::changed_state(
                interaction,
                &mut background_color,
                &mut game_state,
                MainState::Home,
                BACK_PRESSED_BUTTON,
                BACK_HOVERED_BUTTON,
                BACK_NONE_BUTTON,
            );
        }
    }
}

pub fn popup_continue_button(
    interaction: Query<&Interaction, (With<PopupContinueButton>, Changed<Interaction>)>,
    mut background_color: Query<&mut BackgroundColor, With<PopupContinueButton>>,
    mut game_state: ResMut<NextState<MainState>>,
    player_name: Res<PlayerName>,
) {
    if let Ok(interaction) = interaction.get_single() {
        if let Ok(mut background_color) = background_color.get_single_mut() {
            match interaction {
                Interaction::Pressed => {
                    *background_color = CONTINUE_AND_PAUSE_PRESSED_BUTTON;
                    if !player_name.0.is_empty() {
                        game_state.set(MainState::Game);
                    }
                }
                Interaction::Hovered => {
                    *background_color = CONTINUE_AND_PAUSE_HOVERED_BUTTON;
                }
                Interaction::None => {
                    *background_color = CONTINUE_AND_PAUSE_NONE_BUTTON;
                }
            }
        }
    }
}

pub fn received_char(
    mut text_border: Query<&mut Text, With<PopupUserName>>,
    mut received_character: EventReader<ReceivedCharacter>,
    mut player_name: ResMut<PlayerName>,
    asset_server: Res<AssetServer>,
) {
    for ReceivedCharacter { window: _, char } in received_character.iter() {
        if *char == '\u{8}' {
            player_name.0.pop();
        } else {
            player_name.0.push(*char);
        }
        if let Ok(mut text_border) = text_border.get_single_mut() {
            *text_border = Text::from_section(
                &player_name.0,
                TextStyle {
                    font: asset_server.load(FONT_PATH),
                    font_size: 50.,
                    color: Color::BLACK,
                },
            );
        }
    }
}

pub fn update_score(
    mut text: Query<&mut Text, With<GameScoreText>>,
    score: Res<Score>,
) {
    if let Ok(mut text) = text.get_single_mut() {
        let score = score.score;
        text.sections[0].value = format!("Scores: {score}");
    }
}
