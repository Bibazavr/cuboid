use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::random;

// This is the player sprite size.
use crate::assets_loader::SceneAssets;

pub const ENEMY_COUNT: usize = 4;

#[derive(Component)]
pub struct Enemy {}

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, spawn_enemy);
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
