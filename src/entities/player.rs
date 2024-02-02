use ggez::{
    glam::Vec2,
    graphics::{self, Color, Image, Rect},
    Context,
};
use log::info;

use crate::{
    entities::entity::GameEntity,
    utils::{
        collision_checker::check_entity, game_state_handler::GameState,
        sound_handler::SoundHandler, ui::UIHandler,
    },
    GameHandlers, SCALE, SCREEN_HEIGHT, SCREEN_WIDTH, TILE_SIZE,
};

use super::{
    entity::{Direction, EntityData, EntityType},
    objects::asset_setter::AssetSetter,
};
pub struct Player {
    pub entity: EntityData,
    pub screen_x: u32,
    pub screen_y: u32,
}

impl Default for Player {
    fn default() -> Self {
        Player {
            screen_x: (SCREEN_WIDTH / 2) - (TILE_SIZE as u32 / 2),
            screen_y: (SCREEN_HEIGHT / 2) - (TILE_SIZE as u32 / 2),
            entity: EntityData {
                world_x: TILE_SIZE as i32 * 23,
                world_y: TILE_SIZE as i32 * 21,
                speed: 4,
                solid_area: Rect::new(8.0, 16.0, 32.0, 32.0),
                solid_area_default_x: 8,
                solid_area_default_y: 16,
                max_life: 6,
                life: 6,
                entity_type: EntityType::PLAYER,
                attack_area: Rect::new(0.0, 0.0, 36.0, 36.0),
                ..Default::default()
            },
        }
    }
}

impl Player {
    pub fn get_player_images(&mut self, ctx: &mut Context) {
        info!("Loading player images...");
        info!("Loading player up_1 image");
        let up1 = graphics::Image::from_path(ctx, "/player/boy_up_1.png").unwrap();
        self.entity.up_1 = Some(up1);
        info!("Loading player up_2 image");
        let up2 = graphics::Image::from_path(ctx, "/player/boy_up_2.png").unwrap();
        self.entity.up_2 = Some(up2);
        info!("Loading player down_1 image");
        let down1 = graphics::Image::from_path(ctx, "/player/boy_down_1.png").unwrap();
        self.entity.down_1 = Some(down1);
        info!("Loading player down_2 image");
        let down2 = graphics::Image::from_path(ctx, "/player/boy_down_2.png").unwrap();
        self.entity.down_2 = Some(down2);
        info!("Loading player left_1 image");
        let left1 = graphics::Image::from_path(ctx, "/player/boy_left_1.png").unwrap();
        self.entity.left_1 = Some(left1);
        info!("Loading player left_2 image");
        let left2 = graphics::Image::from_path(ctx, "/player/boy_left_2.png").unwrap();
        self.entity.left_2 = Some(left2);
        info!("Loading player right_1 image");
        let right1 = graphics::Image::from_path(ctx, "/player/boy_right_1.png").unwrap();
        self.entity.right_1 = Some(right1);
        info!("Loading player right_2 image");
        let right2 = graphics::Image::from_path(ctx, "/player/boy_right_2.png").unwrap();
        self.entity.right_2 = Some(right2);

        info!("Loading player attack_up_1 image");
        let attack_up_1 = graphics::Image::from_path(ctx, "/player/boy_attack_up_1.png").unwrap();
        self.entity.attack_up_1 = Some(attack_up_1);

        info!("Loading player attack_up_2 image");
        let attack_up_2 = graphics::Image::from_path(ctx, "/player/boy_attack_up_2.png").unwrap();
        self.entity.attack_up_2 = Some(attack_up_2);

        info!("Loading player attack_down_1 image");
        let attack_down_1 =
            graphics::Image::from_path(ctx, "/player/boy_attack_down_1.png").unwrap();
        self.entity.attack_down_1 = Some(attack_down_1);

        info!("Loading player attack_down_2 image");
        let attack_down_2 =
            graphics::Image::from_path(ctx, "/player/boy_attack_down_2.png").unwrap();
        self.entity.attack_down_2 = Some(attack_down_2);

        info!("Loading player attack_left_1 image");
        let attack_left_1 =
            graphics::Image::from_path(ctx, "/player/boy_attack_left_1.png").unwrap();
        self.entity.attack_left_1 = Some(attack_left_1);

        info!("Loading player attack_left_2 image");
        let attack_left_2 =
            graphics::Image::from_path(ctx, "/player/boy_attack_left_2.png").unwrap();
        self.entity.attack_left_2 = Some(attack_left_2);

        info!("Loading player attack_right_1 image");
        let attack_right_1 =
            graphics::Image::from_path(ctx, "/player/boy_attack_right_1.png").unwrap();
        self.entity.attack_right_1 = Some(attack_right_1);

        info!("Loading player attack_right_2 image");
        let attack_right_2 =
            graphics::Image::from_path(ctx, "/player/boy_attack_right_2.png").unwrap();
        self.entity.attack_right_2 = Some(attack_right_2);

        info!("Finished loading player images...")
    }

