#[derive(Default)]
pub enum GameState {
    #[default]
    PLAY,
    PAUSED,
    DIALOGUE,
}

#[derive(Default)]
pub struct GameStateHandler {
    pub game_state: GameState,
}

impl GameStateHandler {}
