use ggez::Context;
use log::info;

use crate::{
    entities::{
        entity::GameEntity, monsters::mon_green_slime::MonGreenSlime, npc::npc_old_man::NPCOldMan,
    },
    TILE_SIZE,
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
        npc_old_man.entity_data.world_x = TILE_SIZE as i32 * 21;
        npc_old_man.entity_data.world_y = TILE_SIZE as i32 * 21;
        let npcs: Vec<Box<dyn GameEntity>> = vec![Box::new(npc_old_man)];
        info!("Finished creating initial NPCs...");
        npcs
    }

    pub fn initialize_monsters(ctx: &mut Context) -> Vec<Box<dyn GameEntity>> {
        info!("Creating initial Monsters...");
        let mut slime_1 = MonGreenSlime::new(ctx);
        slime_1.entity_data.world_x = TILE_SIZE as i32 * 23;
        slime_1.entity_data.world_y = TILE_SIZE as i32 * 36;
        let mut slime_2 = MonGreenSlime::new(ctx);
        slime_2.entity_data.world_x = TILE_SIZE as i32 * 23;
        slime_2.entity_data.world_y = TILE_SIZE as i32 * 37;
        let mut slime_3 = MonGreenSlime::new(ctx);
        slime_3.entity_data.world_x = TILE_SIZE as i32 * 23;
        slime_3.entity_data.world_y = TILE_SIZE as i32 * 38;
        let mut slime_4 = MonGreenSlime::new(ctx);
        slime_4.entity_data.world_x = TILE_SIZE as i32 * 23;
        slime_4.entity_data.world_y = TILE_SIZE as i32 * 39;
        let monsters: Vec<Box<dyn GameEntity>> = vec![
            Box::new(slime_1),
            Box::new(slime_2),
            Box::new(slime_3),
            Box::new(slime_4),
        ];
        info!("Finished creating initial Monsters...");
        monsters
    }
}
