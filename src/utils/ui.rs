use crate::{entities::player::Player, SCALE, SCREEN_HEIGHT, SCREEN_WIDTH, TILE_SIZE};

use chrono::{Duration, Local, NaiveDate, NaiveDateTime, NaiveTime};
use ggez::{
    glam::Vec2,
    graphics::{self, Canvas, Color, DrawParam, Image, PxScale, Text, TextFragment, TextLayout},
    Context,
};
use log::info;

pub struct UIHandler {
    key_text_draw_param: DrawParam,
    key_image: Option<Image>,
    key_image_draw_param: DrawParam,
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
            key_text_draw_param: DrawParam::new().dest(Vec2 { x: 74.0, y: 35.0 }),
            key_image: Some(graphics::Image::from_path(ctx, "/objects/key.png").unwrap()),
            key_image_draw_param: DrawParam::new()
                .dest(Vec2 {
                    x: (TILE_SIZE as f32) / 2.0,
                    y: (TILE_SIZE as f32) / 2.0,
                })
                .scale(Vec2::new(SCALE as f32, SCALE as f32)),
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

    pub fn draw(&mut self, ctx: &mut Context, canvas: &mut Canvas, player: &Player) {
        if self.game_finished {
            canvas.draw(
                Text::new(TextFragment {
                    text: "You found the treasure".to_string(),
                    color: Some(Color::WHITE),
                    font: Some("LiberationMono-Regular".into()),
                    scale: Some(PxScale::from(40.0)),
                })
                .set_layout(TextLayout {
                    h_align: graphics::TextAlign::Middle,
                    v_align: graphics::TextAlign::Middle,
                })
                .set_bounds(Vec2 {
                    x: SCREEN_WIDTH as f32,
                    y: f32::INFINITY,
                }),
                self.finished_game_draw_param,
            );

            canvas.draw(
                Text::new(TextFragment {
                    text: "Congratulations".to_string(),
                    color: Some(Color::YELLOW),
                    font: Some("LiberationMono-Regular".into()),
                    scale: Some(PxScale::from(80.0)),
                })
                .set_layout(TextLayout {
                    h_align: graphics::TextAlign::Middle,
                    v_align: graphics::TextAlign::Middle,
                })
                .set_bounds(Vec2 {
                    x: SCREEN_WIDTH as f32,
                    y: f32::INFINITY,
                }),
                self.congratulations_draw_param,
            );

            canvas.draw(
                Text::new(TextFragment {
                    text: format!(
                        "Your Time is: {:.2}!",
                        (self.play_time_finished - self.play_time_started).num_milliseconds()
                            as f32
                            / 1000.0
                    ),
                    color: Some(Color::WHITE),
                    font: Some("LiberationMono-Regular".into()),
                    scale: Some(PxScale::from(40.0)),
                })
                .set_layout(TextLayout {
                    h_align: graphics::TextAlign::Middle,
                    v_align: graphics::TextAlign::Middle,
                })
                .set_bounds(Vec2 {
                    x: SCREEN_WIDTH as f32,
                    y: f32::INFINITY,
                }),
                self.play_time_finished_game_draw_param,
            );
        } else {
            if let Some(key_image) = &self.key_image {
                canvas.draw(key_image, self.key_image_draw_param);
            }

            canvas.draw(
                &Text::new(TextFragment {
                    text: format!("x {}", player.has_key),
                    color: Some(Color::WHITE),
                    font: Some("LiberationMono-Regular".into()),
                    scale: Some(PxScale::from(40.0)),
                }),
                self.key_text_draw_param,
            );

            canvas.draw(
                &Text::new(TextFragment {
                    text: format!(
                        "Time: {:.2}",
                        (Local::now().naive_local() - self.play_time_started).num_milliseconds()
                            as f32
                            / 1000.0
                    ),
                    color: Some(Color::WHITE),
                    font: Some("LiberationMono-Regular".into()),
                    scale: Some(PxScale::from(40.0)),
                }),
                self.play_time_in_game_draw_param,
            );

            if self.message_on {
                canvas.draw(
                    &Text::new(TextFragment {
                        text: self.message.clone(),
                        color: Some(Color::WHITE),
                        font: Some("LiberationMono-Regular".into()),
                        scale: Some(PxScale::from(30.0)),
                    }),
                    self.message_draw_param,
                );

                if (Local::now().naive_local() - self.message_counter).num_milliseconds() > 2000 {
                    self.message_counter = NaiveDateTime::default();
                    self.message_on = false;
                }
            }
        }
    }
}
