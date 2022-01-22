use crate::game_state::*;
use bevy::prelude::*;

pub static MAP_SIZE: isize = 100;

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
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(
            shape::Plane {
                size: MAP_SIZE as f32,
            }
            .into(),
        ),
        material: materials.add(Color::GREEN.into()),
        transform: Transform::from_translation(Vec3::new(0., -0.1, 0.)),
        ..PbrBundle::default()
    });

    state.set(GameState::Play).unwrap();
}
