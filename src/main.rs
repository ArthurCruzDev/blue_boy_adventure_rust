pub mod utils {
    pub mod collision_checker;
    pub mod key_handler;
    pub mod sound_handler;
    pub mod ui;
}

pub mod entities {
    pub mod entity;
    pub mod object;
    pub mod player;
    pub mod objects {
        pub mod asset_setter;
        pub mod obj_boots;
        pub mod obj_chest;
        pub mod obj_door;
        pub mod obj_key;
    }
}

pub mod tiles {
    pub mod tile;
}

use std::{env, path};

use ::fast_log::filter::ModuleFilter;
use ::fast_log::Config;
use entities::entity::GameEntity;
use entities::objects::asset_setter::AssetSetter;
use entities::player::Player;
use fast_log::fast_log;
use ggez::event::{self, EventHandler};
use ggez::glam::Vec2;
use ggez::graphics::{self, Color, PxScale, Sampler, TextFragment};
use ggez::{timer, Context, ContextBuilder, GameResult};
use tiles::tile::TileManager;
use utils::collision_checker::CollisionChecker;
use utils::key_handler::KeyHandler;
use utils::sound_handler::SoundHandler;
use utils::ui::UIHandler;

const GAME_TITLE: &str = "Blue Boy Adventure Rust";

//SCREEN SETTINGS
const ORIGINAL_TILE_SIZE: u8 = 16;
const SCALE: u8 = 3;
const TILE_SIZE: u8 = ORIGINAL_TILE_SIZE * SCALE;
const MAX_SCREEN_COL: u8 = 16;
const MAX_SCREEN_ROW: u8 = 12;
const SCREEN_WIDTH: u32 = TILE_SIZE as u32 * MAX_SCREEN_COL as u32;
const SCREEN_HEIGHT: u32 = TILE_SIZE as u32 * MAX_SCREEN_ROW as u32;

// WORLD SETTINGS
const MAX_WORLD_COL: u32 = 50;
const MAX_WORLD_ROW: u32 = 50;

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
                .vsync(false),
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
    let my_game = GameData::new(&mut ctx);

    // Run!
    event::run(ctx, event_loop, my_game);
}

pub struct GameHandlers {
    key_handler: KeyHandler,
    tile_manager: TileManager,
    collision_checker: CollisionChecker,
    asset_setter: AssetSetter,
    sound_handler: SoundHandler,
    ui_handler: UIHandler,
}

struct GameData {
    // Your state here...
    player: Player,
    game_handlers: GameHandlers,
}

impl GameData {
    pub fn new(_ctx: &mut Context) -> GameData {
        // Load/create resources such as images here.

        let mut player = Player::default();
        player.get_player_images(_ctx);

        let mut sound_handler = SoundHandler::default();
        sound_handler.play_music(_ctx, 0);

        GameData {
            // ...
            // image1,
            player,
            game_handlers: GameHandlers {
                key_handler: KeyHandler::default(),
                tile_manager: TileManager::new(_ctx),
                collision_checker: CollisionChecker {},
                asset_setter: AssetSetter::new(_ctx),
                sound_handler,
                ui_handler: UIHandler::new(_ctx),
            },
        }
    }
}

impl EventHandler for GameData {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        const DESIRED_FPS: u32 = 75;
        while ctx.time.check_update_time(DESIRED_FPS) {
            if self.game_handlers.ui_handler.game_finished {
                return Ok(());
            }
            // Update code here...
            self.player.update(&mut self.game_handlers, ctx);
            timer::yield_now();
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::BLACK);
        canvas.set_sampler(Sampler::nearest_clamp());

        // Draw code here...
        self.game_handlers
            .tile_manager
            .draw(&mut canvas, &self.player);

        self.game_handlers
            .asset_setter
            .draw(&mut canvas, &self.player);

        self.player.draw(&mut canvas);

        self.game_handlers
            .ui_handler
            .draw(&mut canvas, &self.player);

        //FPS Counter
        canvas.draw(
            &graphics::Text::new(TextFragment {
                text: format!(
                    "FPS: {:.0} | {:.5}ms",
                    ctx.time.fps(),
                    ctx.time.average_delta().as_nanos() / 1_000_000
                ),
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
        self.game_handlers
            .key_handler
            .handle_key_down(input, _repeated);
        Ok(())
    }

    fn key_up_event(
        &mut self,
        _ctx: &mut Context,
        input: ggez::input::keyboard::KeyInput,
    ) -> Result<(), ggez::GameError> {
        self.game_handlers.key_handler.handle_key_up(input);
        Ok(())
    }
}
