use std::hint;

use ggez::graphics::Rect;
use log::info;

use crate::{
    entities::{entity::Direction, player::Player},
    GameHandlers, TILE_SIZE,
};

use super::{
    game_state_handler::{self, GameState, GameStateHandler},
    key_handler::KeyHandler,
    ui::{self, UIHandler},
};

pub struct GameEventHandler {
    pub event_rect: Rect,
    pub event_default_x: i32,
    pub event_default_y: i32,
}

impl GameEventHandler {
    pub fn new() -> Self {
        GameEventHandler {
            event_rect: Rect::new_i32(23, 23, 2, 2),
            event_default_x: 23,
            event_default_y: 23,
        }
    }

    pub fn checkEvent(
        &mut self,
        game_state_handler: &mut GameStateHandler,
        ui_handler: &mut UIHandler,
        key_handler: &mut KeyHandler,
        player: &mut Player,
    ) {
        if self.hit(27, 16, Some(Direction::Right), player) {
            // self.damage_pit(game_state_handler, ui_handler, player, GameState::DIALOGUE);
            self.teleport(ui_handler, player);
        }
        if self.hit(23, 12, Some(Direction::Up), player) {
            self.healing_pool(
                key_handler,
                game_state_handler,
                ui_handler,
                GameState::DIALOGUE,
                player,
            );
        }
    }

    fn hit(
        &mut self,
        event_col: i32,
        event_rol: i32,
        direction: Option<Direction>,
        player: &mut Player,
    ) -> bool {
        let mut hit = false;

        player.entity.solid_area.x += player.entity.world_x as f32;
        player.entity.solid_area.y += player.entity.world_y as f32;
        self.event_rect.x += event_col as f32 * TILE_SIZE as f32;
        self.event_rect.y += event_rol as f32 * TILE_SIZE as f32;

        if player.entity.solid_area.overlaps(&self.event_rect) {
            if let Some(direction2) = direction {
                if player.entity.direction == direction2 {
                    hit = true;
                }
            } else {
                hit = true;
            }
        }

        player.entity.solid_area.x = player.entity.solid_area_default_x as f32;
        player.entity.solid_area.y = player.entity.solid_area_default_y as f32;

        self.event_rect.x = self.event_default_x as f32;
        self.event_rect.y = self.event_default_y as f32;

        hit
    }

    fn damage_pit(
        &self,
        game_state_handler: &mut GameStateHandler,
        ui_handler: &mut UIHandler,
        player: &mut Player,
        game_state: GameState,
    ) {
        game_state_handler.game_state = game_state;
        ui_handler.current_dialogue = "You fall into a pit!".to_string();
        player.entity.life -= 1;
    }

    fn healing_pool(
        &mut self,
        key_handler: &mut KeyHandler,
        game_state_handler: &mut GameStateHandler,
        ui_handler: &mut UIHandler,
        game_state: GameState,
        player: &mut Player,
    ) {
        if key_handler.enter_pressed {
            game_state_handler.game_state = game_state;
            ui_handler.current_dialogue =
                "You drank the the water.\nYour life has been recovered.".to_string();
            player.entity.life = player.entity.max_life;
        }
    }

    fn teleport(&mut self, ui_handler: &mut UIHandler, player: &mut Player) {
        ui_handler.current_dialogue = "Teleport!".to_string();
        player.entity.world_x = TILE_SIZE as i32 * 37;
        player.entity.world_y = TILE_SIZE as i32 * 10;
    }
}
