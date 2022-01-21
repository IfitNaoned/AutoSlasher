use crate::game_state::*;
use bevy::prelude::*;

mod camera;
mod game_state;
mod map;
mod player;

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        //.insert_resource(ClearColor(Color::BLACK))
        .insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 0.2,
        })
        .add_plugins(DefaultPlugins)
        // Init
        .add_state(GameState::Setup)
        // Game plugins
        .add_plugin(camera::Plugin)
        .add_plugin(player::Plugin)
        .add_plugin(player::movement::Plugin)
        .add_plugin(map::Plugin)
        .run();
}
