use bevy::prelude::*;

mod enemies;
mod health;
mod movement;

pub use enemies::*;

pub struct Plugin;
impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(enemies::Plugin)
            .add_plugin(health::Plugin)
            .add_plugin(movement::Plugin);
    }
}
