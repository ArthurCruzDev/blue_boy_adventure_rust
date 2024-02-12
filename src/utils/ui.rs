use crate::{
    entities::{entity::GameEntity, player::Player},
    GAME_TITLE, SCALE, SCREEN_HEIGHT, SCREEN_WIDTH, TILE_SIZE,
};

use super::game_state_handler::{GameState, GameStateHandler};
use chrono::{Local, NaiveDateTime};
use ggez::{
    glam::Vec2,
    graphics::{
        self, Canvas, Color, DrawParam, FillOptions, Image, Mesh, MeshBuilder, PxScale, Rect,
        StrokeOptions, Text, TextAlign, TextFragment, TextLayout,
    },
    Context,
};

#[derive(Clone)]
struct DrawStringProperties {
    text: String,
    font_size: f32,
    color: Color,
    font: String,
    x: f32,
    y: f32,
    bound_x: f32,
    bound_y: f32,
    text_align: TextAlign,
}
impl Default for DrawStringProperties {
    fn default() -> Self {
        Self {
            text: "".to_string(),
            font_size: 32.0,
            color: Color::WHITE,
            font: "Maru Monica".to_string(),
            x: 0.0,
            y: 0.0,
            bound_x: 0.0,
            bound_y: 0.0,
            text_align: TextAlign::Begin,
        }
    }
}

