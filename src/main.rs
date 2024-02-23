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
        pub mod obj_axe;
        pub mod obj_boots;
        pub mod obj_chest;
        pub mod obj_door;
        pub mod obj_fireball;
        pub mod obj_heart;
        pub mod obj_key;
        pub mod obj_potion_red;
        pub mod obj_shield_blue;
        pub mod obj_shield_wood;
        pub mod obj_sword_normal;
    }
    pub mod game_event;
    pub mod npc {
        pub mod npc_old_man;
    }
    pub mod monsters {
        pub mod mon_green_slime;
    }
    pub mod projectiles {
        pub mod projectile;
    }
}

pub mod tiles {
    pub mod tile;
}

use std::cell::RefCell;
use std::rc::Rc;
use std::{env, path};

use ::fast_log::filter::ModuleFilter;
use ::fast_log::Config;
use entities::entity::GameEntity;
use entities::objects::asset_setter::AssetSetter;
use entities::objects::obj_fireball::ObjFireball;
use entities::objects::obj_shield_wood::ObjShieldWood;
use entities::objects::obj_sword_normal::ObjSwordNormal;
use entities::player::Player;
use entities::projectiles::projectile::Projectile;
use fast_log::fast_log;
use ggez::event::{self, EventHandler};
use ggez::glam::Vec2;
use ggez::graphics::{self, Color, PxScale, Sampler, TextFragment};
use ggez::{timer, Context, ContextBuilder, GameResult};
use log::info;
use tiles::tile::TileManager;
use utils::collision_checker;
use utils::game_event_handler::GameEventHandler;
use utils::game_state_handler::{GameState, GameStateHandler};
use utils::key_handler::KeyHandler;
use utils::sound_handler::SoundHandler;
use utils::ui::UIHandler;

use crate::entities::entity::Direction;

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
    objects: Vec<Box<dyn GameEntity>>,
    npcs: Vec<Box<dyn GameEntity>>,
    monsters: Vec<Box<dyn GameEntity>>,
    projectiles: Vec<Box<dyn GameEntity>>,
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
        player.entity.current_weapon =
            Some(Rc::new(RefCell::new(Box::new(ObjSwordNormal::new(ctx)))));
        player.entity.current_shield =
            Some(Rc::new(RefCell::new(Box::new(ObjShieldWood::new(ctx)))));
        player.entity.projectile = Some(Rc::new(RefCell::new(Box::new(ObjFireball::new(ctx)))));
        player.get_player_images(ctx);
        player.entity.attack = player.get_attack();
        player.entity.defense = player.get_defense();
        player.set_items(ctx);

        let sound_handler = SoundHandler::default();

        let objects = AssetSetter::initialize_objects(ctx);
        let npcs = AssetSetter::initialize_npcs(ctx);
        let monsters = AssetSetter::initialize_monsters(ctx);
        let mut projectiles = Vec::new();

        let fireball: Box<dyn GameEntity> = Box::new(ObjFireball::new(ctx));
        projectiles.push(fireball);

        GameData {
            // ...
            // image1,
            player,
            objects,
            npcs,
            monsters,
            projectiles,
            game_handlers: GameHandlers {
                key_handler: KeyHandler::default(),
                tile_manager: TileManager::new(ctx),
                asset_setter: AssetSetter::default(),
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
                GameState::Play => {
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
                        &mut self.player,
                    );

                    self.game_handlers
                        .asset_setter
                        .get_new_projectiles(&mut self.projectiles);
                    update_projectiles(
                        &mut self.monsters,
                        &mut self.projectiles,
                        &mut self.game_handlers,
                        ctx,
                        &mut self.player,
                    );
                    update_ui(&mut self.game_handlers);
                    if self.game_handlers.event_handler.respawn_monsters {
                        self.monsters = AssetSetter::initialize_monsters(ctx);
                        self.game_handlers.event_handler.respawn_monsters = false;
                    }
                }
                GameState::Paused => {}
                GameState::Dialogue => {}
                GameState::Title => {}
                GameState::Character => {}
            }
            timer::yield_now();
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::BLACK);
        canvas.set_sampler(Sampler::nearest_clamp());

        match self.game_handlers.game_state_handler.game_state {
            GameState::Title => {
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

                let mut entity_list: Vec<&dyn GameEntity> = Vec::with_capacity(
                    1 + self.objects.len()
                        + self.npcs.len()
                        + self.monsters.len()
                        + self.projectiles.len(),
                );

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
                self.projectiles
                    .iter()
                    .for_each(|projectile| entity_list.push(projectile.as_ref()));

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
            &mut self.player,
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
    player: &mut Player,
) {
    let mut to_be_removed: Vec<usize> = Vec::new();

    for i in 0..monsters.len() {
        if !monsters[i].entity_data().alive {
            to_be_removed.push(i);
            continue;
        }
        let mut has_collided = false;
        if collision_checker::check_tile(monsters[i].entity_data(), &game_handlers.tile_manager) {
            has_collided = true;
        } else if let Some(_) = collision_checker::check_entity(monsters[i].entity_data(), npcs) {
            has_collided = true;
        } else if collision_checker::check_player(monsters[i].entity_data(), player) {
            has_collided = true;
            player.interact_monster(ctx, monsters[i].as_mut(), game_handlers);
        } else if let Some(_) = collision_checker::check_entity(monsters[i].entity_data(), monsters)
        {
            has_collided = true;
        }

        monsters[i].update(ctx, game_handlers, has_collided);
    }
    to_be_removed.iter().for_each(|index| {
        monsters.swap_remove(*index);
    });
}

pub fn update_player(
    npcs: &mut [Box<dyn GameEntity>],
    objects: &mut Vec<Box<dyn GameEntity>>,
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
    } else if game_handlers.key_handler.enter_pressed {
        player.entity_data_mut().attacking = true;
    }
    if let Some(object_index) = collision_checker::check_object(&player.entity, objects) {
        has_collided = true;
        player.pick_up_object(
            ctx,
            object_index,
            &mut game_handlers.asset_setter,
            &mut game_handlers.sound_handler,
            &mut game_handlers.ui_handler,
            objects,
        );
    } else {
        player.last_collided_object_index = usize::MAX;
    }
    if let Some(monster_index) = collision_checker::check_entity(&player.entity, monsters) {
        has_collided = true;
        player.interact_monster(
            ctx,
            monsters[monster_index as usize].as_mut(),
            game_handlers,
        );
    }
    if player.entity.attacking {
        if let Some(monster_index) = collision_checker::check_entity_hit(&player.entity, monsters) {
            player.damage_monster(
                ctx,
                game_handlers,
                monsters[monster_index as usize].as_mut(),
                player.entity.attack,
            )
        }
    }
    player.update(ctx, game_handlers, has_collided);
}

