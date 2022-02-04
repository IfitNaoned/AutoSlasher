use crate::game_state::*;
use bevy::prelude::*;
use heron::prelude::*;

mod camera;
mod enemies;
mod game_over;
mod game_state;
mod map;
mod physics;
mod player;
mod ui;
mod utils;

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .insert_resource(ClearColor(Color::GRAY))
        .insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 0.2,
        })
        .add_plugins(DefaultPlugins)
        //debug plugins
        // init
        .add_state(GameState::Menu)
        .add_plugin(PhysicsPlugin::default())
        .insert_resource(Gravity::from(Vec3::new(0.0, -9.81, 0.0)))
        // Game plugins
        .add_plugin(camera::Plugin)
        .add_plugin(game_over::Plugin)
        .add_plugin(ui::Plugin)
        .add_plugin(enemies::Plugin)
        .add_plugin(player::Plugin)
        .add_plugin(map::Plugin)
        .run();
}
