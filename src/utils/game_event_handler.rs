use std::{collections::HashMap, ops::Deref};

use ggez::glam::Vec2;

use crate::{
    entities::{
        entity::Direction,
        game_event::{self, GameEvent},
        player::Player,
    },
    TILE_SIZE,
};

use super::{
    game_state_handler::{GameState, GameStateHandler},
    key_handler::KeyHandler,
    ui::UIHandler,
};

pub struct GameEventHandler {
    game_events: HashMap<String, GameEvent>,
    previous_event_coords: Vec2,
    can_touch_event: bool,
}

impl GameEventHandler {
    pub fn new() -> Self {
        let mut game_events = HashMap::new();
        let mut teleport = GameEvent::default();
        teleport.event_default_x = 27;
        teleport.event_default_y = 16;
        game_events.insert("27:16".to_string(), GameEvent::from(Vec2::new(27.0, 16.0)));
        game_events.insert("23:12".to_string(), GameEvent::from(Vec2::new(23.0, 12.0)));
        GameEventHandler {
            game_events,
            previous_event_coords: Vec2::default(),
            can_touch_event: true,
        }
    }

    pub fn check_event(
        &mut self,
        game_state_handler: &mut GameStateHandler,
        ui_handler: &mut UIHandler,
        key_handler: &mut KeyHandler,
        player: &mut Player,
    ) {
        let x_distance = f32::abs(player.entity.world_x as f32 - self.previous_event_coords.x);
        let y_distance = f32::abs(player.entity.world_y as f32 - self.previous_event_coords.y);

        let distance = f32::max(x_distance, y_distance);
        if distance > TILE_SIZE as f32 {
            self.can_touch_event = true;
        }

        if self.can_touch_event {
            if let Some(game_event) = self.game_events.get_mut(&format!("{}:{}", 27, 16)) {
                if let Some(new_coords) =
                    Self::hit(27, 16, Some(Direction::Right), player, game_event)
                {
                    self.previous_event_coords = new_coords;
                    // Self::teleport(ui_handler, player, game_event);
                    Self::damage_pit(
                        game_state_handler,
                        ui_handler,
                        player,
                        GameState::DIALOGUE,
                        game_event,
                    );
                    self.can_touch_event = false;
                }
            }
            if let Some(game_event) = self.game_events.get_mut(&format!("{}:{}", 23, 12)) {
                if let Some(new_coords) =
                    Self::hit(23, 12, Some(Direction::Right), player, game_event)
                {
                    self.previous_event_coords = new_coords;
                    Self::healing_pool(
                        key_handler,
                        game_state_handler,
                        ui_handler,
                        GameState::DIALOGUE,
                        player,
                        game_event,
                    );
                }
            }
        }
    }

    fn hit(
        event_col: i32,
        event_rol: i32,
        direction: Option<Direction>,
        player: &mut Player,
        game_event: &mut GameEvent,
    ) -> Option<Vec2> {
        player.entity.solid_area.x += player.entity.world_x as f32;
        player.entity.solid_area.y += player.entity.world_y as f32;

        game_event.event_rect.x += event_col as f32 * TILE_SIZE as f32;
        game_event.event_rect.y += event_rol as f32 * TILE_SIZE as f32;

        let mut previous_event_coords: Option<Vec2> = None;

        if player.entity.solid_area.overlaps(&game_event.event_rect) && !game_event.event_done {
            if let Some(direction2) = direction {
                if player.entity.direction == direction2 {
                    previous_event_coords = Some(Vec2::new(
                        player.entity.world_x as f32,
                        player.entity.world_y as f32,
                    ));
                }
            } else {
                previous_event_coords = Some(Vec2::new(
                    player.entity.world_x as f32,
                    player.entity.world_y as f32,
                ));
            }
        }

        player.entity.solid_area.x = player.entity.solid_area_default_x as f32;
        player.entity.solid_area.y = player.entity.solid_area_default_y as f32;

        game_event.event_rect.x = game_event.event_default_x as f32;
        game_event.event_rect.y = game_event.event_default_y as f32;

        previous_event_coords
    }

    fn damage_pit(
        game_state_handler: &mut GameStateHandler,
        ui_handler: &mut UIHandler,
        player: &mut Player,
        game_state: GameState,
        game_event: &mut GameEvent,
    ) {
        game_state_handler.game_state = game_state;
        ui_handler.current_dialogue = "You fall into a pit!".to_string();
        player.entity.life -= 1;
        // game_event.event_done = true;
    }

    fn healing_pool(
        key_handler: &mut KeyHandler,
        game_state_handler: &mut GameStateHandler,
        ui_handler: &mut UIHandler,
        game_state: GameState,
        player: &mut Player,
        game_event: &mut GameEvent,
    ) {
        if key_handler.enter_pressed {
            game_state_handler.game_state = game_state;
            ui_handler.current_dialogue =
                "You drank the the water.\nYour life has been recovered.".to_string();
            player.entity.life = player.entity.max_life;
        }
    }

    fn teleport(ui_handler: &mut UIHandler, player: &mut Player, game_event: &mut GameEvent) {
        ui_handler.current_dialogue = "Teleport!".to_string();
        player.entity.world_x = TILE_SIZE as i32 * 37;
        player.entity.world_y = TILE_SIZE as i32 * 10;
    }
}
