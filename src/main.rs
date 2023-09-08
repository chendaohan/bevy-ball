use bevy::prelude::*;
use enemy::EnemyPlugin;
use game_ui::GameUiPlugin;
use player::PlayerPlugin;
use star::StarPlugin;

mod game_ui;
mod player;
mod enemy;
mod star;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(GameUiPlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(EnemyPlugin)
        .add_plugins(StarPlugin)
        .insert_resource(ClearColor(Color::rgb_u8(0xa7, 0xa5, 0xa5)))
        .add_state::<MainState>()
        .add_state::<GameState>()
        .init_resource::<PlayerName>()
        .init_resource::<Score>()
        .add_systems(Startup, spawn_camera)
        .run();
}

// 生成 Camera2d
fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum MainState {
    #[default]
    Home,
    Game,
    Scores,
    Popup,
}

#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum GameState {
    #[default]
    Game,
    Pause
}

#[derive(Resource, Default)]
pub struct PlayerName(String);

#[derive(Resource, Default)]
pub struct Score {
    pub score: usize,
}