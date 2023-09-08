use bevy::prelude::*;

use crate::{GameState, MainState};

use self::systems::{add_enemy, enemies_movements, spawn_enemies, tick_add_enemy_timer, enemy_hit_player};

mod systems;

const COUNT: usize = 3;
const MAX_COUNT: usize = 25;
const GROW_TIME: f32 = 2.;
const ENEMY_SIZE: f32 = 64.;
const ENEMY_SPEED: f32 = 150.;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<EnemyState>()
            .init_resource::<AddEnemyTimer>()
            .add_systems(OnEnter(MainState::Game), spawn_enemies)
            .add_systems(
                Update,
                enemies_movements
                    .run_if(in_state(GameState::Game).and_then(in_state(MainState::Game))),
            )
            .add_systems(
                Update,
                (add_enemy, tick_add_enemy_timer, enemy_hit_player).run_if(
                    in_state(GameState::Game)
                        .and_then(in_state(MainState::Game))
                        .and_then(in_state(EnemyState::Spawn)),
                ),
            );
    }
}

#[derive(Component)]
pub struct Enemy {
    pub direction: Vec2,
}

#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum EnemyState {
    #[default]
    Spawn,
    Stop,
}

#[derive(Resource)]
pub struct AddEnemyTimer {
    pub timer: Timer,
}

impl Default for AddEnemyTimer {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(GROW_TIME, TimerMode::Repeating),
        }
    }
}
