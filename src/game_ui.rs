use bevy::prelude::*;

use crate::MainState;

use self::{
    startup_systems::{
        despawn_game, despawn_home, despawn_popup, despawn_scores, spawn_game, spawn_home,
        spawn_popup, spawn_scores,
    },
    update_systems::{
        game_back_button, game_pause_button, home_exit_button, home_play_button,
        home_scores_button, popup_back_button, popup_continue_button, received_char,
        scores_back_button, update_score,
    },
};

mod components;
mod functions;
mod startup_systems;
mod update_systems;

// 字体路径
const FONT_PATH: &str = "fonts/SourceHanSansCN-Bold.otf";
// 界面底层背景
const NODE_BACKGROUND: BackgroundColor = BackgroundColor(Color::rgb(
    0xa7_u8 as f32 / 255.,
    0xa5_u8 as f32 / 255.,
    0xa5_u8 as f32 / 255.,
));
// 按钮的配色
const HOME_PRESSED_BUTTON: BackgroundColor = BackgroundColor(Color::rgb(
    0x00_u8 as f32 / 255.,
    0x00_u8 as f32 / 255.,
    0xff_u8 as f32 / 255.,
));
const HOME_HOVERED_BUTTON: BackgroundColor = BackgroundColor(Color::rgb(
    0x1e_u8 as f32 / 255.,
    0x90_u8 as f32 / 255.,
    0xff_u8 as f32 / 255.,
));
const HOME_NONE_BUTTON: BackgroundColor = BackgroundColor(Color::rgb(
    0x87_u8 as f32 / 255.,
    0xce_u8 as f32 / 255.,
    0xfa_u8 as f32 / 255.,
));
const BACK_PRESSED_BUTTON: BackgroundColor = BackgroundColor(Color::rgb(
    0xff_u8 as f32 / 255.,
    0x00_u8 as f32 / 255.,
    0x00_u8 as f32 / 255.,
));
const BACK_HOVERED_BUTTON: BackgroundColor = BackgroundColor(Color::rgb(
    0xff_u8 as f32 / 255.,
    0x45_u8 as f32 / 255.,
    0x00_u8 as f32 / 255.,
));
const BACK_NONE_BUTTON: BackgroundColor = BackgroundColor(Color::rgb(
    0xff_u8 as f32 / 255.,
    0x63_u8 as f32 / 255.,
    0x47_u8 as f32 / 255.,
));
const CONTINUE_AND_PAUSE_PRESSED_BUTTON: BackgroundColor = BackgroundColor(Color::rgb(
    0xff_u8 as f32 / 255.,
    0xff_u8 as f32 / 255.,
    0x00_u8 as f32 / 255.,
));
const CONTINUE_AND_PAUSE_HOVERED_BUTTON: BackgroundColor = BackgroundColor(Color::rgb(
    0xee_u8 as f32 / 255.,
    0xdc_u8 as f32 / 255.,
    0x82_u8 as f32 / 255.,
));
const CONTINUE_AND_PAUSE_NONE_BUTTON: BackgroundColor = BackgroundColor(Color::rgb(
    0xff_u8 as f32 / 255.,
    0xec_u8 as f32 / 255.,
    0x8b_u8 as f32 / 255.,
));

pub struct GameUiPlugin;

impl Plugin for GameUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(MainState::Home), spawn_home)
            .add_systems(OnExit(MainState::Home), despawn_home)
            .add_systems(OnEnter(MainState::Game), spawn_game)
            .add_systems(OnExit(MainState::Game), despawn_game)
            .add_systems(OnEnter(MainState::Scores), spawn_scores)
            .add_systems(OnExit(MainState::Scores), despawn_scores)
            .add_systems(OnEnter(MainState::Popup), spawn_popup)
            .add_systems(OnExit(MainState::Popup), despawn_popup)
            .add_systems(
                Update,
                (home_play_button, home_scores_button, home_exit_button)
                    .run_if(in_state(MainState::Home)),
            )
            .add_systems(
                Update,
                (popup_back_button, popup_continue_button, received_char)
                    .run_if(in_state(MainState::Popup)),
            )
            .add_systems(
                Update,
                scores_back_button.run_if(in_state(MainState::Scores)),
            )
            .add_systems(
                Update,
                (game_back_button, game_pause_button, update_score)
                    .run_if(in_state(MainState::Game)),
            );
    }
}
