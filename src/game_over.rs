use crate::{
    game_state::*,
    ui::{score::ScoreEvent, GameTimer},
};
use bevy::prelude::*;

pub const WIN_TIMER_IN_SECONDS: f32 = 1. * 60.;

pub struct PlayerDeadEvent;
pub struct GameTimerEndedEvent;

pub struct Plugin;
impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(GameState::Play)
                .with_system(lose)
                .with_system(win),
        )
        .add_system_set(SystemSet::on_enter(GameState::GameOver).with_system(reset_game_timer))
        .add_event::<PlayerDeadEvent>()
        .add_event::<GameTimerEndedEvent>();
    }
}

fn reset_game_timer(mut game_timer: ResMut<GameTimer>) {
    game_timer.0 = 0.;
}

fn lose(
    mut events: EventReader<PlayerDeadEvent>,
    mut state: ResMut<State<GameState>>,
    mut score_events: EventWriter<ScoreEvent>,
) {
    for PlayerDeadEvent in events.iter() {
        state.set(GameState::GameOver).unwrap();
        score_events.send(ScoreEvent("You lose".to_string()));
    }
}

fn win(
    mut events: EventReader<GameTimerEndedEvent>,
    mut state: ResMut<State<GameState>>,
    mut score_events: EventWriter<ScoreEvent>,
) {
    for event in events.iter() {
        match event {
            GameTimerEndedEvent => {
                state.set(GameState::GameOver).unwrap();
                score_events.send(ScoreEvent("You win".to_string()));
            }
        }
    }
}
