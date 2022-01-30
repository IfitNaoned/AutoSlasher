use crate::game_state::*;
use crate::player::*;
use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

#[derive(Component)]
pub struct MainCamera;

#[derive(Bundle)]
struct MainCameraBundle {
    camera: MainCamera,
    #[bundle]
    input_manager: InputManagerBundle<Action>,
    #[bundle]
    pbr: PerspectiveCameraBundle,
}

pub struct Plugin;
impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Setup).with_system(setup_camera))
            .add_system_set(SystemSet::on_update(GameState::Play).with_system(track_player))
            .add_system_set(SystemSet::on_update(GameState::Play).with_system(update_camera))
            .add_event::<PlayerCameraEvent>();
    }
}

impl MainCameraBundle {
    fn default_input_map() -> InputMap<Action> {
        use Action::*;
        let mut input_map = InputMap::default();
        // Movement
        input_map.insert(Up, KeyCode::Up);
        input_map.insert(Down, KeyCode::Down);
        input_map.insert(Left, KeyCode::Left);
        input_map.insert(Right, KeyCode::Right);
        input_map
    }
}

fn setup_camera(mut commands: Commands) {
    commands.spawn_bundle(MainCameraBundle {
        camera: MainCamera,
        input_manager: InputManagerBundle {
            input_map: MainCameraBundle::default_input_map(),
            action_state: ActionState::default(),
        },
        pbr: PerspectiveCameraBundle {
            transform: Transform::from_xyz(0.0, 30.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..Default::default()
        },
    });
}

struct PlayerCameraEvent(Vec3);

fn track_player(
    mut events: EventWriter<PlayerCameraEvent>,
    query: Query<&Transform, With<Player>>,
) {
    let player_transform = query.single();
    events.send(PlayerCameraEvent(player_transform.translation));
}

fn update_camera(
    mut events: EventReader<PlayerCameraEvent>,
    mut query: Query<&mut Transform, With<MainCamera>>,
) {
    for event in events.iter() {
        match event {
            PlayerCameraEvent(translation) => {
                if let Ok(mut transform) = query.get_single_mut() {
                    transform.translation = *translation + Vec3::new(0.0, 30.0, 10.0);
                    transform.look_at(*translation, Vec3::Y);
                }
            }
        }
    }
}
