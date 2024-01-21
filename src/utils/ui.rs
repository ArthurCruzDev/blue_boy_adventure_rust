use crate::{entities::player::Player, SCALE, SCREEN_HEIGHT, SCREEN_WIDTH, TILE_SIZE};

use chrono::{Local, NaiveDateTime};
use ggez::{
    glam::Vec2,
    graphics::{
        self, Canvas, Color, DrawParam, FillOptions, Image, Mesh, MeshBuilder, PxScale, Quad, Rect,
        StrokeOptions, Text, TextFragment, TextLayout,
    },
    Context,
};
use log::info;
use rand::Fill;

use super::game_state_handler::{GameState, GameStateHandler};

pub struct UIHandler {
    pub message_on: bool,
    message: String,
    message_draw_param: DrawParam,
    message_counter: NaiveDateTime,
    pub game_finished: bool,
    pub current_dialogue: String,
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
            current_dialogue: String::default(),
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
        ctx: &mut Context,
        player: &Player,
        game_state_handler: &GameStateHandler,
    ) {
        match game_state_handler.game_state {
            GameState::PLAY => self.draw_play_state(canvas, player, game_state_handler),
            GameState::PAUSED => self.draw_paused_state(canvas, player, game_state_handler),
            GameState::DIALOGUE => {
                self.draw_dialogue_state(canvas, ctx, player, game_state_handler)
            }
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
    fn draw_dialogue_state(
        &mut self,
        canvas: &mut Canvas,
        ctx: &mut Context,
        player: &Player,
        game_state_handler: &GameStateHandler,
    ) {
        let mut x = TILE_SIZE as f32 * 2.0;
        let mut y = TILE_SIZE as f32 / 2.0;
        let width = SCREEN_WIDTH as f32 - (TILE_SIZE as f32 * 4.0);
        let height = TILE_SIZE as f32 * 4.0;

        self.draw_sub_window(x, y, width, height, canvas, ctx);

        x += TILE_SIZE as f32 / 2.0;
        y += TILE_SIZE as f32 / 2.0;

        canvas.draw(
            Text::new(TextFragment {
                text: self.current_dialogue.clone(),
                scale: Some(PxScale::from(32.0)),
                color: Some(Color::WHITE),
                font: Some("Maru Monica".to_string()),
            })
            .set_layout(TextLayout {
                h_align: graphics::TextAlign::Begin,
                v_align: graphics::TextAlign::Begin,
            })
            .set_bounds(Vec2 {
                x: width - (TILE_SIZE as f32),
                y: height - (TILE_SIZE as f32),
            }),
            DrawParam::default().dest(Vec2::new(x, y)),
        )
    }

    fn draw_sub_window(
        &mut self,
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        canvas: &mut Canvas,
        ctx: &mut Context,
    ) {
        let background_color = Color::new(0.0, 0.0, 0.0, 0.823);
        let stroke_color = Color::new(1.0, 1.0, 1.0, 1.0);
        let background = Rect::new(x, y, width, height);

        let mesh_data = Mesh::from_data(
            ctx,
            MeshBuilder::new()
                .rounded_rectangle(
                    graphics::DrawMode::Fill(FillOptions::default()),
                    background,
                    15.0,
                    background_color,
                )
                .unwrap()
                .rounded_rectangle(
                    graphics::DrawMode::Stroke(StrokeOptions::default().with_line_width(5.0)),
                    background,
                    15.0,
                    stroke_color,
                )
                .unwrap()
                .build(),
        );

        canvas.draw(&mesh_data, DrawParam::default());
    }
}
