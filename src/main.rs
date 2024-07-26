use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::assets_loader::AssetLoaderPlugin;
use crate::camera::MainCameraPlugin;
use crate::enemy::EnemyPlugin;
use crate::ground::GroundPlugin;
use crate::player::PlayerPlugin;

mod assets_loader;
mod camera;
mod enemy;
mod ground;
mod player;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins(AssetLoaderPlugin)
        .add_plugins(MainCameraPlugin)
        .add_plugins(GroundPlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(EnemyPlugin)
        .run();
}
