use bevy::prelude::*;

mod hud;
mod menu;
mod score;
mod ui;

pub use menu::*;
pub use score::*;
pub use ui::*;

pub struct Plugin;
impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(hud::Plugin)
            .add_plugin(menu::Plugin)
            .add_plugin(score::Plugin)
            .add_plugin(ui::Plugin);
    }
}
