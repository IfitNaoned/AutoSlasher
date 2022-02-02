use crate::enemies::*;
use crate::player::*;

pub static ENEMY_SPEED: f32 = 8.;

pub struct Plugin;
impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_update(GameState::Play).with_system(movement));
    }
}

#[allow(clippy::type_complexity)]
fn movement(
    query_player: Query<&Transform, With<Player>>,
    mut enemies: Query<(&mut Velocity, &Transform), (With<Enemy>, Without<Player>)>,
) {
    if let Ok(player_transform) = query_player.get_single() {
        enemies.for_each_mut(|(mut enemy_velocity, enemy_transform)| {
            let d = player_transform.translation - enemy_transform.translation;

            enemy_velocity.linear = d.normalize_or_zero() * ENEMY_SPEED;
        });
    }
}
