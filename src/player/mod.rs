use crate::game_state::*;
use crate::player::movement::*;
use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

static PLAYER_SIZE: isize = 1;

pub mod movement;

#[derive(Component)]
pub struct Player;

#[derive(Bundle)]
struct PlayerBundle {
    player: Player,
    #[bundle]
    input_manager: InputManagerBundle<Action>,
    #[bundle]
    pbr: PbrBundle,
}

pub struct Plugin;
impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Setup).with_system(spawn));
    }
}

impl PlayerBundle {
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

fn spawn(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands
        .spawn_bundle(PlayerBundle {
            player: Player,
            input_manager: InputManagerBundle {
                input_map: PlayerBundle::default_input_map(),
                action_state: ActionState::default(),
            },
            pbr: PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Cube {
                    size: PLAYER_SIZE as f32,
                })),
                material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
                transform: Transform::from_xyz(0., PLAYER_SIZE as f32 / 2., 0.),
                ..Default::default()
            },
        })
        .insert(Player)
        .insert_bundle(InputManagerBundle::<Action>::default());
}
