use bevy::prelude::*;

#[derive(Component)]
pub struct HomePage;

#[derive(Component)]
pub struct GamePage;

#[derive(Component)]
pub struct ScoresPage;

#[derive(Component)]
pub struct PopupPage;

#[derive(Component)]
pub struct PlayButton;

#[derive(Component)]
pub struct ScoresButton;

#[derive(Component)]
pub struct ExitButton;

#[derive(Component)]
pub struct PopupUserName;

#[derive(Component)]
pub struct GameScoreText;

#[derive(Component)]
pub struct GameBackButton;

#[derive(Component)]
pub struct GamePauseButton;

#[derive(Component)]
pub struct ScoresBackButton;

#[derive(Component)]
pub struct ScoresList {
    pub position: usize,
}

#[derive(Component)]
pub struct PopupBackButton;

#[derive(Component)]
pub struct PopupContinueButton;