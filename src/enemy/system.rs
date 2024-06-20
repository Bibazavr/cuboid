use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use rand::random;
use crate::enemy::components::Enemy;

pub const ENEMY_COUNT: usize = 4; // This is the player sprite size.

pub fn spawn_enemy(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    for _ in  0..ENEMY_COUNT {
        let random_x = random::<f32>() * window.width();
        let random_y = random::<f32>() * window.height();
        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(random_x, random_y, 0.0),
                texture: asset_server.load("sprites/cuboid_enemy.png", ),
                ..default()
            },
            Enemy {},
        ));
    }
}
