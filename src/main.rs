use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::assets_loader::AssetLoaderPlugin;
use crate::enemy::EnemyPlugin;
use crate::player::PlayerPlugin;

mod assets_loader;
mod enemy;
mod player;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(AssetLoaderPlugin)
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
