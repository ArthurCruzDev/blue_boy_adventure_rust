pub mod key_handler {
    pub mod key_handler;
}

pub mod entities {
    pub mod entity;
    pub mod player;
}

pub mod tiles {
    pub mod tile;
}

use std::{env, path};

use ::fast_log::filter::ModuleFilter;
use ::fast_log::Config;
use entities::entity::GameEntity;
use entities::player::Player;
use fast_log::fast_log;
use ggez::event::{self, EventHandler};
use ggez::glam::Vec2;
use ggez::graphics::{self, Color, PxScale, Sampler, TextFragment};
use ggez::{Context, ContextBuilder, GameResult};
use key_handler::key_handler::KeyHandler;
use tiles::tile::TileManager;

const GAME_TITLE: &str = "Blue Boy Adventure Rust";

const ORIGINAL_TILE_SIZE: u8 = 16;
const SCALE: u8 = 3;

const TILE_SIZE: u8 = ORIGINAL_TILE_SIZE * SCALE;

const MAX_SCREEN_COL: u8 = 16;
const MAX_SCREEN_ROW: u8 = 12;
const SCREEN_WIDTH: u32 = TILE_SIZE as u32 * MAX_SCREEN_COL as u32;
const SCREEN_HEIGHT: u32 = TILE_SIZE as u32 * MAX_SCREEN_ROW as u32;

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
                .dimensions(SCREEN_WIDTH as f32, SCREEN_HEIGHT as f32)
                .resizable(false)
                .maximized(false)
                .borderless(false),
        );

    // Make a Context.
    let (mut ctx, event_loop) = cb.build().expect("aieee, could not create ggez context!");

    // Create an instance of your event handler.
    // Usually, you should provide it with the Context object to
    // use when setting your game up.
    let my_game = GameState::new(&mut ctx);

    // Run!
    event::run(ctx, event_loop, my_game);
}

struct GameState {
    // Your state here...
    // image1: graphics::Image,
    player: Player,
    key_handler: KeyHandler,
    tile_manager: TileManager,
}

impl GameState {
    pub fn new(_ctx: &mut Context) -> GameState {
        // Load/create resources such as images here.
        // let image1 = graphics::Image::from_path(_ctx, "/skull.png").unwrap();

        let mut player = Player::default();
        player.get_player_images(_ctx);

        GameState {
            // ...
            // image1,
            player,
            key_handler: KeyHandler::default(),
            tile_manager: TileManager::new(_ctx),
        }
    }
}

impl EventHandler for GameState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        // Update code here...
        self.player.update(&self.key_handler);
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::BLACK);
        canvas.set_sampler(Sampler::nearest_clamp());
        // Draw code here...

        // canvas.draw(&self.image1, graphics::DrawParam::new());

        self.tile_manager.draw(ctx, &mut canvas);

        self.player.draw(ctx, &mut canvas);

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

        canvas.finish(ctx)
    }

    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        input: ggez::input::keyboard::KeyInput,
        _repeated: bool,
    ) -> Result<(), ggez::GameError> {
        self.key_handler.handle_key_down(input, _repeated);
        Ok(())
    }

    fn key_up_event(
        &mut self,
        _ctx: &mut Context,
        input: ggez::input::keyboard::KeyInput,
    ) -> Result<(), ggez::GameError> {
        self.key_handler.handle_key_up(input);
        Ok(())
    }
}
