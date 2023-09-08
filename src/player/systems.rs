use bevy::{prelude::*, window::PrimaryWindow};

use crate::{star::{Star, STAR_SIZE}, Score};

use super::{Player, PLAYER_SIZE, PLAYER_SPEED};

pub fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("sprites/ball_blue_large.png"),
            ..default()
        },
        Player,
    ));
}

pub fn despawn_player(mut commands: Commands, player: Query<Entity, With<Player>>) {
    if let Ok(player_entity) = player.get_single() {
        commands.entity(player_entity).despawn();
    }
}

pub fn player_movement(
    mut transform: Query<&mut Transform, With<Player>>,
    window: Query<&Window, With<PrimaryWindow>>,
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    if let Ok(mut transfrom) = transform.get_single_mut() {
        let mut translation = transfrom.translation;
        let moved_distance = PLAYER_SPEED * time.delta_seconds();
        if keyboard_input.any_pressed([KeyCode::Up, KeyCode::W]) {
            translation.y += moved_distance;
        }
        if keyboard_input.any_pressed([KeyCode::Down, KeyCode::S]) {
            translation.y -= moved_distance;
        }
        if keyboard_input.any_pressed([KeyCode::Left, KeyCode::A]) {
            translation.x -= moved_distance;
        }
        if keyboard_input.any_pressed([KeyCode::Right, KeyCode::D]) {
            translation.x += moved_distance;
        }
        let window = window.get_single().unwrap();
        let width = window.width();
        let height = window.height();
        if translation.x + PLAYER_SIZE / 2. > width / 2. {
            translation.x = width / 2. - PLAYER_SIZE / 2.;
        } else if translation.x - PLAYER_SIZE / 2. < -width / 2. {
            translation.x = -width / 2. + PLAYER_SIZE / 2.;
        }
        if translation.y + PLAYER_SIZE / 2. > height / 2. - height * 0.1 {
            translation.y = height / 2. - height * 0.1 - PLAYER_SIZE / 2.;
        } else if translation.y - PLAYER_SIZE / 2. < -height / 2. {
            translation.y = -height / 2. + PLAYER_SIZE / 2.;
        }
        transfrom.translation = translation;
    }
}

pub fn player_hit_star(
    mut commands: Commands,
    player: Query<&Transform, With<Player>>,
    stars: Query<(&Transform, Entity), With<Star>>,
    mut score: ResMut<Score>,
    asset_server: Res<AssetServer>,
) {
    if let Ok(player_transform) = player.get_single() {
        let player_translation = player_transform.translation;
        for (star_transform, star_entity) in stars.iter() {
            let star_translation = star_transform.translation;
            let distance = player_translation.distance(star_translation);
            if distance <= PLAYER_SIZE / 2. + STAR_SIZE / 2. {
                commands.entity(star_entity).despawn();
                score.score += 1;
                commands.spawn(AudioBundle {
                    source: asset_server.load("audio/laserLarge_000.ogg"),
                    settings: PlaybackSettings::DESPAWN,
                });
            }
        }
    }
}
