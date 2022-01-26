use crate::{map::MAP_SIZE, player::*};
use bevy::core::FixedTimestep;
use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

const ATTACKS_RATE_TIME: f64 = 30.0 / 60.0;

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
        );
    }
}

fn attacks(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    query_player: Query<&Transform, With<Player>>,
) {
    let attack_type: AttackType = rand::random();
    if let Ok(&player_transform) = query_player.get_single() {
        let mut rng = rand::thread_rng();
        let attack_direction: Vec3 = Vec3::new(
            rng.gen_range(-(MAP_SIZE as f32 / 2.)..=(MAP_SIZE as f32 / 2.)),
            0.,
            rng.gen_range(-(MAP_SIZE as f32 / 2.)..=(MAP_SIZE as f32 / 2.)),
        )
        .normalize_or_zero()
            * get_attack_speed(attack_type);

        match attack_type {
            AttackType::Knife => {
                commands
                    .spawn_bundle(AttackBundle {
                        attack_type,
                        pbr: PbrBundle {
                            mesh: meshes.add(Mesh::from(shape::Cube { size: 0.3 })),
                            material: materials.add(Color::ALICE_BLUE.into()),
                            transform: player_transform,
                            ..Default::default()
                        },
                    })
                    .insert(Velocity {
                        linear: attack_direction,
                        ..Default::default()
                    })
                    .insert(RigidBody::Dynamic)
                    .insert(CollisionShape::Sphere { radius: 0.3 })
                    .insert(
                        CollisionLayers::none()
                            .with_group(Layer::PlayerAttacks)
                            .with_group(Layer::Player)
                            .with_mask(Layer::Enemies)
                            .with_mask(Layer::World),
                    );
            }
            AttackType::Fireball => {
                commands
                    .spawn_bundle(AttackBundle {
                        attack_type,
                        pbr: PbrBundle {
                            mesh: meshes.add(Mesh::from(shape::Icosphere {
                                radius: 0.3,
                                subdivisions: 12,
                            })),
                            material: materials.add(Color::ALICE_BLUE.into()),
                            transform: player_transform,
                            ..Default::default()
                        },
                    })
                    .insert(Velocity {
                        linear: attack_direction,
                        ..Default::default()
                    })
                    .insert(RigidBody::Dynamic)
                    .insert(CollisionShape::Sphere { radius: 0.3 })
                    .insert(
                        CollisionLayers::none()
                            .with_group(Layer::PlayerAttacks)
                            .with_group(Layer::Player)
                            .with_mask(Layer::Enemies)
                            .with_mask(Layer::World),
                    );
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
