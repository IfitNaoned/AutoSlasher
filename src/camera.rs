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
    perspective_camera: PerspectiveCameraBundle,
}

pub struct Plugin;
impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Setup).with_system(setup_camera))
            .add_system_set(SystemSet::on_update(GameState::Play).with_system(update_camera));
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
        perspective_camera: PerspectiveCameraBundle {
            transform: Transform::from_xyz(0.0, 30.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..Default::default()
        },
    });
}

fn update_camera(
    mut camera: Query<&mut Transform, With<MainCamera>>,
    player: Query<&Transform, Without<MainCamera>>,
) {
    let tracked_position = player.single().translation;
    if let Ok(mut transform) = camera.get_single_mut() {
        transform.translation = tracked_position + Vec3::new(0.0, 30.0, 10.0);
        transform.look_at(tracked_position, Vec3::Y);
    }
}
