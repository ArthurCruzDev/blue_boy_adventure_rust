#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
pub mod utils {
    pub mod collision_checker;
    pub mod game_event_handler;
    pub mod game_state_handler;
    pub mod key_handler;
    pub mod sound_handler;
    pub mod ui;
}

pub mod entities {
    pub mod entity;
    pub mod player;
    pub mod objects {
        pub mod asset_setter;
        pub mod obj_boots;
        pub mod obj_chest;
        pub mod obj_door;
        pub mod obj_heart;
        pub mod obj_key;
    }
    pub mod game_event;
    pub mod npc {
        pub mod npc_old_man;
    }
    pub mod monsters {
        pub mod mon_green_slime;
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
use utils::collision_checker;
use utils::game_event_handler::GameEventHandler;
use utils::game_state_handler::{GameState, GameStateHandler};
use utils::key_handler::KeyHandler;
use utils::sound_handler::SoundHandler;
use utils::ui::UIHandler;

const GAME_TITLE: &str = "Blue Boy Adventure Rust";

//SCREEN SETTINGS
const ORIGINAL_TILE_SIZE: u8 = 16;
const SCALE: u8 = 3;
const TILE_SIZE: u8 = ORIGINAL_TILE_SIZE * SCALE;
const MAX_SCREEN_COL: u16 = 48;
const MAX_SCREEN_ROW: u16 = 16;
const SCREEN_WIDTH: u32 = TILE_SIZE as u32 * MAX_SCREEN_COL as u32;
const SCREEN_HEIGHT: u32 = TILE_SIZE as u32 * MAX_SCREEN_ROW as u32;
const DESIRED_FPS: u32 = 75;

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
    asset_setter: AssetSetter,
    sound_handler: SoundHandler,
    ui_handler: UIHandler,
    game_state_handler: GameStateHandler,
    event_handler: GameEventHandler,
}

struct GameData {
    // Your state here...
    player: Player,
    dummy_player: Player,
    objects: Vec<Box<dyn GameEntity>>,
    npcs: Vec<Box<dyn GameEntity>>,
    monsters: Vec<Box<dyn GameEntity>>,
    game_handlers: GameHandlers,
}

impl GameData {
    pub fn new(ctx: &mut Context) -> GameData {
        // Load/create resources such as images here.

        ctx.gfx.add_font(
            "Maru Monica",
            graphics::FontData::from_path(ctx, "/fonts/x12y16pxMaruMonica.ttf").unwrap(),
        );

        ctx.gfx.add_font(
            "Purisa Bold",
            graphics::FontData::from_path(ctx, "/fonts/Purisa Bold.ttf").unwrap(),
        );

        let mut player = Player::default();
        player.get_player_images(ctx);

        let mut sound_handler = SoundHandler::default();

        let asset_setter = AssetSetter {};

        let objects = AssetSetter::initialize_objects();
        let npcs = AssetSetter::initialize_npcs(ctx);
        let monsters = AssetSetter::initialize_monsters(ctx);

        GameData {
            // ...
            // image1,
            player,
            dummy_player: Player::default(),
            objects,
            npcs,
            monsters,
            game_handlers: GameHandlers {
                key_handler: KeyHandler::default(),
                tile_manager: TileManager::new(ctx),
                asset_setter,
                sound_handler,
                ui_handler: UIHandler::new(ctx),
                game_state_handler: GameStateHandler::default(),
                event_handler: GameEventHandler::new(),
            },
        }
    }
}

impl EventHandler for GameData {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        while ctx.time.check_update_time(DESIRED_FPS) {
            // Update code here...
            match self.game_handlers.game_state_handler.game_state {
                GameState::PLAY => {
                    if self.game_handlers.ui_handler.game_finished {
                        return Ok(());
                    }

                    update_player(
                        &mut self.npcs,
                        &mut self.objects,
                        &mut self.monsters,
                        &mut self.game_handlers,
                        ctx,
                        &mut self.player,
                    );
                    update_npcs(
                        &mut self.npcs,
                        &self.objects,
                        &self.monsters,
                        &mut self.game_handlers,
                        ctx,
                        &self.player,
                    );
                    update_monsters(
                        &self.npcs,
                        &self.objects,
                        &mut self.monsters,
                        &mut self.game_handlers,
                        ctx,
                        &self.player,
                    );
                }
                GameState::PAUSED => {}
                GameState::DIALOGUE => {}
                GameState::TITLE => {}
            }
            timer::yield_now();
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::BLACK);
        canvas.set_sampler(Sampler::nearest_clamp());

