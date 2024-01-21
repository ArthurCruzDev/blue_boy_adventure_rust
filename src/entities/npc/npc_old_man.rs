use ggez::{
    glam::Vec2,
    graphics::{self, Canvas, Image},
    Context,
};
use log::info;

use crate::{
    entities::{
        entity::{Direction, EntityData, GameEntity},
        object::HasObjectData,
        player::Player,
    },
    GameHandlers, SCALE, TILE_SIZE,
};

pub struct NPCOldMan {
    pub screen_x: u32,
    pub screen_y: u32,
    pub entity: EntityData,
}

impl NPCOldMan {
    pub fn new(ctx: &mut Context) -> Self {
        let mut npc_old_man = NPCOldMan {
            screen_x: 0,
            screen_y: 0,
            entity: EntityData {
                direction: Direction::Down,
                speed: 1,
                ..Default::default()
            },
        };
        npc_old_man.get_npcoldman_images(ctx);
        npc_old_man.set_dialogue();
        npc_old_man
    }
    pub fn get_npcoldman_images(&mut self, ctx: &mut Context) {
        info!("Loading NPC Old Man images...");
        info!("Loading NPC Old Man up_1 image");
        let up1 = graphics::Image::from_path(ctx, "/npc/oldman_up_1.png").unwrap();
        self.entity.up_1 = Some(up1);
        info!("Loading NPC Old Man up_2 image");
        let up2 = graphics::Image::from_path(ctx, "/npc/oldman_up_2.png").unwrap();
        self.entity.up_2 = Some(up2);
        info!("Loading NPC Old Man down_1 image");
        let down1 = graphics::Image::from_path(ctx, "/npc/oldman_down_1.png").unwrap();
        self.entity.down_1 = Some(down1);
        info!("Loading NPC Old Man down_2 image");
        let down2 = graphics::Image::from_path(ctx, "/npc/oldman_down_2.png").unwrap();
        self.entity.down_2 = Some(down2);
        info!("Loading NPC Old Man left_1 image");
        let left1 = graphics::Image::from_path(ctx, "/npc/oldman_left_1.png").unwrap();
        self.entity.left_1 = Some(left1);
        info!("Loading NPC Old Man left_2 image");
        let left2 = graphics::Image::from_path(ctx, "/npc/oldman_left_2.png").unwrap();
        self.entity.left_2 = Some(left2);
        info!("Loading NPC Old Man right_1 image");
        let right1 = graphics::Image::from_path(ctx, "/npc/oldman_right_1.png").unwrap();
        self.entity.right_1 = Some(right1);
        info!("Loading NPC Old Man right_2 image");
        let right2 = graphics::Image::from_path(ctx, "/npc/oldman_right_2.png").unwrap();
        self.entity.right_2 = Some(right2);
        info!("Finished loading NPC Old Man images...")
    }

    fn set_dialogue(&mut self) {
        self.entity.dialogues = vec![
            String::from("Hello, lad."),
            String::from("So you've como to this island to find the treasure?"),
            String::from(
                "I used to be a great wizard but now... I'm a bit too old for taking adventure.",
            ),
            String::from("Well, good luck on you."),
        ];
    }
}

impl GameEntity for NPCOldMan {
    fn update(
        &mut self,
        game_handlers: &mut GameHandlers,
        _ctx: &mut Context,
        _objects: &mut Vec<Box<dyn HasObjectData>>,
        _npcs: &mut Vec<Box<dyn GameEntity>>,
        player: &mut Player,
    ) {
        self.set_action();
        self.entity.is_collision_on = false;
        game_handlers
            .collision_checker
            .check_tile(&mut self.entity, &game_handlers.tile_manager);

        game_handlers
            .collision_checker
            .check_player(&mut self.entity, player);

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

    fn draw(&self, canvas: &mut Canvas, player: &Player) {
        let image: Option<&Image> = match self.entity.direction {
            Direction::Up => match self.entity.sprite_num {
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
            Direction::Down => match self.entity.sprite_num {
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
            Direction::Left => match self.entity.sprite_num {
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
            Direction::Right => match self.entity.sprite_num {
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
        let screen_x = self.entity.world_x - player.entity.world_x + player.screen_x as i32;
        let screen_y = self.entity.world_y - player.entity.world_y + player.screen_y as i32;

        if self.entity.world_x + (TILE_SIZE as i32) > player.entity.world_x - player.screen_x as i32
            && self.entity.world_x - (TILE_SIZE as i32)
                < player.entity.world_x + player.screen_x as i32
            && self.entity.world_y + (TILE_SIZE as i32)
                > player.entity.world_y - player.screen_y as i32
            && self.entity.world_y - (TILE_SIZE as i32)
                < player.entity.world_y + player.screen_y as i32
        {
            match image {
                Some(image) => canvas.draw(
                    image,
                    graphics::DrawParam::new()
                        .dest(Vec2::new(screen_x as f32, screen_y as f32))
                        .scale(Vec2::new(SCALE as f32, SCALE as f32)),
                ),
                None => {
                    todo!()
                }
            }
        }
    }

    fn entity_data(&self) -> &EntityData {
        &self.entity
    }

    fn entity_data_mut(&mut self) -> &mut EntityData {
        &mut self.entity
    }
}
