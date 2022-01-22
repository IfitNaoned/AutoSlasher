use crate::player::*;
use strum_macros::*;

pub struct Plugin;
impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugin(InputManagerPlugin::<Action>::default())
        //.add_system_set(SystemSet::on_update(GameState::Play).with_system(movement))
        //.add_system(movement)
        ;
    }
}

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, EnumIter)]
pub enum Action {
    // Movement
    Up,
    Down,
    Left,
    Right,
}

fn movement(mut query: Query<(&ActionState<Action>, &mut Transform), With<Player>>) {
    let (action_state, mut transform) = query.single_mut();

    if action_state.pressed(Action::Up) {
        transform.translation.z += 10.;
    }

    if action_state.pressed(Action::Down) {
        transform.translation.z -= 10.;
    }

    if action_state.pressed(Action::Left) {
        transform.translation.x -= 10.;
    }

    if action_state.pressed(Action::Right) {
        transform.translation.x += 10.;
    }
}
