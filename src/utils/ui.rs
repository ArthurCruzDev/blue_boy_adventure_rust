use crate::{entities::player::Player, SCALE, SCREEN_HEIGHT, SCREEN_WIDTH, TILE_SIZE};

use chrono::{Local, NaiveDateTime};
use ggez::{
    glam::Vec2,
    graphics::{self, Canvas, Color, DrawParam, Image, PxScale, Text, TextFragment, TextLayout},
    Context,
};

use super::game_state_handler::{GameState, GameStateHandler};

pub struct UIHandler {
    pub message_on: bool,
    message: String,
    message_draw_param: DrawParam,
    message_counter: NaiveDateTime,
    pub game_finished: bool,
    finished_game_draw_param: DrawParam,
    congratulations_draw_param: DrawParam,
    play_time_started: NaiveDateTime,
    play_time_in_game_draw_param: DrawParam,
    play_time_finished_game_draw_param: DrawParam,
    pub play_time_finished: NaiveDateTime,
}

impl UIHandler {
    pub fn new(ctx: &Context) -> Self {
        UIHandler {
            message_on: false,
            message: String::default(),
            message_draw_param: DrawParam::new().dest(Vec2 {
                x: (TILE_SIZE as f32) / 2.0,
                y: (TILE_SIZE as f32) * 5.0,
            }),
            message_counter: NaiveDateTime::default(),
            game_finished: false,
            finished_game_draw_param: DrawParam::new().dest(Vec2 {
                x: (SCREEN_WIDTH as f32 / 2.0),
                y: (SCREEN_HEIGHT as f32 / 2.0) - (TILE_SIZE as f32) * 2.0,
            }),
            congratulations_draw_param: DrawParam::new().dest(Vec2 {
                x: (SCREEN_WIDTH as f32 / 2.0),
                y: (SCREEN_HEIGHT as f32 / 2.0) + (TILE_SIZE as f32) * 1.0,
            }),
            play_time_started: Local::now().naive_local(),
            play_time_in_game_draw_param: DrawParam::new().dest(Vec2 {
                x: (TILE_SIZE as f32) * 11.0,
                y: 35.0,
            }),
            play_time_finished_game_draw_param: DrawParam::new().dest(Vec2 {
                x: (SCREEN_WIDTH as f32 / 2.0),
                y: (SCREEN_HEIGHT as f32 / 2.0) + (TILE_SIZE as f32) * 2.0,
            }),
            play_time_finished: NaiveDateTime::default(),
        }
    }

    pub fn show_message(&mut self, text: String) {
        self.message = text;
        self.message_on = true;
        self.message_counter = Local::now().naive_local();
    }

    pub fn draw(
        &mut self,
        canvas: &mut Canvas,
        player: &Player,
        game_state_handler: &GameStateHandler,
    ) {
        match game_state_handler.game_state {
            GameState::PLAY => self.draw_play_state(canvas, player, game_state_handler),
            GameState::PAUSED => self.draw_paused_state(canvas, player, game_state_handler),
        }
    }

    fn draw_play_state(
        &mut self,
        canvas: &mut Canvas,
        player: &Player,
        game_state_handler: &GameStateHandler,
    ) {
    }

    fn draw_paused_state(
        &mut self,
        canvas: &mut Canvas,
        player: &Player,
        game_state_handler: &GameStateHandler,
    ) {
        canvas.draw(
            Text::new(TextFragment {
                text: "PAUSED".to_string(),
                color: Some(Color::WHITE),
                scale: Some(PxScale::from(80.0)),
                ..Default::default()
            })
            .set_layout(TextLayout {
                h_align: graphics::TextAlign::Middle,
                v_align: graphics::TextAlign::Middle,
            })
            .set_bounds(Vec2 {
                x: f32::INFINITY,
                y: f32::INFINITY,
            }),
            DrawParam::new().dest(Vec2 {
                x: (SCREEN_WIDTH as f32 / 2.0),
                y: (SCREEN_HEIGHT as f32 / 2.0),
            }),
        )
    }
}