        match self.game_handlers.game_state_handler.game_state {
            GameState::TITLE => {
                self.game_handlers.ui_handler.draw(
                    &mut canvas,
                    ctx,
                    &self.player,
                    &self.game_handlers.game_state_handler,
                );
            }
            _ => {
                self.game_handlers
                    .tile_manager
                    .draw(ctx, &mut canvas, &self.player);

                let mut entity_list: Vec<&dyn GameEntity> =
                    Vec::with_capacity(1 + self.objects.len() + self.npcs.len());

                entity_list.push(&self.player);
                self.objects
                    .iter()
                    .for_each(|obj| entity_list.push(obj.as_ref()));

                self.npcs
                    .iter()
                    .for_each(|npc| entity_list.push(npc.as_ref()));
                self.monsters
                    .iter()
                    .for_each(|monster| entity_list.push(monster.as_ref()));

                entity_list.sort_by_key(|game_entity| game_entity.entity_data().world_y);
                entity_list
                    .iter()
                    .for_each(|game_entity| game_entity.draw(&mut canvas, &self.player));

                self.game_handlers.ui_handler.draw(
                    &mut canvas,
                    ctx,
                    &self.player,
                    &self.game_handlers.game_state_handler,
                );
            }
        }
        // Draw code here...

        //FPS Counter
        canvas.draw(
            &graphics::Text::new(TextFragment {
                text: format!(
                    "FPS: {:.0} | {:.3}ms",
                    ctx.time.fps(),
                    ctx.time.average_delta().as_nanos() as f32 / 1_000_000_f32
                ),
                color: Some(Color::WHITE),
                font: Some("Maru Monica".into()),
                scale: Some(PxScale::from(24.0)),
            }),
            graphics::DrawParam::new().dest(Vec2 { x: 5.0, y: 5.0 }),
        );

        canvas.finish(ctx)
    }

    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        input: ggez::input::keyboard::KeyInput,
        _repeated: bool,
    ) -> Result<(), ggez::GameError> {
        self.game_handlers.key_handler.handle_key_down(
            input,
            _repeated,
            ctx,
            &mut self.game_handlers.game_state_handler,
            &mut self.game_handlers.ui_handler,
            &mut self.game_handlers.sound_handler,
        );
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

pub fn update_npcs(
    npcs: &mut [Box<dyn GameEntity>],
    objects: &Vec<Box<dyn GameEntity>>,
    monsters: &Vec<Box<dyn GameEntity>>,
    game_handlers: &mut GameHandlers,
    ctx: &mut Context,
    player: &Player,
) {
    for i in 0..npcs.len() {
        let mut has_collided = false;
        if collision_checker::check_tile(npcs[i].entity_data(), &game_handlers.tile_manager) {
            has_collided = true;
        } else if let Some(_) = collision_checker::check_entity(npcs[i].entity_data(), npcs) {
            has_collided = true;
        } else if collision_checker::check_player(npcs[i].entity_data(), player) {
            has_collided = true;
        } else if let Some(_) = collision_checker::check_entity(npcs[i].entity_data(), monsters) {
            has_collided = true;
        }

        npcs[i].update(ctx, game_handlers, has_collided);
    }
}

pub fn update_monsters(
    npcs: &[Box<dyn GameEntity>],
    _: &[Box<dyn GameEntity>],
    monsters: &mut Vec<Box<dyn GameEntity>>,
    game_handlers: &mut GameHandlers,
    ctx: &mut Context,
    player: &Player,
) {
    for i in 0..monsters.len() {
        let mut has_collided = false;
        if collision_checker::check_tile(monsters[i].entity_data(), &game_handlers.tile_manager) {
            has_collided = true;
        } else if let Some(_) = collision_checker::check_entity(monsters[i].entity_data(), npcs) {
            has_collided = true;
        } else if collision_checker::check_player(monsters[i].entity_data(), player) {
            has_collided = true;
        } else if let Some(_) = collision_checker::check_entity(monsters[i].entity_data(), monsters)
        {
            has_collided = true;
        }

        monsters[i].update(ctx, game_handlers, has_collided);
    }
}

pub fn update_player(
    npcs: &mut [Box<dyn GameEntity>],
    objects: &mut [Box<dyn GameEntity>],
    monsters: &mut [Box<dyn GameEntity>],
    game_handlers: &mut GameHandlers,
    ctx: &mut Context,
    player: &mut Player,
) {
    let mut has_collided = false;
    if collision_checker::check_tile(player.entity_data(), &game_handlers.tile_manager) {
        has_collided = true;
    }
    if let Some(npc_index) = collision_checker::check_entity(player.entity_data(), npcs) {
        player.interact_npc(npcs[npc_index as usize].as_mut(), game_handlers);
        has_collided = true;
    }
    if let Some(object_index) = collision_checker::check_object(&player.entity, objects) {
        has_collided = true;
        player.pick_up_object(
            ctx,
            object_index,
            &mut game_handlers.asset_setter,
            &mut game_handlers.sound_handler,
            &mut game_handlers.ui_handler,
        );
    }
    if let Some(_) = collision_checker::check_entity(&player.entity, monsters) {
        has_collided = true;
    }
    player.update(ctx, game_handlers, has_collided);
}
