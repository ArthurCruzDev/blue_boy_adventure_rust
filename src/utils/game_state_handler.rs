use std::default;

#[derive(Default)]
pub enum GameState {
    PLAY,
    PAUSED,
    DIALOGUE,
    #[default]
    TITLE,
}

#[derive(Default)]
pub enum TitleScreenSubState {
    #[default]
    MainMenu,
    ClassMenu,
}

#[derive(Default)]
pub struct GameStateHandler {
    pub game_state: GameState,
    pub title_screen_state: TitleScreenSubState,
}

impl GameStateHandler {}
