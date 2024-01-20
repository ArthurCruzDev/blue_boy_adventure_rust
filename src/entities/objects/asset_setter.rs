use ggez::{graphics::Canvas, Context};
use log::info;

use crate::{
    entities::{
        entity::GameEntity, npc::npc_old_man::NPCOldMan, object::HasObjectData, player::Player,
    },
    GameHandlers, TILE_SIZE,
};

pub struct AssetSetter {}

impl AssetSetter {
    pub fn initialize_objects() -> Vec<Box<dyn HasObjectData>> {
        info!("Creating initial objects...");
        let current_objects: Vec<Box<dyn HasObjectData>> = vec![];
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

    pub fn draw_objects(objects: &[Box<dyn HasObjectData>], canvas: &mut Canvas, player: &Player) {
        objects.iter().for_each(|obj| {
            if obj.object_data().image.is_some() {
                obj.object_data().draw(canvas, player);
            }
        });
    }

    pub fn draw_npcs(npcs: &[Box<dyn GameEntity>], canvas: &mut Canvas, player: &Player) {
        npcs.iter().for_each(|npc| npc.draw(canvas, player))
    }

    pub fn update_npcs(
        npcs: &mut [Box<dyn GameEntity>],
        objects: &mut Vec<Box<dyn HasObjectData>>,
        game_handlers: &mut GameHandlers,
        ctx: &mut Context,
        player: &mut Player,
    ) {
        npcs.iter_mut()
            .for_each(|npc| npc.update(game_handlers, ctx, objects, &mut Vec::default(), player))
    }
}
