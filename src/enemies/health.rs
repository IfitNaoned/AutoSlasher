use crate::enemies::*;

struct EnemyDieEvent();

pub struct Plugin;
impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_update(GameState::Play).with_system(kill_enemy))
            .add_event::<EnemyDieEvent>();
    }
}

fn kill_enemy(
    mut commands: Commands,
    mut events: EventReader<CollisionEvent>,
    mut enemy_dead_events: EventWriter<EnemyDieEvent>,
) {
    events
        .iter()
        .filter(|e| e.is_started())
        .filter_map(|event| {
            let (entity_1, entity_2) = event.rigid_body_entities();
            let (layers_1, layers_2) = event.collision_layers();
            if (is_player(layers_1) && is_enemy(layers_2))
                || (is_player(layers_2) && is_enemy(layers_1))
            {
                Some([entity_1, entity_2])
            } else {
                None
            }
        })
        .for_each(|entities| {
            commands.entity(entities[0]).despawn();
            commands.entity(entities[1]).despawn();
            enemy_dead_events.send(EnemyDieEvent());
        });
}

fn is_player(layers: CollisionLayers) -> bool {
    layers.contains_group(Layer::PlayerAttacks) && !layers.contains_group(Layer::Enemies)
}

fn is_enemy(layers: CollisionLayers) -> bool {
    !layers.contains_group(Layer::PlayerAttacks) && layers.contains_group(Layer::Enemies)
}
