use crate::game_over::*;
use crate::game_state::*;
use bevy::prelude::*;

#[derive(Component)]
pub struct SelectedOption;

#[derive(Default)]
pub struct GameTimer(pub f32);

pub const TEXT_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);
pub const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::BLACK;
const HOVERED_PRESSED_BUTTON: Color = Color::rgb(0.25, 0.65, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

pub struct Plugin;
impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_ui_camera)
            .add_system_set(SystemSet::on_update(GameState::Play).with_system(game_timer))
            .init_resource::<GameTimer>();
    }
}

fn spawn_ui_camera(mut commands: Commands) {
    commands.spawn_bundle(UiCameraBundle::default());
}

#[allow(clippy::all)]
pub fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut UiColor, Option<&SelectedOption>),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color, selected) in interaction_query.iter_mut() {
        *color = match (*interaction, selected) {
            (Interaction::Clicked, _) => PRESSED_BUTTON.into(),
            (Interaction::Hovered, Some(_)) => HOVERED_PRESSED_BUTTON.into(),
            (Interaction::Hovered, None) => HOVERED_BUTTON.into(),
            (Interaction::None, Some(_)) => PRESSED_BUTTON.into(),
            (Interaction::None, None) => NORMAL_BUTTON.into(),
        }
    }
}

fn game_timer(
    time: Res<Time>,
    mut game_timer: ResMut<GameTimer>,
    mut end_game_event: EventWriter<GameTimerEndedEvent>,
) {
    game_timer.0 += time.delta_seconds();

    if game_timer.0 > WIN_TIMER_IN_SECONDS {
        end_game_event.send(GameTimerEndedEvent);
    }
}
