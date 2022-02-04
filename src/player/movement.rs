use super::player::*;
use crate::game_state::*;
use bevy::prelude::*;
use heron::prelude::*;
use leafwing_input_manager::prelude::*;
use strum::EnumIter;

pub static PLAYER_SPEED: f32 = 10.;

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, EnumIter)]
pub enum Action {
    // Movement
    Up,
    Down,
    Left,
    Right,
}

pub struct Plugin;
impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(InputManagerPlugin::<Action>::default())
            .add_system_set(SystemSet::on_update(GameState::Play).with_system(movement));
    }
}

fn movement(mut query: Query<(&ActionState<Action>, &mut Velocity), With<Player>>) {
    let (action_state, mut velocity) = query.single_mut();

    let mut direction = Vec3::ZERO;

    if action_state.pressed(Action::Down) {
        direction.z += 1.0;
    }

    if action_state.pressed(Action::Up) {
        direction.z -= 1.0;
    }

    if action_state.pressed(Action::Right) {
        direction.x += 1.0;
    }

    if action_state.pressed(Action::Left) {
        direction.x -= 1.0;
    }

    velocity.linear = direction.normalize_or_zero() * PLAYER_SPEED;
}
