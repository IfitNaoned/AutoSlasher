use crate::game_state::*;
use crate::physics::*;
use bevy::prelude::*;
use heron::prelude::*;

pub static MAP_SIZE: isize = 100;
pub static BORDER_SIZE: isize = 2;

pub struct Plugin;
impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Setup).with_system(create_map));
    }
}

fn create_map(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut state: ResMut<State<GameState>>,
) {
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(
                shape::Plane {
                    size: MAP_SIZE as f32,
                }
                .into(),
            ),
            material: materials.add(Color::GREEN.into()),
            transform: Transform::from_translation(Vec3::new(0., 0., 0.)),
            ..PbrBundle::default()
        })
        .insert(RigidBody::Static)
        .insert(CollisionShape::HeightField {
            size: Vec2::new(MAP_SIZE as f32, MAP_SIZE as f32),
            heights: vec![vec![0., 0., 0., 0., 0.], vec![0., 0., 0., 0., 0.]],
        })
        .insert(
            CollisionLayers::none()
                .with_group(Layer::World)
                .with_mask(Layer::Player)
                .with_mask(Layer::Enemies),
        );

    //top border
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(
                shape::Quad {
                    size: Vec2::new(MAP_SIZE as f32, BORDER_SIZE as f32),
                    ..Default::default()
                }
                .into(),
            ),
            material: materials.add(Color::GREEN.into()),
            transform: Transform::from_translation(Vec3::new(
                0.,
                (BORDER_SIZE / 2) as f32,
                -(MAP_SIZE / 2) as f32,
            )),
            ..PbrBundle::default()
        })
        .insert(RigidBody::Static)
        .insert(CollisionShape::Cuboid {
            half_extends: Vec3::new((MAP_SIZE / 2) as f32, BORDER_SIZE as f32, 0.),
            border_radius: None,
        })
        .insert(
            CollisionLayers::none()
                .with_group(Layer::World)
                .with_mask(Layer::Player)
                .with_mask(Layer::Enemies),
        );

    //bottom border
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(
                shape::Quad {
                    size: Vec2::new(MAP_SIZE as f32, BORDER_SIZE as f32),
                    ..Default::default()
                }
                .into(),
            ),
            material: materials.add(Color::GREEN.into()),
            transform: Transform::from_translation(Vec3::new(
                0.,
                (BORDER_SIZE / 2) as f32,
                (MAP_SIZE / 2) as f32,
            )),
            ..PbrBundle::default()
        })
        .insert(RigidBody::Static)
        .insert(CollisionShape::Cuboid {
            half_extends: Vec3::new((MAP_SIZE / 2) as f32, BORDER_SIZE as f32, 0.),
            border_radius: None,
        })
        .insert(
            CollisionLayers::none()
                .with_group(Layer::World)
                .with_mask(Layer::Player)
                .with_mask(Layer::Enemies),
        );

    //left border
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(
                shape::Quad {
                    size: Vec2::new(MAP_SIZE as f32, BORDER_SIZE as f32),
                    ..Default::default()
                }
                .into(),
            ),
            material: materials.add(Color::GREEN.into()),
            transform: Transform {
                translation: Vec3::new(-(MAP_SIZE / 2) as f32, (BORDER_SIZE / 2) as f32, 0.),
                rotation: Quat::from_rotation_y(std::f32::consts::PI / 2.),
                ..Default::default()
            },
            ..PbrBundle::default()
        })
        .insert(RigidBody::Static)
        .insert(CollisionShape::Cuboid {
            half_extends: Vec3::new((MAP_SIZE / 2) as f32, BORDER_SIZE as f32, 0.),
            border_radius: None,
        })
        .insert(
            CollisionLayers::none()
                .with_group(Layer::World)
                .with_mask(Layer::Player)
                .with_mask(Layer::Enemies),
        );

    //right border
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(
                shape::Quad {
                    size: Vec2::new(MAP_SIZE as f32, BORDER_SIZE as f32),
                    ..Default::default()
                }
                .into(),
            ),
            material: materials.add(Color::GREEN.into()),
            transform: Transform {
                translation: Vec3::new((MAP_SIZE / 2) as f32, (BORDER_SIZE / 2) as f32, 0.),
                rotation: Quat::from_rotation_y(-std::f32::consts::PI / 2.),
                ..Default::default()
            },
            ..PbrBundle::default()
        })
        .insert(RigidBody::Static)
        .insert(CollisionShape::Cuboid {
            half_extends: Vec3::new((MAP_SIZE / 2) as f32, BORDER_SIZE as f32, 0.),
            border_radius: None,
        })
        .insert(
            CollisionLayers::none()
                .with_group(Layer::World)
                .with_mask(Layer::Player)
                .with_mask(Layer::Enemies),
        );

    state.set(GameState::Play).unwrap();
}