    pub fn pick_up_object(
        &mut self,
        ctx: &mut Context,
        index: i32,
        asset_setter: &mut AssetSetter,
        sound_handler: &mut SoundHandler,
        ui: &mut UIHandler,
    ) {
        if index != 999 {}
    }

    pub fn interact_npc(&mut self, npc: &mut dyn GameEntity, game_handlers: &mut GameHandlers) {
        if game_handlers.key_handler.enter_pressed {
            game_handlers.game_state_handler.game_state = GameState::DIALOGUE;
            npc.speak(game_handlers, self);
        }
    }

    pub fn interact_monster(
        &mut self,
        monster: &mut dyn GameEntity,
        game_handlers: &mut GameHandlers,
    ) {
        if !self.entity.is_invincible {
            self.entity.life -= 1;
            self.entity.is_invincible = true;
        }
    }

    pub fn attack(&mut self) {
        self.entity.sprite_counter += 1;

        if self.entity.sprite_counter <= 5 {
            self.entity.sprite_num = 1;
        }
        if self.entity.sprite_counter > 5 && self.entity.sprite_counter <= 25 {
            self.entity.sprite_num = 2;
        }
        if self.entity.sprite_counter > 25 {
            self.entity.sprite_num = 1;
            self.entity.sprite_counter = 0;
            self.entity.attacking = false;
        }
    }

    pub fn damage_monster(&mut self, monster: &mut dyn GameEntity) -> bool {
        if !monster.entity_data().is_invincible {
            monster.entity_data_mut().life -= 1;
            monster.entity_data_mut().is_invincible = true;

            if monster.entity_data().life <= 0 {
                return true;
            }
        }
        false
    }
}

impl GameEntity for Player {
    fn update(&mut self, ctx: &mut Context, game_handlers: &mut GameHandlers, has_collided: bool) {
        if self.entity.attacking {
            self.attack();
        } else if game_handlers.key_handler.left_pressed
            || game_handlers.key_handler.right_pressed
            || game_handlers.key_handler.down_pressed
            || game_handlers.key_handler.up_pressed
            || game_handlers.key_handler.enter_pressed
        {
            if game_handlers.key_handler.left_pressed {
                self.entity.direction = Direction::LEFT;
            } else if game_handlers.key_handler.right_pressed {
                self.entity.direction = Direction::RIGHT;
            } else if game_handlers.key_handler.up_pressed {
                self.entity.direction = Direction::UP;
            } else if game_handlers.key_handler.down_pressed {
                self.entity.direction = Direction::DOWN;
            }

            self.entity.is_collision_on = has_collided;

            game_handlers.event_handler.check_event(
                &mut game_handlers.game_state_handler,
                &mut game_handlers.ui_handler,
                &mut game_handlers.key_handler,
                self,
            );

            if !self.entity.is_collision_on && !game_handlers.key_handler.enter_pressed {
                match self.entity.direction {
                    Direction::UP => {
                        self.entity.world_y -= self.entity.speed;
                    }
                    Direction::DOWN => {
                        self.entity.world_y += self.entity.speed;
                    }
                    Direction::LEFT => {
                        self.entity.world_x -= self.entity.speed;
                    }
                    Direction::RIGHT => {
                        self.entity.world_x += self.entity.speed;
                    }
                }
            }

            game_handlers.key_handler.enter_pressed = false;

            self.entity.sprite_counter += 1;

            if self.entity.sprite_counter > 12 {
                if self.entity.sprite_num == 1 {
                    self.entity.sprite_num = 2;
                } else {
                    self.entity.sprite_num = 1;
                }
                self.entity.sprite_counter = 0;
            }
        }
        if self.entity.is_invincible {
            self.entity.invincible_counter += 1;
            if self.entity.invincible_counter > 75 {
                self.entity.is_invincible = false;
                self.entity.invincible_counter = 0;
            }
        }
    }

