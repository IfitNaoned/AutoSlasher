use crate::game_state::*;
use crate::physics::*;
use bevy::prelude::*;
use heron::prelude::*;
use leafwing_input_manager::prelude::*;
use strum_macros::*;

static PLAYER_SIZE: isize = 1;
static PLAYER_COLLISION_SIZE: f32 = PLAYER_SIZE as f32 / 2.;
pub static PLAYER_SPEED: f32 = 10.;

pub mod attacks;
pub mod health;
pub mod movement;

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, EnumIter)]
pub enum Action {
    // Movement
    Up,
    Down,
    Left,
    Right,
}

#[derive(Component)]
struct Health(i32);

#[derive(Component)]
pub struct Player;

#[derive(Bundle)]
struct PlayerBundle {
    player: Player,
    life: Health,
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
            life: Health(100),
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
        .insert(Velocity {
            ..Default::default()
        })
        .insert(RotationConstraints::lock())
        .insert(PhysicMaterial {
            restitution: 0.,
            friction: 0.0,
            density: 1000.0,
        })
        .insert(RigidBody::Dynamic)
        .insert(CollisionShape::Cuboid {
            half_extends: Vec3::splat(PLAYER_COLLISION_SIZE),
            border_radius: None,
        })
        .insert(
            CollisionLayers::none()
                .with_group(Layer::Player)
                .with_mask(Layer::Enemies)
                .with_mask(Layer::World),
        );
}
