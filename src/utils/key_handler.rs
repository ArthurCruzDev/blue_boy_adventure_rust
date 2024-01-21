use ggez::winit::event::VirtualKeyCode;

use super::game_state_handler::{GameState, GameStateHandler};

#[derive(Debug, Default)]
pub struct KeyHandler {
    pub left_pressed: bool,
    pub right_pressed: bool,
    pub up_pressed: bool,
    pub down_pressed: bool,
    pub enter_pressed: bool,
}

impl KeyHandler {
    pub fn handle_key_down(
        &mut self,
        input: ggez::input::keyboard::KeyInput,
        _repeated: bool,
        game_state_handler: &mut GameStateHandler,
    ) {
        match game_state_handler.game_state {
            GameState::PLAY => match input.keycode {
                Some(key) => match key {
                    VirtualKeyCode::A => {
                        self.left_pressed = true;
                    }
                    VirtualKeyCode::W => {
                        self.up_pressed = true;
                    }
                    VirtualKeyCode::D => {
                        self.right_pressed = true;
                    }
                    VirtualKeyCode::S => {
                        self.down_pressed = true;
                    }
                    VirtualKeyCode::P => game_state_handler.game_state = GameState::PAUSED,
                    VirtualKeyCode::Return => {
                        self.enter_pressed = true;
                    }
                    _ => {}
                },
                None => {
                    self.left_pressed = false;
                    self.right_pressed = false;
                    self.up_pressed = false;
                    self.down_pressed = false;
                    self.enter_pressed = false;
                }
            },
            GameState::PAUSED => match input.keycode {
                Some(key) => match key {
                    VirtualKeyCode::P => game_state_handler.game_state = GameState::PLAY,
                    _ => {}
                },
                None => {}
            },
            GameState::DIALOGUE => match input.keycode {
                Some(key) => match key {
                    VirtualKeyCode::Return => game_state_handler.game_state = GameState::PLAY,
                    _ => {}
                },
                None => {}
            },
        }
    }

    pub fn handle_key_up(&mut self, input: ggez::input::keyboard::KeyInput) {
        match input.keycode {
            Some(key) => match key {
                VirtualKeyCode::A => {
                    self.left_pressed = false;
                }
                VirtualKeyCode::W => {
                    self.up_pressed = false;
                }
                VirtualKeyCode::D => {
                    self.right_pressed = false;
                }
                VirtualKeyCode::S => {
                    self.down_pressed = false;
                }
                VirtualKeyCode::Return => {
                    self.enter_pressed = false;
                }
                _ => {}
            },
            None => {
                self.left_pressed = false;
                self.right_pressed = false;
                self.up_pressed = false;
                self.down_pressed = false;
                self.enter_pressed = false;
            }
        }
    }
}
