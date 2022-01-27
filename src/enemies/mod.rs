use crate::game_state::*;
use crate::map::collision::*;
use crate::map::*;
use crate::physics::*;
use bevy::core::FixedTimestep;
use bevy::prelude::*;
use heron::prelude::*;
use rand::prelude::*;

static ENEMY_SIZE: isize = 1;
const ENEMY_SPAWN_TIME_STEP: f64 = 30.0 / 60.0;
pub static ENEMY_MAX_POSITION: isize = (MAP_SIZE / 2) - ENEMY_SIZE;
pub static ENEMY_MIN_POSITION: isize = -ENEMY_MAX_POSITION;

pub mod health;
pub mod movement;

#[derive(Component)]
pub struct Enemy;

#[derive(Component)]
pub struct DespawnEnemy;

#[derive(Bundle)]
struct EnemyBundle {
    enemy: Enemy,
    #[bundle]
    pbr: PbrBundle,
}

pub struct Plugin;
impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(GameState::Play)
                .with_run_criteria(FixedTimestep::step(ENEMY_SPAWN_TIME_STEP))
                .with_system(spawn),
        )
        .add_system_set(SystemSet::on_update(GameState::Play).with_system(despawn_enemies));
    }
}

fn despawn_enemies(mut commands: Commands, query: Query<Entity, With<DespawnEnemy>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

fn spawn(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands
        .spawn_bundle(EnemyBundle {
            enemy: Enemy,
            pbr: PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Icosphere {
                    radius: ENEMY_SIZE as f32 / 2.,
                    subdivisions: 48,
                })),
                material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
                transform: EnemyBundle::get_random_spawn_position(),
                ..Default::default()
            },
        })
        .insert(Velocity {
            ..Default::default()
        })
        .insert(PhysicMaterial {
            restitution: 0.,
            friction: 0.0,
            density: 1.0,
        })
        .insert(RigidBody::Dynamic)
        .insert(CollisionShape::Sphere {
            radius: ENEMY_SIZE as f32 / 2.,
        })
        .insert(
            CollisionLayers::none()
                .with_group(Layer::Enemies)
                .with_mask(Layer::Enemies)
                .with_mask(Layer::Player)
                .with_mask(Layer::World),
        )
        .insert(DespawnOutOfBounds);
}

impl EnemyBundle {
    fn get_random_spawn_position() -> Transform {
        let mut rng = rand::thread_rng();

        let sides: i32 = rng.gen_range(1..=4);

        match sides {
            1 => Transform::from_xyz(
                rng.gen_range(ENEMY_MIN_POSITION..ENEMY_MAX_POSITION) as f32,
                ENEMY_SIZE as f32 / 2.,
                ENEMY_MIN_POSITION as f32,
            ),
            2 => Transform::from_xyz(
                rng.gen_range(ENEMY_MIN_POSITION..ENEMY_MAX_POSITION) as f32,
                ENEMY_SIZE as f32 / 2.,
                ENEMY_MAX_POSITION as f32,
            ),
            3 => Transform::from_xyz(
                ENEMY_MIN_POSITION as f32,
                ENEMY_SIZE as f32 / 2.,
                rng.gen_range(ENEMY_MIN_POSITION..ENEMY_MAX_POSITION) as f32,
            ),
            4 => Transform::from_xyz(
                ENEMY_MAX_POSITION as f32,
                ENEMY_SIZE as f32 / 2.,
                rng.gen_range(ENEMY_MIN_POSITION..ENEMY_MAX_POSITION) as f32,
            ),
            _ => Transform::from_xyz(
                rng.gen_range(ENEMY_MIN_POSITION..ENEMY_MAX_POSITION) as f32,
                ENEMY_SIZE as f32 / 2.,
                ENEMY_MIN_POSITION as f32,
            ),
        }
    }
}
