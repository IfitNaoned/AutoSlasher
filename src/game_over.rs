use crate::{game_state::*, ui::score::ScoreEvent};
use bevy::prelude::*;

pub const WIN_TIMER_IN_SECONDS: f32 = 0.1 * 60.;

pub struct PlayerDeadEvent();
pub struct GameTimerEndedEvent();

pub struct Plugin;
impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_update(GameState::Play).with_system(loose))
            .add_system_set(SystemSet::on_update(GameState::Play).with_system(win))
            .add_event::<PlayerDeadEvent>()
            .add_event::<GameTimerEndedEvent>();
    }
}

fn loose(
    mut events: EventReader<PlayerDeadEvent>,
    mut state: ResMut<State<GameState>>,
    mut score_events: EventWriter<ScoreEvent>,
) {
    for event in events.iter() {
        match event {
            PlayerDeadEvent() => {
                state.set(GameState::GameOver).unwrap();
                score_events.send(ScoreEvent("You loose".to_string()));
            }
        }
    }
}

fn win(
    mut events: EventReader<GameTimerEndedEvent>,
    mut state: ResMut<State<GameState>>,
    mut score_events: EventWriter<ScoreEvent>,
) {
    for event in events.iter() {
        match event {
            GameTimerEndedEvent() => {
                state.set(GameState::GameOver).unwrap();
                score_events.send(ScoreEvent("You win".to_string()));
            }
        }
    }
}
