use ggez::{
    graphics::{self, Canvas, Rect},
    Context,
};
use rand::{thread_rng, Rng};

use crate::GameHandlers;

use super::{object::HasObjectData, player::Player};

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
    pub solid_area_default_x: i32,
    pub solid_area_default_y: i32,
    pub action_lock_counter: i32,
    pub dialogues: Vec<String>,
    pub dialogue_index: usize,
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
        }
    }
}

pub trait GameEntity {
    fn update(
        &mut self,
        game_handlers: &mut GameHandlers,
        ctx: &mut Context,
        objects: &mut Vec<Box<dyn HasObjectData>>,
        npcs: &mut Vec<Box<dyn GameEntity>>,
        player: &mut Player,
    );
    fn draw(&self, canvas: &mut Canvas, player: &Player);
    fn set_action(&mut self) {
        self.entity_data_mut().action_lock_counter += 1;

        if self.entity_data().action_lock_counter == 120 {
            let mut rng = thread_rng();
            let random_number: u32 = rng.gen_range(1..101);

            if random_number <= 25 {
                self.entity_data_mut().direction = Direction::Up;
            } else if random_number <= 50 {
                self.entity_data_mut().direction = Direction::Down;
            } else if random_number <= 75 {
                self.entity_data_mut().direction = Direction::Left;
            } else {
                self.entity_data_mut().direction = Direction::Right;
            }
            self.entity_data_mut().action_lock_counter = 0;
        }
    }
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
            Direction::Up => self.entity_data_mut().direction = Direction::Down,
            Direction::Down => self.entity_data_mut().direction = Direction::Up,
            Direction::Left => self.entity_data_mut().direction = Direction::Right,
            Direction::Right => self.entity_data_mut().direction = Direction::Left,
        }
    }
}
