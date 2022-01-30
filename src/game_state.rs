#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub enum GameState {
    Menu,
    Setup,
    Play,
    GameOver,
}
