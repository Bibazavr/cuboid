mod enemy;
mod player;

use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use enemy::EnemyPlugin;
use player::PlayerPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup_camera)
        .add_plugins(PlayerPlugin)
        .add_plugins(EnemyPlugin)
        .run();
}

fn setup_camera(mut commands: Commands, widow_query: Query<&Window, With<PrimaryWindow>>) {
    let window = widow_query.get_single().unwrap();
    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        ..default()
    });
}