fn update_ui(game_handlers: &mut GameHandlers) {
    game_handlers.ui_handler.upadte();
}

pub fn update_projectiles(
    monsters: &mut [Box<dyn GameEntity>],
    projectiles: &mut Vec<Box<dyn GameEntity>>,
    game_handlers: &mut GameHandlers,
    ctx: &mut Context,
    player: &mut Player,
) {
    let mut to_be_removed: Vec<usize> = Vec::new();

    for i in 0..projectiles.len() {
        if !projectiles[i].entity_data().alive {
            to_be_removed.push(i);
            continue;
        }

        if let Some(entity_type) = &projectiles[i].entity_data().projectile_thrown_by {
            match entity_type {
                entities::entity::EntityType::PLAYER => {
                    if let Some(monster_index) =
                        collision_checker::check_entity(projectiles[i].entity_data(), monsters)
                    {
                        player.damage_monster(
                            ctx,
                            game_handlers,
                            monsters[monster_index as usize].as_mut(),
                            projectiles[i].entity_data().attack,
                        );
                        projectiles[i].entity_data_mut().alive = false;
                        to_be_removed.push(i);
                    }
                }
                entities::entity::EntityType::MONSTER => {
                    if collision_checker::check_player(projectiles[i].entity_data(), player) {
                        // has_collided = true;
                        // player.interact_monster(ctx, monsters[i].as_mut(), game_handlers);
                    }
                }
                _ => {}
            }
        }
        let entity_data = projectiles[i].entity_data_mut();
        match entity_data.direction {
            Direction::UP => entity_data.world_y -= entity_data.speed,
            Direction::DOWN => entity_data.world_y += entity_data.speed,
            Direction::LEFT => entity_data.world_x -= entity_data.speed,
            Direction::RIGHT => entity_data.world_x += entity_data.speed,
        }

        entity_data.life -= 1;
        if entity_data.life <= 0 {
            entity_data.alive = false;
        }

        entity_data.sprite_counter += 1;
        if entity_data.sprite_counter > 12 {
            if entity_data.sprite_num == 1 {
                entity_data.sprite_num = 2;
            } else if entity_data.sprite_num == 2 {
                entity_data.sprite_num = 1;
            }
            entity_data.sprite_counter = 0;
        }
    }
    to_be_removed.iter().for_each(|index| {
        projectiles.swap_remove(*index);
    });
}
