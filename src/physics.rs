use heron::prelude::*;

#[derive(PhysicsLayer)]
pub enum Layer {
    World,
    Player,
    Enemies,
    PlayerAttacks,
}
