use bevy::{prelude::*, window::PrimaryWindow};
use rand::Rng;

use crate::player::Player;

use super::{AddEnemyTimer, Enemy, EnemyState, COUNT, ENEMY_SIZE, ENEMY_SPEED, MAX_COUNT};

pub fn spawn_enemies(
    mut commands: Commands,
    window: Query<&Window, With<PrimaryWindow>>,
    enemies_positions: Query<&Transform, With<Enemy>>,
    asset_server: Res<AssetServer>,
    mut enemy_state: ResMut<NextState<EnemyState>>,
) {
    let window = window.get_single().unwrap();
    let enemies_positions: Vec<Vec2> = enemies_positions
        .iter()
        .map(|transform| Vec2::new(transform.translation.x, transform.translation.y))
        .collect();
    for _ in 0..COUNT {
        spawn_enemy(
            &mut commands,
            window,
            &enemies_positions,
            &asset_server,
            &mut enemy_state,
        );
    }
}

pub fn tick_add_enemy_timer(mut timer: ResMut<AddEnemyTimer>, time: Res<Time>) {
    timer.timer.tick(time.delta());
}

pub fn add_enemy(
    mut commands: Commands,
    window: Query<&Window, With<PrimaryWindow>>,
    enemies_positions: Query<&Transform, With<Enemy>>,
    asset_server: Res<AssetServer>,
    mut enemy_state: ResMut<NextState<EnemyState>>,
    timer: Res<AddEnemyTimer>,
) {
    if timer.timer.finished() {
        let window = window.get_single().unwrap();
        let enemies_positions: Vec<Vec2> = enemies_positions
            .iter()
            .map(|transform| Vec2::new(transform.translation.x, transform.translation.y))
            .collect();
        spawn_enemy(
            &mut commands,
            window,
            &enemies_positions,
            &asset_server,
            &mut enemy_state,
        );
    }
}

fn spawn_enemy(
    commands: &mut Commands,
    window: &Window,
    enemies_positions: &[Vec2],
    asset_server: &AssetServer,
    enemy_state: &mut NextState<EnemyState>,
) {
    if enemies_positions.len() < MAX_COUNT {
        let width = window.width();
        let height = window.height();
        let x = rand::thread_rng()
            .gen_range((-width / 2. + ENEMY_SIZE / 2.)..=(width / 2. - ENEMY_SIZE / 2.));
        let y = rand::thread_rng().gen_range(
            (-height / 2. + ENEMY_SIZE / 2.)..=(height / 2. - ENEMY_SIZE / 2. - height * 0.1),
        );

        let direction = {
            let x = if rand::random::<bool>() {
                rand::thread_rng().gen_range(-0.9_f32..=-0.1_f32)
            } else {
                rand::thread_rng().gen_range(0.1_f32..=0.9_f32)
            };
            let y = if x < 0. { -1. - x } else { 1. - x };
            Vec2::new(x, y).normalize()
        };
        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(x, y, 0.),
                texture: asset_server.load("sprites/ball_red_large.png"),
                ..default()
            },
            Enemy { direction },
        ));
    } else {
        enemy_state.set(EnemyState::Stop);
    }
}

pub fn enemies_movements(
    mut commands: Commands,
    mut enemies: Query<(&mut Transform, &mut Enemy), With<Enemy>>,
    window: Query<&Window, With<PrimaryWindow>>,
    time: Res<Time>,
    asset_server: Res<AssetServer>,
) {
    let window = window.get_single().unwrap();
    let width = window.width();
    let height = window.height();
    for (mut transform, mut enemy) in enemies.iter_mut() {
        let direction = Vec3::new(enemy.direction.x, enemy.direction.y, 0.);
        let mut translation =
            transform.translation + direction * ENEMY_SPEED * time.delta_seconds();

        let mut direction = enemy.direction;
        let mut direction_changed = false;
        let x_min = -width / 2. + ENEMY_SIZE / 2.;
        let x_max = width / 2. - ENEMY_SIZE / 2.;
        let y_min = -height / 2. + ENEMY_SIZE / 2.;
        let y_max = height / 2. - height * 0.1 - ENEMY_SIZE / 2.;
        if translation.x > x_max || translation.x < x_min {
            direction.x *= -1.;
            direction_changed = true;
        }
        if translation.x > x_max {
            translation.x = x_max;
        } else if translation.x < x_min {
            translation.x = x_min;
        }
        if translation.y > y_max || translation.y < y_min {
            direction.y *= -1.;
            direction_changed = true;
        }
        if translation.y > y_max {
            translation.y = y_max;
        } else if translation.y < y_min {
            translation.y = y_min;
        }

        transform.translation = translation;
        enemy.direction = direction;

        if direction_changed {
            let settings = PlaybackSettings::DESPAWN;
            if rand::random::<bool>() {
                commands.spawn(AudioBundle {
                    source: asset_server.load("audio/pluck_001.ogg"),
                    settings,
                });
            } else {
                commands.spawn(AudioBundle {
                    source: asset_server.load("audio/pluck_002.ogg"),
                    settings,
                });
            }
        }
    }
}

pub fn enemy_hit_player(
    mut commands: Commands,
    player: Query<(&Transform, Entity), With<Player>>,
    enemies: Query<&Transform, With<Enemy>>,
    asset_server: Res<AssetServer>,
) {
    if let Ok((player_transform, player_entity)) = player.get_single() {
        for enemy_transform in enemies.iter() {
            let distance = player_transform.translation.distance(enemy_transform.translation);
            if distance < ENEMY_SIZE {
                commands.entity(player_entity).despawn();
                commands.spawn(AudioBundle {
                    source: asset_server.load("audio/explosionCrunch_000.ogg"),
                    settings: PlaybackSettings::DESPAWN,
                });
            }
        }
    }
}