use crate::game_state::*;
use bevy::prelude::*;

mod camera;
mod enemies;
mod game_state;
mod map;
mod player;
mod ui;

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 0.2,
        })
        .add_plugins(DefaultPlugins)
        //debug plugins
        // init
        .add_state(GameState::Setup)
        // Game plugins
        .add_plugin(camera::Plugin)
        .add_plugin(ui::Plugin)
        .add_plugin(player::Plugin)
        .add_plugin(player::movement::Plugin)
        .add_plugin(enemies::Plugin)
        .add_plugin(enemies::movement::Plugin)
        .add_plugin(map::Plugin)
        .run();
}
