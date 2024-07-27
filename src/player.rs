use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub const PLAYER_SPEED: f32 = 10.0;
pub const PLAYER_SIZE: f32 = 1.0;

pub struct PlayerPlugin;

#[derive(Component, Debug)]
pub struct Player;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, spawn_player)
            .add_systems(Update, player_movement);
    }
}

pub fn spawn_player(mut commands: Commands) {
    commands
        .spawn((
            Player,
            RigidBody::KinematicPositionBased,
            Collider::cuboid(PLAYER_SIZE, PLAYER_SIZE, PLAYER_SIZE),
        ))
        .insert(GravityScale(0.5))
        .insert(Sleeping::disabled())
        .insert(Ccd::enabled())
        .insert(SpatialBundle::default())
        .insert(KinematicCharacterController {
            ..KinematicCharacterController::default()
        });
}

pub fn player_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<&mut KinematicCharacterController, With<Player>>,
    time: Res<Time>,
) {
    if let Ok(mut controller) = player_query.get_single_mut() {
        let mut direction = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::ArrowLeft) || keyboard_input.pressed(KeyCode::KeyA) {
            direction += Vec3::new(-1.0, 0.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::ArrowRight) || keyboard_input.pressed(KeyCode::KeyD) {
            direction += Vec3::new(1.0, 0.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::ArrowUp) || keyboard_input.pressed(KeyCode::KeyW) {
            direction += Vec3::new(0.0, 0.0, -1.0);
        }
        if keyboard_input.pressed(KeyCode::ArrowDown) || keyboard_input.pressed(KeyCode::KeyS) {
            direction += Vec3::new(0.0, 0.0, 1.0);
        }

        if direction.length() > 0.0 {
            direction = direction.normalize();
        }

        controller.translation = Some(direction * PLAYER_SPEED * time.delta_seconds());
    }
}
