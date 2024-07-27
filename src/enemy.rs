use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use rand::random;

use crate::player::Player;

pub const ENEMY_COUNT: usize = 4;
pub const ENEMY_SPEED: f32 = 5.0;
pub const ENEMY_SIZE: f32 = 1.0;

#[derive(Component)]
pub struct Enemy;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, spawn_enemy)
            .add_systems(Update, move_enemy);
    }
}

pub fn spawn_enemy(mut commands: Commands) {
    for _ in 0..ENEMY_COUNT {
        let random_x = random::<f32>() * 10.0;
        let random_y = random::<f32>() * 10.0;
        let random_z = random::<f32>() * 10.0;
        commands
            .spawn((
                Enemy,
                RigidBody::KinematicPositionBased,
                Collider::ball(ENEMY_SIZE),
            ))
            .insert(GravityScale(0.5))
            .insert(Sleeping::disabled())
            .insert(Ccd::enabled())
            .insert(SpatialBundle {
                transform: Transform::from_xyz(random_x, random_y, random_z),
                ..default()
            })
            .insert(KinematicCharacterController { ..default() });
    }
}

pub fn move_enemy(
    mut enemy_query: Query<(&mut KinematicCharacterController, &Transform), With<Enemy>>,
    player_query: Query<&Transform, (With<Player>, Without<Enemy>)>,
    time: Res<Time>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        for (mut enemy_controller, enemy_transoform) in enemy_query.iter_mut() {
            let mut direction = Vec3::ZERO;

            let player_translation = player_transform.translation;

            let enemy_translation = enemy_transoform.translation;

            if player_translation.x > enemy_translation.x {
                direction += Vec3::new(1.0, 0.0, 0.0);
            } else {
                direction += Vec3::new(-1.0, 0.0, 0.0);
            }

            if player_translation.y > enemy_translation.y {
                direction += Vec3::new(0.0, 1.0, 0.0);
            } else {
                direction += Vec3::new(0.0, -1.0, 0.0);
            }

            if player_translation.z > enemy_translation.z {
                direction += Vec3::new(0.0, 0.0, 1.0);
            } else {
                direction += Vec3::new(0.0, 0.0, -1.0);
            }

            if direction.length() > 0.0 {
                direction = direction.normalize();
            }

            enemy_controller.translation = Some(direction * ENEMY_SPEED * time.delta_seconds());
        }
    }
}
