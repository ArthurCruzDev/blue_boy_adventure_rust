use ggez::{graphics::Canvas, Context};
use log::info;

use crate::{
    entities::{entity::GameEntity, npc::npc_old_man::NPCOldMan, player::Player},
    GameHandlers, TILE_SIZE,
};

pub struct AssetSetter {}

impl AssetSetter {
    pub fn initialize_objects() -> Vec<Box<dyn GameEntity>> {
        info!("Creating initial objects...");
        let current_objects: Vec<Box<dyn GameEntity>> = vec![];
        info!("Finished creating initial objects...");
        current_objects
    }

    pub fn initialize_npcs(ctx: &mut Context) -> Vec<Box<dyn GameEntity>> {
        info!("Creating initial NPCs...");
        let mut npc_old_man = NPCOldMan::new(ctx);
        npc_old_man.entity.world_x = TILE_SIZE as i32 * 21;
        npc_old_man.entity.world_y = TILE_SIZE as i32 * 21;
        let npcs: Vec<Box<dyn GameEntity>> = vec![Box::new(npc_old_man)];
        info!("Finished creating initial NPCs...");
        npcs
    }

    pub fn update_npcs(
        npcs: &mut [Box<dyn GameEntity>],
        objects: &mut Vec<Box<dyn GameEntity>>,
        game_handlers: &mut GameHandlers,
        ctx: &mut Context,
        player: &mut Player,
    ) {
        npcs.iter_mut()
            .for_each(|npc| npc.update(game_handlers, ctx, objects, &mut Vec::default(), player))
    }
}
