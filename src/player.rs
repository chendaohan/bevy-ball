use bevy::prelude::*;

use crate::{GameState, MainState};

use self::systems::{despawn_player, player_hit_star, player_movement, spawn_player};

mod systems;

const PLAYER_SPEED: f32 = 500.;
const PLAYER_SIZE: f32 = 64.;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(MainState::Game),
            spawn_player.run_if(in_state(GameState::Game)),
        )
        .add_systems(OnExit(MainState::Game), despawn_player)
        .add_systems(
            Update,
            (player_movement, player_hit_star)
                .run_if(in_state(MainState::Game).and_then(in_state(GameState::Game))),
        );
    }
}

#[derive(Component)]
pub struct Player;