pub struct UIHandler {
    pub message_on: bool,
    message: String,
    message_draw_param: DrawParam,
    message_counter: NaiveDateTime,
    pub game_finished: bool,
    pub current_dialogue: String,
    pub command_num: i8,
    pub heart_full: Option<Image>,
    pub heart_half: Option<Image>,
    pub heart_blank: Option<Image>,
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
            command_num: 0,
            heart_full: Some(graphics::Image::from_path(ctx, "/objects/heart_full.png").unwrap()),
            heart_half: Some(graphics::Image::from_path(ctx, "/objects/heart_half.png").unwrap()),
            heart_blank: Some(graphics::Image::from_path(ctx, "/objects/heart_blank.png").unwrap()),
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
            GameState::Play => self.draw_play_state(canvas, player, game_state_handler),
            GameState::Paused => {
                self.draw_player_life(canvas, player);
                self.draw_paused_state(canvas, player, game_state_handler)
            }
            GameState::Dialogue => {
                self.draw_player_life(canvas, player);
                self.draw_dialogue_state(canvas, ctx, player, game_state_handler)
            }
            GameState::Title => self.draw_title_state(canvas, ctx, player, game_state_handler),
            GameState::Character => self.draw_character_screen(canvas, ctx, player),
        }
    }

    fn draw_play_state(
        &mut self,
        canvas: &mut Canvas,
        player: &Player,
        game_state_handler: &GameStateHandler,
    ) {
        self.draw_player_life(canvas, player);
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

    fn draw_character_screen(&mut self, canvas: &mut Canvas, ctx: &mut Context, player: &Player) {
        let mut x = TILE_SIZE as f32;
        let mut y = TILE_SIZE as f32;
        let width = TILE_SIZE as f32 * 5.0;
        let height = TILE_SIZE as f32 * 10.0;

        self.draw_sub_window(x, y, width, height, canvas, ctx);

        x += 20.0;
        y += TILE_SIZE as f32 / 4.0;
        let line_height = 35.0;

        let mut text_properties = DrawStringProperties {
            text: "Level".to_string(),
            x,
            y,
            bound_x: width / 2.0,
            bound_y: 32.0,
            ..Default::default()
        };

        Self::draw_string(canvas, &text_properties);

        text_properties.y += line_height;
        text_properties.text = "Life".to_string();
        Self::draw_string(canvas, &text_properties);

        text_properties.y += line_height;
        text_properties.text = "Strenght".to_string();
        Self::draw_string(canvas, &text_properties);

        text_properties.y += line_height;
        text_properties.text = "Dexterity".to_string();
        Self::draw_string(canvas, &text_properties);

        text_properties.y += line_height;
        text_properties.text = "Attack".to_string();
        Self::draw_string(canvas, &text_properties);

        text_properties.y += line_height;
        text_properties.text = "Defense".to_string();
        Self::draw_string(canvas, &text_properties);

        text_properties.y += line_height;
        text_properties.text = "Exp".to_string();
        Self::draw_string(canvas, &text_properties);

        text_properties.y += line_height;
        text_properties.text = "Next Level".to_string();
        Self::draw_string(canvas, &text_properties);

        text_properties.y += line_height;
        text_properties.text = "Coin".to_string();
        Self::draw_string(canvas, &text_properties);

        text_properties.y += line_height + 20.0;
        text_properties.text = "Weapon".to_string();
        Self::draw_string(canvas, &text_properties);

        text_properties.y += TILE_SIZE as f32;
        text_properties.text = "Shield".to_string();
        Self::draw_string(canvas, &text_properties);

        text_properties.text_align = TextAlign::End;
        text_properties.x = x + (TILE_SIZE as f32 * 5.0) - 40.0;
        text_properties.y = TILE_SIZE as f32 + (TILE_SIZE as f32 / 4.0);
        text_properties.bound_x = width / 2.0;
        text_properties.text = format!("{}", player.entity_data().level);
        Self::draw_string(canvas, &text_properties);

        text_properties.y += line_height;
        text_properties.text = format!(
            "{}/{}",
            player.entity_data().life,
            player.entity_data().max_life
        );
        Self::draw_string(canvas, &text_properties);

        text_properties.y += line_height;
        text_properties.text = format!("{}", player.entity_data().strength);
        Self::draw_string(canvas, &text_properties);

        text_properties.y += line_height;
        text_properties.text = format!("{}", player.entity_data().dexterity);
        Self::draw_string(canvas, &text_properties);

        text_properties.y += line_height;
        text_properties.text = format!("{}", player.entity_data().attack);
        Self::draw_string(canvas, &text_properties);

        text_properties.y += line_height;
        text_properties.text = format!("{}", player.entity_data().defense);
        Self::draw_string(canvas, &text_properties);

        text_properties.y += line_height;
        text_properties.text = format!("{}", player.entity_data().exp);
        Self::draw_string(canvas, &text_properties);

        text_properties.y += line_height;
        text_properties.text = format!("{}", player.entity_data().next_level_exp);
        Self::draw_string(canvas, &text_properties);

        text_properties.y += line_height;
        text_properties.text = format!("{}", player.entity_data().coin);
        Self::draw_string(canvas, &text_properties);

        text_properties.y += line_height + 20.0;
        if let Some(weapon) = &player.entity_data().current_weapon {
            if let Some(image) = &weapon.entity_data().down_1 {
                canvas.draw(
                    image,
                    DrawParam::new()
                        .dest(Vec2::new(
                            text_properties.x - TILE_SIZE as f32,
                            text_properties.y,
                        ))
                        .scale(Vec2::new(SCALE as f32, SCALE as f32)),
                );
            }
        }

        text_properties.y += TILE_SIZE as f32;
        if let Some(weapon) = &player.entity_data().current_shield {
            if let Some(image) = &weapon.entity_data().down_1 {
                canvas.draw(
                    image,
                    DrawParam::new()
                        .dest(Vec2::new(
                            text_properties.x - TILE_SIZE as f32,
                            text_properties.y,
                        ))
                        .scale(Vec2::new(SCALE as f32, SCALE as f32)),
                );
            }
        }
    }

    fn draw_string(canvas: &mut Canvas, text_properties: &DrawStringProperties) {
        canvas.draw(
            Text::new(TextFragment {
                text: text_properties.text.to_string(),
                scale: Some(PxScale::from(text_properties.font_size)),
                color: Some(text_properties.color),
                font: Some(text_properties.font.to_string()),
            })
            .set_layout(TextLayout {
                h_align: text_properties.text_align,
                v_align: graphics::TextAlign::Begin,
            })
            .set_bounds(Vec2 {
                x: text_properties.bound_x,
                y: text_properties.bound_y,
            }),
            DrawParam::default().dest(Vec2::new(text_properties.x, text_properties.y)),
        );
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
        let background_color = Color::new(0.0, 0.0, 0.0, 0.95);
        let stroke_color = Color::new(1.0, 1.0, 1.0, 1.0);
        let background = Rect::new(x, y, width, height);
        let stroke_bounds = Rect::new(x + 4.0, y + 4.0, width - 8.0, height - 8.0);

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
                    stroke_bounds,
                    14.0,
                    stroke_color,
                )
                .unwrap()
                .build(),
        );

        canvas.draw(&mesh_data, DrawParam::default());
    }

    fn draw_title_state(
        &mut self,
        canvas: &mut Canvas,
        ctx: &mut Context,
        player: &Player,
        game_state_handler: &GameStateHandler,
    ) {
        match game_state_handler.title_screen_state {
            super::game_state_handler::TitleScreenSubState::MainMenu => {
                self.draw_title_state_main_menu(canvas, ctx, player, game_state_handler);
            }
            super::game_state_handler::TitleScreenSubState::ClassMenu => {
                self.draw_title_state_class_menu(canvas, ctx, player, game_state_handler);
            }
        }
    }

    fn draw_title_state_main_menu(
        &mut self,
        canvas: &mut Canvas,
        ctx: &mut Context,
        player: &Player,
        game_state_handler: &GameStateHandler,
    ) {
        canvas.draw(
            Text::new(TextFragment {
                text: GAME_TITLE.to_string(),
                scale: Some(PxScale::from(96.0)),
                color: Some(Color::from_rgba(100, 100, 100, 255)),
                font: Some("Maru Monica".to_string()),
            })
            .set_layout(TextLayout {
                h_align: graphics::TextAlign::Middle,
                v_align: graphics::TextAlign::Middle,
            })
            .set_bounds(Vec2 {
                x: SCREEN_WIDTH as f32,
                y: SCREEN_HEIGHT as f32,
            }),
            DrawParam::default().dest(Vec2::new(
                SCREEN_WIDTH as f32 / 2.0 + 5.0,
                TILE_SIZE as f32 * 3.0 + 5.0,
            )),
        );
        canvas.draw(
            Text::new(TextFragment {
                text: GAME_TITLE.to_string(),
                scale: Some(PxScale::from(96.0)),
                color: Some(Color::WHITE),
                font: Some("Maru Monica".to_string()),
            })
            .set_layout(TextLayout {
                h_align: graphics::TextAlign::Middle,
                v_align: graphics::TextAlign::Middle,
            })
            .set_bounds(Vec2 {
                x: SCREEN_WIDTH as f32,
                y: SCREEN_HEIGHT as f32,
            }),
            DrawParam::default().dest(Vec2::new(SCREEN_WIDTH as f32 / 2.0, TILE_SIZE as f32 * 3.0)),
        );
        canvas.draw(
            &player.entity_data().down_1.clone().unwrap(),
            DrawParam::new()
                .dest(Vec2::new(
                    SCREEN_WIDTH as f32 / 2.0 - TILE_SIZE as f32,
                    TILE_SIZE as f32 * 5.0,
                ))
                .scale(Vec2::new(SCALE as f32 * 2.0, SCALE as f32 * 2.0)),
        );

        canvas.draw(
            Text::new(TextFragment {
                text: "NEW GAME".to_string(),
                scale: Some(PxScale::from(40.0)),
                color: Some(Color::WHITE),
                font: Some("Maru Monica".to_string()),
            })
            .set_layout(TextLayout {
                h_align: graphics::TextAlign::Middle,
                v_align: graphics::TextAlign::Middle,
            })
            .set_bounds(Vec2 {
                x: SCREEN_WIDTH as f32,
                y: SCREEN_HEIGHT as f32,
            }),
            DrawParam::default().dest(Vec2::new(SCREEN_WIDTH as f32 / 2.0, TILE_SIZE as f32 * 8.5)),
        );

        if self.command_num == 0 {
            canvas.draw(
                Text::new(TextFragment {
                    text: ">".to_string(),
                    scale: Some(PxScale::from(40.0)),
                    color: Some(Color::WHITE),
                    font: Some("Maru Monica".to_string()),
                })
                .set_layout(TextLayout {
                    h_align: graphics::TextAlign::Middle,
                    v_align: graphics::TextAlign::Middle,
                })
                .set_bounds(Vec2 {
                    x: SCREEN_WIDTH as f32,
                    y: SCREEN_HEIGHT as f32,
                }),
                DrawParam::default().dest(Vec2::new(
                    (SCREEN_WIDTH as f32 / 2.0) - (TILE_SIZE as f32 * 2.0),
                    TILE_SIZE as f32 * 8.5,
                )),
            );
        }

        canvas.draw(
            Text::new(TextFragment {
                text: "LOAD GAME".to_string(),
                scale: Some(PxScale::from(40.0)),
                color: Some(Color::WHITE),
                font: Some("Maru Monica".to_string()),
            })
            .set_layout(TextLayout {
                h_align: graphics::TextAlign::Middle,
                v_align: graphics::TextAlign::Middle,
            })
            .set_bounds(Vec2 {
                x: SCREEN_WIDTH as f32,
                y: SCREEN_HEIGHT as f32,
            }),
            DrawParam::default().dest(Vec2::new(SCREEN_WIDTH as f32 / 2.0, TILE_SIZE as f32 * 9.5)),
        );

        if self.command_num == 1 {
            canvas.draw(
                Text::new(TextFragment {
                    text: ">".to_string(),
                    scale: Some(PxScale::from(40.0)),
                    color: Some(Color::WHITE),
                    font: Some("Maru Monica".to_string()),
                })
                .set_layout(TextLayout {
                    h_align: graphics::TextAlign::Middle,
                    v_align: graphics::TextAlign::Middle,
                })
                .set_bounds(Vec2 {
                    x: SCREEN_WIDTH as f32,
                    y: SCREEN_HEIGHT as f32,
                }),
                DrawParam::default().dest(Vec2::new(
                    (SCREEN_WIDTH as f32 / 2.0) - (TILE_SIZE as f32 * 2.0),
                    TILE_SIZE as f32 * 9.5,
                )),
            );
        }

        canvas.draw(
            Text::new(TextFragment {
                text: "QUIT".to_string(),
                scale: Some(PxScale::from(40.0)),
                color: Some(Color::WHITE),
                font: Some("Maru Monica".to_string()),
            })
            .set_layout(TextLayout {
                h_align: graphics::TextAlign::Middle,
                v_align: graphics::TextAlign::Middle,
            })
            .set_bounds(Vec2 {
                x: SCREEN_WIDTH as f32,
                y: SCREEN_HEIGHT as f32,
            }),
            DrawParam::default().dest(Vec2::new(
                SCREEN_WIDTH as f32 / 2.0,
                TILE_SIZE as f32 * 10.5,
            )),
        );

        if self.command_num == 2 {
            canvas.draw(
                Text::new(TextFragment {
                    text: ">".to_string(),
                    scale: Some(PxScale::from(40.0)),
                    color: Some(Color::WHITE),
                    font: Some("Maru Monica".to_string()),
                })
                .set_layout(TextLayout {
                    h_align: graphics::TextAlign::Middle,
                    v_align: graphics::TextAlign::Middle,
                })
                .set_bounds(Vec2 {
                    x: SCREEN_WIDTH as f32,
                    y: SCREEN_HEIGHT as f32,
                }),
                DrawParam::default().dest(Vec2::new(
                    (SCREEN_WIDTH as f32 / 2.0) - (TILE_SIZE as f32 * 2.0),
                    TILE_SIZE as f32 * 10.5,
                )),
            );
        }
    }

    fn draw_title_state_class_menu(
        &mut self,
        canvas: &mut Canvas,
        ctx: &mut Context,
        player: &Player,
        game_state_handler: &GameStateHandler,
    ) {
        let mut x: f32 = SCREEN_WIDTH as f32 / 2.0;
        let mut y: f32 = TILE_SIZE as f32 * 3.0;

        canvas.draw(
            Text::new(TextFragment {
                text: "Select Your Class".to_string(),
                scale: Some(PxScale::from(42.0)),
                color: Some(Color::WHITE),
                font: Some("Maru Monica".to_string()),
            })
            .set_layout(TextLayout {
                h_align: graphics::TextAlign::Middle,
                v_align: graphics::TextAlign::Middle,
            })
            .set_bounds(Vec2 {
                x: SCREEN_WIDTH as f32,
                y: SCREEN_HEIGHT as f32,
            }),
            DrawParam::default().dest(Vec2::new(x, y)),
        );

        y += TILE_SIZE as f32 * 3f32;

        canvas.draw(
            Text::new(TextFragment {
                text: "Fighter".to_string(),
                scale: Some(PxScale::from(42.0)),
                color: Some(Color::WHITE),
                font: Some("Maru Monica".to_string()),
            })
            .set_layout(TextLayout {
                h_align: graphics::TextAlign::Middle,
                v_align: graphics::TextAlign::Middle,
            })
            .set_bounds(Vec2 {
                x: SCREEN_WIDTH as f32,
                y: SCREEN_HEIGHT as f32,
            }),
            DrawParam::default().dest(Vec2::new(x, y)),
        );
        if self.command_num == 0 {
            canvas.draw(
                Text::new(TextFragment {
                    text: ">".to_string(),
                    scale: Some(PxScale::from(42.0)),
                    color: Some(Color::WHITE),
                    font: Some("Maru Monica".to_string()),
                })
                .set_layout(TextLayout {
                    h_align: graphics::TextAlign::Middle,
                    v_align: graphics::TextAlign::Middle,
                })
                .set_bounds(Vec2 {
                    x: SCREEN_WIDTH as f32,
                    y: SCREEN_HEIGHT as f32,
                }),
                DrawParam::default().dest(Vec2::new(x - TILE_SIZE as f32 * 2f32, y)),
            );
        }

        y += TILE_SIZE as f32 * 1f32;
        canvas.draw(
            Text::new(TextFragment {
                text: "Thief".to_string(),
                scale: Some(PxScale::from(42.0)),
                color: Some(Color::WHITE),
                font: Some("Maru Monica".to_string()),
            })
            .set_layout(TextLayout {
                h_align: graphics::TextAlign::Middle,
                v_align: graphics::TextAlign::Middle,
            })
            .set_bounds(Vec2 {
                x: SCREEN_WIDTH as f32,
                y: SCREEN_HEIGHT as f32,
            }),
            DrawParam::default().dest(Vec2::new(x, y)),
        );
        if self.command_num == 1 {
            canvas.draw(
                Text::new(TextFragment {
                    text: ">".to_string(),
                    scale: Some(PxScale::from(42.0)),
                    color: Some(Color::WHITE),
                    font: Some("Maru Monica".to_string()),
                })
                .set_layout(TextLayout {
                    h_align: graphics::TextAlign::Middle,
                    v_align: graphics::TextAlign::Middle,
                })
                .set_bounds(Vec2 {
                    x: SCREEN_WIDTH as f32,
                    y: SCREEN_HEIGHT as f32,
                }),
                DrawParam::default().dest(Vec2::new(x - TILE_SIZE as f32 * 2f32, y)),
            );
        }

        y += TILE_SIZE as f32 * 1f32;
        canvas.draw(
            Text::new(TextFragment {
                text: "Sorcerer".to_string(),
                scale: Some(PxScale::from(42.0)),
                color: Some(Color::WHITE),
                font: Some("Maru Monica".to_string()),
            })
            .set_layout(TextLayout {
                h_align: graphics::TextAlign::Middle,
                v_align: graphics::TextAlign::Middle,
            })
            .set_bounds(Vec2 {
                x: SCREEN_WIDTH as f32,
                y: SCREEN_HEIGHT as f32,
            }),
            DrawParam::default().dest(Vec2::new(x, y)),
        );
        if self.command_num == 2 {
            canvas.draw(
                Text::new(TextFragment {
                    text: ">".to_string(),
                    scale: Some(PxScale::from(42.0)),
                    color: Some(Color::WHITE),
                    font: Some("Maru Monica".to_string()),
                })
                .set_layout(TextLayout {
                    h_align: graphics::TextAlign::Middle,
                    v_align: graphics::TextAlign::Middle,
                })
                .set_bounds(Vec2 {
                    x: SCREEN_WIDTH as f32,
                    y: SCREEN_HEIGHT as f32,
                }),
                DrawParam::default().dest(Vec2::new(x - TILE_SIZE as f32 * 2f32, y)),
            );
        }

        y += TILE_SIZE as f32 * 2f32;
        canvas.draw(
            Text::new(TextFragment {
                text: "Back".to_string(),
                scale: Some(PxScale::from(42.0)),
                color: Some(Color::WHITE),
                font: Some("Maru Monica".to_string()),
            })
            .set_layout(TextLayout {
                h_align: graphics::TextAlign::Middle,
                v_align: graphics::TextAlign::Middle,
            })
            .set_bounds(Vec2 {
                x: SCREEN_WIDTH as f32,
                y: SCREEN_HEIGHT as f32,
            }),
            DrawParam::default().dest(Vec2::new(x, y)),
        );
        if self.command_num == 3 {
            canvas.draw(
                Text::new(TextFragment {
                    text: ">".to_string(),
                    scale: Some(PxScale::from(42.0)),
                    color: Some(Color::WHITE),
                    font: Some("Maru Monica".to_string()),
                })
                .set_layout(TextLayout {
                    h_align: graphics::TextAlign::Middle,
                    v_align: graphics::TextAlign::Middle,
                })
                .set_bounds(Vec2 {
                    x: SCREEN_WIDTH as f32,
                    y: SCREEN_HEIGHT as f32,
                }),
                DrawParam::default().dest(Vec2::new(x - TILE_SIZE as f32 * 2f32, y)),
            );
        }
    }

    fn draw_player_life(&mut self, canvas: &mut Canvas, player: &Player) {
        let mut x = TILE_SIZE as f32 / 2.0;
        let mut y = TILE_SIZE as f32 / 2.0;
        let mut i = 0;

        while i < player.entity.max_life / 2 {
            if let Some(image) = &self.heart_blank {
                canvas.draw(
                    image,
                    DrawParam::new()
                        .dest(Vec2::new(x, y))
                        .scale(Vec2::new(SCALE as f32, SCALE as f32)),
                );
            }
            i += 1;
            x += TILE_SIZE as f32;
        }

        x = TILE_SIZE as f32 / 2.0;
        y = TILE_SIZE as f32 / 2.0;
        i = 0;

        while i < player.entity.life {
            if let Some(image) = &self.heart_half {
                canvas.draw(
                    image,
                    DrawParam::new()
                        .dest(Vec2::new(x, y))
                        .scale(Vec2::new(SCALE as f32, SCALE as f32)),
                );
            }
            i += 1;
            if i < player.entity.life {
                if let Some(image) = &self.heart_full {
                    canvas.draw(
                        image,
                        DrawParam::new()
                            .dest(Vec2::new(x, y))
                            .scale(Vec2::new(SCALE as f32, SCALE as f32)),
                    );
                }
            }
            i += 1;
            x += TILE_SIZE as f32;
        }
    }
}
