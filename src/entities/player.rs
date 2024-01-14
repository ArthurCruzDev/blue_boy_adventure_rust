use ggez::{
    glam::Vec2,
    graphics::{self, Image, PxScale},
    Context,
};
use log::info;

use crate::{entities::entity::GameEntity, key_handler::key_handler::KeyHandler, SCALE, TILE_SIZE};

use super::entity::{Direction, EntityData};
pub struct Player {
    entity: EntityData,
}

impl Default for Player {
    fn default() -> Self {
        Player {
            entity: EntityData {
                speed: 4,
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
}

impl GameEntity for Player {
    fn update(&mut self, key_handler: &KeyHandler) {
        if key_handler.left_pressed
            || key_handler.right_pressed
            || key_handler.down_pressed
            || key_handler.up_pressed
        {
            if key_handler.left_pressed {
                self.entity.x -= self.entity.speed;
                self.entity.direction = Direction::Left;
            } else if key_handler.right_pressed {
                self.entity.x += self.entity.speed;
                self.entity.direction = Direction::Right;
            } else if key_handler.up_pressed {
                self.entity.y -= self.entity.speed;
                self.entity.direction = Direction::Up;
            } else if key_handler.down_pressed {
                self.entity.y += self.entity.speed;
                self.entity.direction = Direction::Down;
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

    fn draw(&self, ctx: &Context, canvas: &mut ggez::graphics::Canvas) {
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
                    .dest(Vec2::new(self.entity.x as f32, self.entity.y as f32))
                    .scale(Vec2::new(SCALE as f32, SCALE as f32)),
            ),
            None => {
                todo!()
            }
        }
    }
}
