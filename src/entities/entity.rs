use ggez::{
    glam::Vec2,
    graphics::{self, Canvas, Image, Rect},
    Context,
};
use log::info;
use rand::{thread_rng, Rng};

use crate::{utils::collision_checker, GameHandlers, SCALE, TILE_SIZE};

use super::player::Player;

#[derive(Debug, Default, PartialEq)]
pub enum Direction {
    UP,
    #[default]
    DOWN,
    LEFT,
    RIGHT,
}

#[derive(Debug, Default, PartialEq)]
pub enum EntityType {
    PLAYER,
    NPC,
    #[default]
    MONSTER,
    OBJECT,
}

#[derive(Debug)]
pub struct EntityData {
    pub world_x: i32,
    pub world_y: i32,
    pub speed: i32,
    pub up_1: Option<graphics::Image>,
    pub up_2: Option<graphics::Image>,
    pub down_1: Option<graphics::Image>,
    pub down_2: Option<graphics::Image>,
    pub left_1: Option<graphics::Image>,
    pub left_2: Option<graphics::Image>,
    pub right_1: Option<graphics::Image>,
    pub right_2: Option<graphics::Image>,
    pub direction: Direction,
    pub sprite_counter: u32,
    pub sprite_num: u32,
    pub solid_area: Rect,
    pub is_collision_on: bool,
    pub solid_area_default_x: i32,
    pub solid_area_default_y: i32,
    pub action_lock_counter: i32,
    pub dialogues: Vec<String>,
    pub dialogue_index: usize,
    pub max_life: i32,
    pub life: i32,
    pub image: Option<Image>,
    pub image2: Option<Image>,
    pub image3: Option<Image>,
    pub name: String,
    pub is_collidable: bool,
    pub is_invincible: bool,
    pub invincible_counter: i32,
    pub entity_type: EntityType,
}

impl Default for EntityData {
    fn default() -> Self {
        EntityData {
            world_x: 0,
            world_y: 0,
            speed: 0,
            up_1: None,
            up_2: None,
            down_1: None,
            down_2: None,
            left_1: None,
            left_2: None,
            right_1: None,
            right_2: None,
            direction: Direction::default(),
            sprite_counter: 0,
            sprite_num: 1,
            solid_area: Rect::new(0.0, 0.0, 48.0, 48.0),
            is_collision_on: false,
            solid_area_default_x: 0,
            solid_area_default_y: 0,
            action_lock_counter: 0,
            dialogues: Vec::default(),
            dialogue_index: 0,
            max_life: 0,
            life: 0,
            image: None,
            image2: None,
            image3: None,
            name: "".to_string(),
            is_collidable: false,
            is_invincible: false,
            invincible_counter: 0,
            entity_type: EntityType::default(),
        }
    }
}

impl PartialEq for EntityData {
    fn eq(&self, other: &Self) -> bool {
        self.world_x == other.world_x
            && self.world_y == other.world_y
            && self.speed == other.speed
            && self.solid_area == other.solid_area
            && self.solid_area_default_x == other.solid_area_default_x
            && self.solid_area_default_y == other.solid_area_default_y
            && self.max_life == other.max_life
            && self.name == other.name
    }
}

pub trait GameEntity {
    fn update(&mut self, ctx: &mut Context, game_handlers: &mut GameHandlers, has_collided: bool) {
        self.set_action();
        self.entity_data_mut().is_collision_on = has_collided;

        if !self.entity_data().is_collision_on {
            match self.entity_data().direction {
                Direction::UP => {
                    self.entity_data_mut().world_y -= self.entity_data().speed;
                }
                Direction::DOWN => {
                    self.entity_data_mut().world_y += self.entity_data().speed;
                }
                Direction::LEFT => {
                    self.entity_data_mut().world_x -= self.entity_data().speed;
                }
                Direction::RIGHT => {
                    self.entity_data_mut().world_x += self.entity_data().speed;
                }
            }
        }
        self.entity_data_mut().sprite_counter += 1;

        if self.entity_data().sprite_counter > 12 {
            if self.entity_data().sprite_num == 1 {
                self.entity_data_mut().sprite_num = 2;
            } else {
                self.entity_data_mut().sprite_num = 1;
            }
            self.entity_data_mut().sprite_counter = 0;
        }
    }

    fn draw(&self, canvas: &mut Canvas, player: &Player) {
        let image: Option<&Image> = match self.entity_data().direction {
            Direction::UP => match self.entity_data().sprite_num {
                1 => match &self.entity_data().up_1 {
                    Some(image) => Some(image),
                    None => None,
                },
                2 => match &self.entity_data().up_2 {
                    Some(image) => Some(image),
                    None => None,
                },
                _ => None,
            },
            Direction::DOWN => match self.entity_data().sprite_num {
                1 => match &self.entity_data().down_1 {
                    Some(image) => Some(image),
                    None => None,
                },
                2 => match &self.entity_data().down_2 {
                    Some(image) => Some(image),
                    None => None,
                },
                _ => None,
            },
            Direction::LEFT => match self.entity_data().sprite_num {
                1 => match &self.entity_data().left_1 {
                    Some(image) => Some(image),
                    None => None,
                },
                2 => match &self.entity_data().left_2 {
                    Some(image) => Some(image),
                    None => None,
                },
                _ => None,
            },
            Direction::RIGHT => match self.entity_data().sprite_num {
                1 => match &self.entity_data().right_1 {
                    Some(image) => Some(image),
                    None => None,
                },
                2 => match &self.entity_data().right_2 {
                    Some(image) => Some(image),
                    None => None,
                },
                _ => None,
            },
        };
        let screen_x = self.entity_data().world_x - player.entity.world_x + player.screen_x as i32;
        let screen_y = self.entity_data().world_y - player.entity.world_y + player.screen_y as i32;

        if self.entity_data().world_x + (TILE_SIZE as i32)
            > player.entity.world_x - player.screen_x as i32
            && self.entity_data().world_x - (TILE_SIZE as i32)
                < player.entity.world_x + player.screen_x as i32
            && self.entity_data().world_y + (TILE_SIZE as i32)
                > player.entity.world_y - player.screen_y as i32
            && self.entity_data().world_y - (TILE_SIZE as i32)
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

    fn set_action(&mut self) {}
    fn entity_data(&self) -> &EntityData;
    fn entity_data_mut(&mut self) -> &mut EntityData;
    fn speak(&mut self, game_handlers: &mut GameHandlers, player: &Player) {
        game_handlers.ui_handler.current_dialogue = self
            .entity_data()
            .dialogues
            .get(self.entity_data().dialogue_index)
            .unwrap()
            .to_string();
        self.entity_data_mut().dialogue_index += 1;
        if self.entity_data_mut().dialogue_index == self.entity_data().dialogues.len() {
            self.entity_data_mut().dialogue_index = 0;
        }

        match player.entity.direction {
            Direction::UP => self.entity_data_mut().direction = Direction::DOWN,
            Direction::DOWN => self.entity_data_mut().direction = Direction::UP,
            Direction::LEFT => self.entity_data_mut().direction = Direction::RIGHT,
            Direction::RIGHT => self.entity_data_mut().direction = Direction::LEFT,
        }
    }
}
