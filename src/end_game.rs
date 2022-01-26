use crate::game_state::*;
use bevy::prelude::*;

pub struct EndGameEvent(pub bool);

pub struct Plugin;
impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_update(GameState::Play).with_system(end_game))
            .add_event::<EndGameEvent>();
    }
}

fn end_game(mut events: EventReader<EndGameEvent>, mut state: ResMut<State<GameState>>) {
    for event in events.iter() {
        match event {
            EndGameEvent(victory) => {
                if *victory {
                    //todo victory screen
                    eprintln!("victory");
                } else {
                    //todo loose screen
                    eprintln!("loose");
                }
                state.set(GameState::Finish).unwrap();
            }
        }
    }
}
