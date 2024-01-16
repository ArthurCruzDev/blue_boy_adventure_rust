use ggez::{
    graphics::{self, Canvas, Rect},
    Context,
};

use crate::{
    tiles::tile::TileManager,
    utils::{
        collision_checker::{self, CollisionChecker},
        key_handler::KeyHandler,
    },
};

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
            solid_area: Rect::default(),
            is_collision_on: false,
        }
    }
}

pub trait GameEntity {
    fn update(
        &mut self,
        key_handler: &KeyHandler,
        collision_checker: &CollisionChecker,
        tile_manager: &TileManager,
    );
    fn draw(&self, ctx: &Context, canvas: &mut Canvas);
}
