use crate::player::*;

pub struct Plugin;
impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(InputManagerPlugin::<Action>::default())
            .add_system_set(SystemSet::on_update(GameState::Play).with_system(movement));
    }
}

fn movement(mut query: Query<(&ActionState<Action>, &mut Transform), With<Player>>) {
    let (action_state, mut transform) = query.single_mut();

    if action_state.pressed(Action::Down) {
        transform.translation.z += 1.;
    }

    if action_state.pressed(Action::Up) {
        transform.translation.z -= 1.;
    }

    if action_state.pressed(Action::Left) {
        transform.translation.x -= 1.;
    }

    if action_state.pressed(Action::Right) {
        transform.translation.x += 1.;
    }
}
