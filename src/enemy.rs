use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::random;

use crate::assets_loader::SceneAssets;
use crate::player::Player;

pub const ENEMY_COUNT: usize = 4;
pub const ENEMY_SPEED: f32 = 400.0;

#[derive(Component)]
pub struct Enemy {}

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, spawn_enemy)
            .add_systems(Update, move_enemy);
    }
}

pub fn spawn_enemy(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    scene_assets: Res<SceneAssets>,
) {
    let window = window_query.get_single().unwrap();

    for _ in 0..ENEMY_COUNT {
        let random_x = random::<f32>() * window.width();
        let random_y = random::<f32>() * window.height();
        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(random_x, random_y, 0.0),
                texture: scene_assets.enemy.clone(),
                ..default()
            },
            Enemy {},
        ));
    }
}

pub fn move_enemy(
    mut enemy_query: Query<&mut Transform, With<Enemy>>,
    player_query: Query<&mut Transform, (With<Player>, Without<Enemy>)>,
    time: Res<Time>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        for mut enemy_transform in enemy_query.iter_mut() {
            let mut direction = Vec3::ZERO;

            if player_transform.translation.x > enemy_transform.translation.x {
                direction += Vec3::new(1.0, 0.0, 0.0);
            } else {
                direction += Vec3::new(-1.0, 0.0, 0.0);
            }

            if (player_transform.translation.y > enemy_transform.translation.y) {
                direction += Vec3::new(0.0, 1.0, 0.0);
            } else {
                direction += Vec3::new(0.0, -1.0, 0.0);
            }

            if direction.length() > 0.0 {
                direction = direction.normalize();
            }

            enemy_transform.translation += direction * ENEMY_SPEED * time.delta_seconds();
        }
    }
}
