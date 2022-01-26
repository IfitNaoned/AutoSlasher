#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub enum GameState {
    Setup,
    Play,
    Finish,
}
