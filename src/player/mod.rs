use bevy::prelude::*;

mod attacks;
mod health;
mod movement;
mod player;

pub use attacks::*;
pub use player::*;

pub struct Plugin;
impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(attacks::Plugin)
            .add_plugin(player::Plugin)
            .add_plugin(health::Plugin)
            .add_plugin(movement::Plugin);
    }
}
