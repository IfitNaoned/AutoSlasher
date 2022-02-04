use super::map::*;
use crate::game_state::*;
use crate::physics::*;
use bevy::prelude::*;
use heron::prelude::*;

#[derive(Component)]
pub struct DespawnOutOfBounds;

const AREA_BOUNDS: f32 = 5.0;

pub struct Plugin;
impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(GameState::Play).with_system(despawn_out_of_bounds),
        )
        .add_system_set(SystemSet::on_update(GameState::Play).with_system(despawn_player_attacks));
    }
}

fn despawn_out_of_bounds(
    mut commands: Commands,
    query: Query<(Entity, &Transform), With<DespawnOutOfBounds>>,
) {
    let min_x = -AREA_BOUNDS + -MAP_SIZE as f32 / 2.0;
    let max_x = AREA_BOUNDS + MAP_SIZE as f32 / 2.0;
    let min_z = -AREA_BOUNDS + -MAP_SIZE as f32 / 2.0;
    let max_z = AREA_BOUNDS + MAP_SIZE as f32 / 2.0;

    query.for_each(|(entity, transform)| {
        let t = &transform.translation;
        if t.x < min_x || t.x > max_x || t.z < min_z || t.z > max_z || t.y < 0. {
            commands.entity(entity).despawn_recursive();
        }
    });
}

fn despawn_player_attacks(mut commands: Commands, mut events: EventReader<CollisionEvent>) {
    events
        .iter()
        .filter(|e| e.is_started())
        .filter_map(|event| {
            let (entity_1, entity_2) = event.rigid_body_entities();
            let (layers_1, layers_2) = event.collision_layers();
            if is_player_attacks(layers_1) && is_world(layers_2) {
                Some(entity_1)
            } else if is_player_attacks(layers_2) && is_world(layers_1) {
                Some(entity_2)
            } else {
                None
            }
        })
        .for_each(|enemy_entity| commands.entity(enemy_entity).despawn());
}

fn is_player_attacks(layers: CollisionLayers) -> bool {
    layers.contains_group(Layer::PlayerAttacks) && !layers.contains_group(Layer::World)
}

fn is_world(layers: CollisionLayers) -> bool {
    !layers.contains_group(Layer::PlayerAttacks) && layers.contains_group(Layer::World)
}
