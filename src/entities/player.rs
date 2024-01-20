use chrono::Local;
use ggez::{
    glam::Vec2,
    graphics::{self, Image, Rect},
    Context,
};
use log::info;

use crate::{
    entities::entity::GameEntity,
    utils::{sound_handler::SoundHandler, ui::UIHandler},
    GameHandlers, SCALE, SCREEN_HEIGHT, SCREEN_WIDTH, TILE_SIZE,
};

use super::{
    entity::{Direction, EntityData},
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
        info!("Finished loading player images...")
    }

    fn pick_up_object(
        &mut self,
        ctx: &mut Context,
        index: i32,
        asset_setter: &mut AssetSetter,
        sound_handler: &mut SoundHandler,
        ui: &mut UIHandler,
    ) {
        if index != 999 {}
    }
}

impl GameEntity for Player {
    fn update(&mut self, game_handlers: &mut GameHandlers, ctx: &mut Context) {
        if game_handlers.key_handler.left_pressed
            || game_handlers.key_handler.right_pressed
            || game_handlers.key_handler.down_pressed
            || game_handlers.key_handler.up_pressed
        {
            if game_handlers.key_handler.left_pressed {
                self.entity.direction = Direction::Left;
            } else if game_handlers.key_handler.right_pressed {
                self.entity.direction = Direction::Right;
            } else if game_handlers.key_handler.up_pressed {
                self.entity.direction = Direction::Up;
            } else if game_handlers.key_handler.down_pressed {
                self.entity.direction = Direction::Down;
            }

            self.entity.is_collision_on = false;
            game_handlers
                .collision_checker
                .check_tile(&mut self.entity, &game_handlers.tile_manager);
            let index = game_handlers.collision_checker.check_object(
                &mut self.entity,
                true,
                &mut game_handlers.asset_setter.current_objects,
            );

            self.pick_up_object(
                ctx,
                index,
                &mut game_handlers.asset_setter,
                &mut game_handlers.sound_handler,
                &mut game_handlers.ui_handler,
            );

            if !self.entity.is_collision_on {
                match self.entity.direction {
                    Direction::Up => {
                        self.entity.world_y -= self.entity.speed;
                    }
                    Direction::Down => {
                        self.entity.world_y += self.entity.speed;
                    }
                    Direction::Left => {
                        self.entity.world_x -= self.entity.speed;
                    }
                    Direction::Right => {
                        self.entity.world_x += self.entity.speed;
                    }
                }
            }
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
    }

    fn draw(&self, canvas: &mut ggez::graphics::Canvas) {
        let image: Option<&Image> = match self.entity.direction {
            super::entity::Direction::Up => match self.entity.sprite_num {
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
            super::entity::Direction::Down => match self.entity.sprite_num {
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
            super::entity::Direction::Left => match self.entity.sprite_num {
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
            super::entity::Direction::Right => match self.entity.sprite_num {
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
        };
        match image {
            Some(image) => canvas.draw(
                image,
                graphics::DrawParam::new()
                    .dest(Vec2::new(self.screen_x as f32, self.screen_y as f32))
                    .scale(Vec2::new(SCALE as f32, SCALE as f32)),
            ),
            None => {
                todo!()
            }
        }
    }
}
