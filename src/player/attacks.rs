use crate::map::collision::*;
use crate::{map::MAP_SIZE, player::*};
use bevy::core::FixedTimestep;
use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

const ATTACKS_RATE_TIME: f64 = 30.0 / 60.0;

#[derive(Component)]
pub struct DespawnAttack;

#[derive(Component)]
pub struct Attack;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Component, Copy)]
enum AttackType {
    Knife,
    Fireball,
}

#[derive(Bundle)]
struct AttackBundle {
    attack_type: AttackType,
    #[bundle]
    pbr: PbrBundle,
}

impl Distribution<AttackType> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> AttackType {
        match rng.gen_range(0..=2) {
            0 => AttackType::Knife,
            1 => AttackType::Fireball,
            _ => AttackType::Knife,
        }
    }
}
pub struct Plugin;
impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(GameState::Play)
                .with_run_criteria(FixedTimestep::step(ATTACKS_RATE_TIME))
                .with_system(attacks),
        )
        .add_system_set(SystemSet::on_update(GameState::Play).with_system(despawn_attacks));
    }
}

fn despawn_attacks(mut commands: Commands, query: Query<Entity, With<DespawnAttack>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

fn attacks(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    query_player: Query<&Transform, With<Player>>,
) {
    if let Ok(&player_transform) = query_player.get_single() {
        let attack_type: AttackType = rand::random();

        let proj_offset_y = player_transform.translation.y + (get_attack_size(attack_type) / 2.);

        let mut proj_spawn_transform = player_transform;
        proj_spawn_transform.translation.y = proj_offset_y;

        let mut rng = rand::thread_rng();
        let mut attack_direction: Vec3 = Vec3::new(
            rng.gen_range(-(MAP_SIZE as f32 / 2.)..=(MAP_SIZE as f32 / 2.)),
            proj_offset_y,
            rng.gen_range(-(MAP_SIZE as f32 / 2.)..=(MAP_SIZE as f32 / 2.)),
        )
        .normalize_or_zero();

        attack_direction.x *= get_attack_speed(attack_type);
        attack_direction.z *= get_attack_speed(attack_type);

        match attack_type {
            AttackType::Knife => {
                commands
                    .spawn_bundle(AttackBundle {
                        attack_type,
                        pbr: PbrBundle {
                            mesh: meshes.add(Mesh::from(shape::Cube {
                                size: get_attack_size(attack_type),
                            })),
                            material: materials.add(Color::DARK_GRAY.into()),
                            transform: proj_spawn_transform,
                            ..Default::default()
                        },
                    })
                    .insert(Velocity {
                        linear: attack_direction,
                        ..Default::default()
                    })
                    .insert(RigidBody::KinematicVelocityBased)
                    .insert(CollisionShape::Sphere {
                        radius: get_attack_size(attack_type) / 2.,
                    })
                    .insert(
                        CollisionLayers::none()
                            .with_groups([Layer::PlayerAttacks, Layer::Player])
                            .with_masks([Layer::Enemies, Layer::World]),
                    )
                    .insert(Attack)
                    .insert(DespawnOutOfBounds);
            }
            AttackType::Fireball => {
                commands
                    .spawn_bundle(AttackBundle {
                        attack_type,
                        pbr: PbrBundle {
                            mesh: meshes.add(Mesh::from(shape::Icosphere {
                                radius: get_attack_size(attack_type),
                                subdivisions: 12,
                            })),
                            material: materials.add(Color::RED.into()),
                            transform: proj_spawn_transform,
                            ..Default::default()
                        },
                    })
                    .insert(Velocity {
                        linear: attack_direction,
                        ..Default::default()
                    })
                    .insert(RigidBody::KinematicVelocityBased)
                    .insert(CollisionShape::Sphere {
                        radius: get_attack_size(attack_type) / 2.,
                    })
                    .insert(
                        CollisionLayers::none()
                            .with_groups([Layer::PlayerAttacks, Layer::Player])
                            .with_masks([Layer::Enemies, Layer::World]),
                    )
                    .insert(Attack)
                    .insert(DespawnOutOfBounds);
            }
        }
    }
}

fn get_attack_speed(attack_type: AttackType) -> f32 {
    match attack_type {
        AttackType::Fireball => 10.,
        AttackType::Knife => 30.,
    }
}

fn get_attack_size(attack_type: AttackType) -> f32 {
    match attack_type {
        AttackType::Fireball => 0.8,
        AttackType::Knife => 0.3,
    }
}
