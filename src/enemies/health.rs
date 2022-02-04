use super::enemies::*;
use crate::game_state::*;
use crate::physics::*;
use crate::player::*;
use bevy::prelude::*;
use heron::prelude::*;

struct EnemyDieEvent();

pub struct Plugin;
impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(GameState::Play)
                .with_system(kill_enemy)
                .label("kill_enemy"),
        )
        .add_event::<EnemyDieEvent>();
    }
}

fn kill_enemy(mut commands: Commands, mut events: EventReader<CollisionEvent>) {
    events
        .iter()
        .filter(|e| e.is_started())
        .filter_map(|event| {
            let (entity_1, entity_2) = event.rigid_body_entities();
            let (layers_1, layers_2) = event.collision_layers();
            if is_player_attack(layers_1) && is_enemy(layers_2) {
                Some((entity_2, entity_1))
            } else if is_player_attack(layers_2) && is_enemy(layers_1) {
                Some((entity_1, entity_2))
            } else {
                None
            }
        })
        .for_each(|(entity_1, entity_2)| {
            commands.entity(entity_1).insert(DespawnEnemy);
            commands.entity(entity_2).insert(DespawnAttack);
        });
}

fn is_player_attack(layers: CollisionLayers) -> bool {
    layers.contains_group(Layer::PlayerAttacks) && !layers.contains_group(Layer::Enemies)
}

fn is_enemy(layers: CollisionLayers) -> bool {
    !layers.contains_group(Layer::PlayerAttacks) && layers.contains_group(Layer::Enemies)
}
