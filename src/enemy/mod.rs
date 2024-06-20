use bevy::prelude::*;
use system::spawn_enemy;

mod system;
mod components;


pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_enemy);
    }
}
