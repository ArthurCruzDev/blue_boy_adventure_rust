use std::default;

use ggez::{
    graphics::{self, Canvas},
    Context,
};

use crate::key_handler::key_handler::KeyHandler;

#[derive(Debug, Default)]
pub enum Direction {
    Up,
    #[default]
    Down,
    Left,
    Right,
}

#[derive(Debug)]
pub struct EntityData {
    pub x: i32,
    pub y: i32,
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
}

impl Default for EntityData {
    fn default() -> Self {
        EntityData {
            x: 0,
            y: 0,
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
        }
    }
}

pub trait GameEntity {
    fn update(&mut self, key_handler: &KeyHandler);
    fn draw(&self, ctx: &Context, canvas: &mut Canvas);
}