    fn draw(&self, canvas: &mut ggez::graphics::Canvas, _player: &Player) {
        let mut temp_screen_x = self.screen_x as f32;
        let mut temp_screen_y = self.screen_y as f32;

        let image: Option<&Image> = match self.entity.direction {
            super::entity::Direction::UP => match self.entity.attacking {
                true => {
                    temp_screen_y -= TILE_SIZE as f32;
                    match self.entity.sprite_num {
                        1 => match &self.entity.attack_up_1 {
                            Some(image) => Some(image),
                            None => None,
                        },
                        2 => match &self.entity.attack_up_2 {
                            Some(image) => Some(image),
                            None => None,
                        },
                        _ => None,
                    }
                }
                false => match self.entity.sprite_num {
                    1 => match &self.entity.up_1 {
                        Some(image) => Some(image),
                        None => None,
                    },
                    2 => match &self.entity.up_2 {
                        Some(image) => Some(image),
                        None => None,
                    },
                    _ => None,
                },
            },
            super::entity::Direction::DOWN => match self.entity.attacking {
                true => match self.entity.sprite_num {
                    1 => match &self.entity.attack_down_1 {
                        Some(image) => Some(image),
                        None => None,
                    },
                    2 => match &self.entity.attack_down_2 {
                        Some(image) => Some(image),
                        None => None,
                    },
                    _ => None,
                },
                false => match self.entity.sprite_num {
                    1 => match &self.entity.down_1 {
                        Some(image) => Some(image),
                        None => None,
                    },
                    2 => match &self.entity.down_2 {
                        Some(image) => Some(image),
                        None => None,
                    },
                    _ => None,
                },
            },
            super::entity::Direction::LEFT => match self.entity.attacking {
                true => {
                    temp_screen_x -= TILE_SIZE as f32;
                    match self.entity.sprite_num {
                        1 => match &self.entity.attack_left_1 {
                            Some(image) => Some(image),
                            None => None,
                        },
                        2 => match &self.entity.attack_left_2 {
                            Some(image) => Some(image),
                            None => None,
                        },
                        _ => None,
                    }
                }
                false => match self.entity.sprite_num {
                    1 => match &self.entity.left_1 {
                        Some(image) => Some(image),
                        None => None,
                    },
                    2 => match &self.entity.left_2 {
                        Some(image) => Some(image),
                        None => None,
                    },
                    _ => None,
                },
            },
            super::entity::Direction::RIGHT => match self.entity.attacking {
                true => match self.entity.sprite_num {
                    1 => match &self.entity.attack_right_1 {
                        Some(image) => Some(image),
                        None => None,
                    },
                    2 => match &self.entity.attack_right_2 {
                        Some(image) => Some(image),
                        None => None,
                    },
                    _ => None,
                },
                false => match self.entity.sprite_num {
                    1 => match &self.entity.right_1 {
                        Some(image) => Some(image),
                        None => None,
                    },
                    2 => match &self.entity.right_2 {
                        Some(image) => Some(image),
                        None => None,
                    },
                    _ => None,
                },
            },
        };
        match image {
            Some(image) => canvas.draw(
                image,
                graphics::DrawParam::new()
                    .dest(Vec2::new(temp_screen_x, temp_screen_y))
                    .scale(Vec2::new(SCALE as f32, SCALE as f32))
                    .color(Color::new(
                        1.0,
                        1.0,
                        1.0,
                        if self.entity.is_invincible { 0.7 } else { 1.0 },
                    )),
            ),
            None => {
                todo!()
            }
        }
    }

    fn set_action(&mut self) {}

    fn entity_data(&self) -> &EntityData {
        &self.entity
    }

    fn entity_data_mut(&mut self) -> &mut EntityData {
        &mut self.entity
    }

    fn speak(&mut self, _game_handlers: &mut GameHandlers, _player: &Player) {}
}
