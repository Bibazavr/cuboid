use bevy::prelude::*;

use crate::assets_loader::AssetLoaderPlugin;
use crate::camera::MainCameraPlugin;
use crate::enemy::EnemyPlugin;
use crate::player::PlayerPlugin;

mod assets_loader;
mod camera;
mod enemy;
mod player;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(AssetLoaderPlugin)
        .add_plugins(MainCameraPlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(EnemyPlugin)
        .run();
}
