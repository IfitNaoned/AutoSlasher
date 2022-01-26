use crate::game_state::*;
use bevy::prelude::*;
use heron::prelude::*;

mod camera;
mod end_game;
mod enemies;
mod game_state;
mod map;
mod physics;
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
        .add_plugin(PhysicsPlugin::default())
        .insert_resource(Gravity::from(Vec3::new(0.0, -9.81, 0.0)))
        // Game plugins
        .add_plugin(camera::Plugin)
        .add_plugin(ui::Plugin)
        .add_plugin(end_game::Plugin)
        .add_plugin(player::Plugin)
        .add_plugin(player::movement::Plugin)
        .add_plugin(player::health::Plugin)
        .add_plugin(enemies::Plugin)
        .add_plugin(enemies::movement::Plugin)
        .add_plugin(map::Plugin)
        .run();
}
