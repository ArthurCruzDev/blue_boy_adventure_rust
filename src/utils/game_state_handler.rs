use std::default;

#[derive(Default)]
pub enum GameState {
    Play,
    Paused,
    Dialogue,
    #[default]
    Title,
    Character,
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
