use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::player::Player;

pub const ENEMY_COUNT: usize = 4;
pub const ENEMY_SPEED: f32 = 10.0;
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
        commands
            .spawn((
                Enemy,
                RigidBody::Dynamic,
                Collider::ball(ENEMY_SIZE),
                Velocity {
                    linvel: Vec3::new(0.0, 0.0, 0.0),
                    angvel: Vec3::new(0.0, 0.0, 0.0),
                },
            ))
            .insert(GravityScale(0.5))
            .insert(Sleeping::disabled())
            .insert(Ccd::enabled());
    }
}

pub fn move_enemy(
    mut enemy_query: Query<&mut Velocity, With<Enemy>>,
    player_query: Query<&mut Velocity, (With<Player>, Without<Enemy>)>,
    time: Res<Time>,
) {
    if let Ok(player_velocity) = player_query.get_single() {
        for mut enemy_velocity in enemy_query.iter_mut() {
            let mut direction = Vec3::ZERO;

            if player_velocity.linvel.x > enemy_velocity.linvel.x {
                direction += Vec3::new(1.0, 0.0, 0.0);
            } else {
                direction += Vec3::new(-1.0, 0.0, 0.0);
            }

            if player_velocity.linvel.y > enemy_velocity.linvel.y {
                direction += Vec3::new(0.0, 1.0, 0.0);
            } else {
                direction += Vec3::new(0.0, -1.0, 0.0);
            }

            if player_velocity.linvel.z > enemy_velocity.linvel.z {
                direction += Vec3::new(0.0, 0.0, 1.0);
            } else {
                direction += Vec3::new(0.0, 0.0, -1.0);
            }

            if direction.length() > 0.0 {
                direction = direction.normalize();
            }

            enemy_velocity.linvel += direction * ENEMY_SPEED * time.delta_seconds();
            enemy_velocity.angvel += direction * ENEMY_SPEED * time.delta_seconds();
        }
    }
}
