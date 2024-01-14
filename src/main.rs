pub mod key_handler {
    pub mod key_handler;
}

use std::{env, path};

use ::fast_log::filter::ModuleFilter;
use ::fast_log::Config;
use fast_log::fast_log;
use ggez::event::{self, EventHandler};
use ggez::glam::Vec2;
use ggez::graphics::{self, Color, FontData, PxScale, TextFragment};
use ggez::{timer, Context, ContextBuilder, GameResult};
use key_handler::key_handler::KeyHandler;
use mint::Point2;

const GAME_TITLE: &str = "Blue Boy Adventure Rust";

const ORIGINAL_TILE_SIZE: u8 = 16;
const SCALE: u8 = 3;

const TILE_SIZE: u8 = ORIGINAL_TILE_SIZE * SCALE;

fn main() {
    fast_log::init(
        Config::new()
            .console()
            .level(log::LevelFilter::Debug)
            .filter(ModuleFilter {
                exclude: None,
                include: Some(vec!["blue_boy_adventure_rust".to_string()]),
            })
            .chan_len(Some(100000)),
    )
    .unwrap();

    let mut cb = ContextBuilder::new(GAME_TITLE, "Arthur Cruz");

    if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        println!("Adding path {path:?}");
        cb = cb.add_resource_path(path);
    }

    cb = cb
        .window_setup(
            ggez::conf::WindowSetup::default()
                .title(GAME_TITLE)
                .vsync(true),
        )
        .window_mode(
            ggez::conf::WindowMode::default()
                .dimensions(600.0, 400.0)
                .resizable(false)
                .maximized(false)
                .borderless(false),
        );

    // Make a Context.
    let (mut ctx, event_loop) = cb.build().expect("aieee, could not create ggez context!");

    // Create an instance of your event handler.
    // Usually, you should provide it with the Context object to
    // use when setting your game up.
    let my_game = MyGame::new(&mut ctx);

    // Run!
    event::run(ctx, event_loop, my_game);
}

struct MyGame {
    // Your state here...
    // image1: graphics::Image,
    player_x: f32,
    player_y: f32,
    player_speed: u8,
    key_handler: KeyHandler,
}

impl MyGame {
    pub fn new(_ctx: &mut Context) -> MyGame {
        // Load/create resources such as images here.
        // let image1 = graphics::Image::from_path(_ctx, "/skull.png").unwrap();

        MyGame {
            // ...
            // image1,
            player_x: 0.0,
            player_y: 0.0,
            player_speed: 4,
            key_handler: KeyHandler::default(),
        }
    }
}

impl EventHandler for MyGame {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        // Update code here...
        if self.key_handler.left_pressed {
            self.player_x -= self.player_speed as f32;
        } else if self.key_handler.right_pressed {
            self.player_x += self.player_speed as f32;
        } else if self.key_handler.up_pressed {
            self.player_y -= self.player_speed as f32;
        } else if self.key_handler.down_pressed {
            self.player_y += self.player_speed as f32;
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::BLACK);
        // Draw code here...

        // canvas.draw(&self.image1, graphics::DrawParam::new());

        //FPS Counter
        canvas.draw(
            &graphics::Text::new(TextFragment {
                text: format!("FPS: {:.0}", ctx.time.fps()),
                color: Some(Color::WHITE),
                font: Some("LiberationMono-Regular".into()),
                scale: Some(PxScale::from(16.0)),
            }),
            graphics::DrawParam::new().dest(Vec2 { x: 5.0, y: 5.0 }),
        );

        canvas.draw(
            &graphics::Mesh::new_rectangle(
                ctx,
                graphics::DrawMode::fill(),
                graphics::Rect {
                    x: self.player_x,
                    y: self.player_y,
                    w: TILE_SIZE as f32,
                    h: TILE_SIZE as f32,
                },
                graphics::Color::WHITE,
            )?,
            graphics::DrawParam::new(),
        );

        canvas.finish(ctx)
    }

    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        input: ggez::input::keyboard::KeyInput,
        _repeated: bool,
    ) -> Result<(), ggez::GameError> {
        self.key_handler.handle_key_down(input, _repeated);
        Ok(())
    }

    fn key_up_event(
        &mut self,
        ctx: &mut Context,
        input: ggez::input::keyboard::KeyInput,
    ) -> Result<(), ggez::GameError> {
        self.key_handler.handle_key_up(input);
        Ok(())
    }
}
