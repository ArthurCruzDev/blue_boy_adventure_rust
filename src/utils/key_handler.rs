use ggez::{winit::event::VirtualKeyCode, Context};

use super::{
    game_state_handler::{GameState, GameStateHandler, TitleScreenSubState},
    sound_handler::{self, SoundHandler},
    ui::UIHandler,
};

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
        ctx: &mut Context,
        game_state_handler: &mut GameStateHandler,
        ui_handler: &mut UIHandler,
        sound_handler: &mut SoundHandler,
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
            GameState::TITLE => match game_state_handler.title_screen_state {
                super::game_state_handler::TitleScreenSubState::MainMenu => {
                    match input.keycode {
                        Some(key) => match key {
                            VirtualKeyCode::W => {
                                ui_handler.command_num -= 1;
                                if ui_handler.command_num < 0 {
                                    ui_handler.command_num = 2;
                                }
                            }
                            VirtualKeyCode::S => {
                                ui_handler.command_num += 1;
                                if ui_handler.command_num > 2 {
                                    ui_handler.command_num = 0;
                                }
                            }
                            VirtualKeyCode::Return => match ui_handler.command_num {
                                0 => {
                                    game_state_handler.title_screen_state =
                                        TitleScreenSubState::ClassMenu;
                                }
                                1 => {}
                                2 => {
                                    ctx.request_quit();
                                }
                                _ => {}
                            },
                            _ => {}
                        },
                        None => {}
                    };
                }
                super::game_state_handler::TitleScreenSubState::ClassMenu => {
                    if let Some(key) = input.keycode {
                        match key {
                            VirtualKeyCode::W => {
                                ui_handler.command_num -= 1;
                                if ui_handler.command_num < 0 {
                                    ui_handler.command_num = 3;
                                }
                            }
                            VirtualKeyCode::S => {
                                ui_handler.command_num += 1;
                                if ui_handler.command_num > 3 {
                                    ui_handler.command_num = 0;
                                }
                            }
                            VirtualKeyCode::Return => match ui_handler.command_num {
                                0 => {
                                    game_state_handler.game_state = GameState::PLAY;
                                    sound_handler.play_music(ctx, 0);
                                }
                                1 => {
                                    game_state_handler.game_state = GameState::PLAY;
                                    sound_handler.play_music(ctx, 0);
                                }
                                2 => {
                                    game_state_handler.game_state = GameState::PLAY;
                                    sound_handler.play_music(ctx, 0);
                                }
                                3 => {
                                    game_state_handler.title_screen_state =
                                        TitleScreenSubState::MainMenu;
                                    ui_handler.command_num = 0;
                                }
                                _ => {}
                            },
                            _ => {}
                        }
                    }
                }
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
