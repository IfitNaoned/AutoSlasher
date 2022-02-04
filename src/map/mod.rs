use bevy::prelude::*;

pub mod collision;
pub mod map;

pub use collision::*;
pub use map::*;

pub struct Plugin;
impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(collision::Plugin);
        app.add_plugin(map::Plugin);
    }
}
