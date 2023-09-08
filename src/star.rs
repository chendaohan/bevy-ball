use bevy::prelude::*;

use crate::{GameState, MainState};

use self::systems::{add_star, spawn_stars, tick_add_star_timer};

mod systems;

const COUNT: usize = 8;
const GROW_TIME: f32 = 1.;
pub const STAR_SIZE: f32 = 30.;

pub struct StarPlugin;

impl Plugin for StarPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AddStarTimer>()
            .add_systems(OnEnter(MainState::Game), spawn_stars)
            .add_systems(
                Update,
                (add_star, tick_add_star_timer).run_if(in_state(GameState::Game).and_then(in_state(MainState::Game))),
            );
    }
}

#[derive(Component)]
pub struct Star;

#[derive(Resource)]
pub struct AddStarTimer {
    pub timer: Timer,
}

impl Default for AddStarTimer {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(GROW_TIME, TimerMode::Repeating),
        }
    }
}
