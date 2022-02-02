use bevy::prelude::*;
use heron::prelude::*;

use crate::{
    game_over::PlayerDeadEvent,
    physics::Layer,
    player::{self, Player},
    GameState,
};
struct PlayerHealthEvent(i32);

pub struct Plugin;
impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_update(GameState::Play).with_system(touch_by_enemy))
            .add_system_set(SystemSet::on_update(GameState::Play).with_system(update_health))
            .add_event::<PlayerHealthEvent>();
    }
}

fn update_health(
    mut events: EventReader<PlayerHealthEvent>,
    mut query: Query<&mut player::Health, With<Player>>,
    mut end_game_event: EventWriter<PlayerDeadEvent>,
) {
    for PlayerHealthEvent(health) in events.iter() {
        let mut player_health = query.single_mut();
        player_health.0 += health;
        if player_health.0 < 1 {
            end_game_event.send(PlayerDeadEvent);
        }
    }
}

fn touch_by_enemy(
    mut events: EventReader<CollisionEvent>,
    mut lose_health_events: EventWriter<PlayerHealthEvent>,
) {
    let mut hit_counter = 0;

    events
        .iter()
        .filter(|e| e.is_started())
        .filter(|event| {
            let (layers_1, layers_2) = event.collision_layers();
            (is_player(layers_1) && is_enemy(layers_2))
                || (is_player(layers_2) && is_enemy(layers_1))
        })
        .for_each(|_| hit_counter += 1);

    lose_health_events.send(PlayerHealthEvent(-hit_counter));
}

fn is_player(layers: CollisionLayers) -> bool {
    layers.contains_group(Layer::Player)
        && !layers.contains_group(Layer::Enemies)
        && !layers.contains_group(Layer::PlayerAttacks)
}

fn is_enemy(layers: CollisionLayers) -> bool {
    !layers.contains_group(Layer::Player) && layers.contains_group(Layer::Enemies)
}
