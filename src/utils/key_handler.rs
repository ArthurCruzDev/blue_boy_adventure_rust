use ggez::winit::event::VirtualKeyCode;

use super::game_state_handler::{self, GameState, GameStateHandler};

#[derive(Debug, Default)]
pub struct KeyHandler {
    pub left_pressed: bool,
    pub right_pressed: bool,
    pub up_pressed: bool,
    pub down_pressed: bool,
}

impl KeyHandler {
    pub fn handle_key_down(
        &mut self,
        input: ggez::input::keyboard::KeyInput,
        _repeated: bool,
        game_state_handler: &mut GameStateHandler,
    ) {
        match input.keycode {
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
                VirtualKeyCode::P => match game_state_handler.game_state {
                    GameState::PLAY => {
                        game_state_handler.game_state = GameState::PAUSED;
                    }
                    GameState::PAUSED => game_state_handler.game_state = GameState::PLAY,
                },
                _ => {}
            },
            None => {
                self.left_pressed = false;
                self.right_pressed = false;
                self.up_pressed = false;
                self.down_pressed = false;
            }
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
                _ => {}
            },
            None => {
                self.left_pressed = false;
                self.right_pressed = false;
                self.up_pressed = false;
                self.down_pressed = false;
            }
        }
    }
}
