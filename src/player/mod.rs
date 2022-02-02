use crate::game_state::*;
use crate::physics::*;
use crate::utils::*;
use bevy::prelude::*;
use heron::prelude::*;
use leafwing_input_manager::prelude::*;
use strum::EnumIter;

static PLAYER_SIZE: f32 = 1.0;
static PLAYER_COLLISION_SIZE: f32 = PLAYER_SIZE / 2.;
pub static PLAYER_SPEED: f32 = 10.;
static PLAYER_HEALTH_MAX: i32 = 100;

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
    #[bundle]
    input_manager: InputManagerBundle<Action>,
    #[bundle]
    pbr: PbrBundle,
}

pub struct Plugin;
impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Setup).with_system(spawn_player))
            .add_system_set(
                SystemSet::on_exit(GameState::Play).with_system(despawn_entities::<Player>),
            );
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

fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands
        .spawn_bundle(PlayerBundle {
            input_manager: InputManagerBundle {
                input_map: PlayerBundle::default_input_map(),
                action_state: ActionState::default(),
            },
            pbr: PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Cube {
                    size: PLAYER_SIZE,
                })),
                material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
                transform: Transform::from_xyz(0., PLAYER_SIZE / 2., 0.),
                ..Default::default()
            },
        })
        .insert_bundle((
            Player,
            Health(PLAYER_HEALTH_MAX),
            Velocity::default(),
            RotationConstraints::lock(),
            PhysicMaterial {
                restitution: 0.,
                friction: 0.0,
                density: 1000.0,
            },
            RigidBody::Dynamic,
            CollisionShape::Cuboid {
                half_extends: Vec3::splat(PLAYER_COLLISION_SIZE),
                border_radius: None,
            },
            CollisionLayers::none()
                .with_group(Layer::Player)
                .with_mask(Layer::Enemies)
                .with_mask(Layer::World),
        ));
}
