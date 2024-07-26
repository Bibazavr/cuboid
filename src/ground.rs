use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub struct GroundPlugin;

#[derive(Component, Debug)]
pub struct Ground;

impl Plugin for GroundPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, spawn_ground);
    }
}

pub fn spawn_ground(mut commands: Commands) {
    commands
        .spawn((Collider::cuboid(100.0, 0.1, 100.0), Ground))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, -2.0, 0.0)));
}
