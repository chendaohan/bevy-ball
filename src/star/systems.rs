use bevy::{prelude::*, window::PrimaryWindow};
use rand::Rng;

use super::{AddStarTimer, COUNT, STAR_SIZE, Star};

pub fn spawn_stars(
    mut commands: Commands,
    window: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window.get_single().unwrap();
    for _ in 0..COUNT {
        spawn_star(&mut commands, window, &asset_server);
    }
}

pub fn tick_add_star_timer(mut timer: ResMut<AddStarTimer>, time: Res<Time>) {
     timer.timer.tick(time.delta());
}

pub fn add_star(
    mut commands: Commands,
    window: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    timer: Res<AddStarTimer>,
) {
    if timer.timer.finished() {
        let window = window.get_single().unwrap();
        spawn_star(&mut commands, window, &asset_server);
    }
}

fn spawn_star(
    commands: &mut Commands,
    window: &Window,
    asset_server: &AssetServer,
) {
    let width = window.width();
        let height = window.height();
        let x = rand::thread_rng()
            .gen_range((-width / 2. + STAR_SIZE / 2.)..=(width / 2. - STAR_SIZE / 2.));
        let y = rand::thread_rng().gen_range(
            (-height / 2. + STAR_SIZE / 2.)..=(height / 2. - STAR_SIZE / 2. - height * 0.1),
        );
        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(x, y, 0.),
                texture: asset_server.load("sprites/star.png"),
                ..default()
            },
            Star
        ));
}

