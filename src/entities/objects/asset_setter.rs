use ggez::{graphics::Canvas, Context};
use log::info;

use crate::{
    entities::{
        object::HasObjectData,
        objects::{obj_chest::ObjChest, obj_door::ObjDoor},
        player::Player,
    },
    TILE_SIZE,
};

use super::obj_key::ObjKey;

pub struct AssetSetter {
    pub current_objects: Vec<Box<dyn HasObjectData>>,
}

impl AssetSetter {
    pub fn new(ctx: &mut Context) -> Self {
        info!("Initializing Asset Setter...");
        info!("Creating initial objects...");
        let current_objects: Vec<Box<dyn HasObjectData>> = vec![
            Box::new(ObjKey::new(
                ctx,
                23 * TILE_SIZE as i32,
                7 * TILE_SIZE as i32,
            )),
            Box::new(ObjKey::new(
                ctx,
                23 * TILE_SIZE as i32,
                40 * TILE_SIZE as i32,
            )),
            Box::new(ObjKey::new(
                ctx,
                38 * TILE_SIZE as i32,
                8 * TILE_SIZE as i32,
            )),
            Box::new(ObjDoor::new(
                ctx,
                10 * TILE_SIZE as i32,
                11 * TILE_SIZE as i32,
            )),
            Box::new(ObjDoor::new(
                ctx,
                8 * TILE_SIZE as i32,
                28 * TILE_SIZE as i32,
            )),
            Box::new(ObjDoor::new(
                ctx,
                12 * TILE_SIZE as i32,
                22 * TILE_SIZE as i32,
            )),
            Box::new(ObjChest::new(
                ctx,
                10 * TILE_SIZE as i32,
                7 * TILE_SIZE as i32,
            )),
        ];
        info!("Finished creating initial objects...");
        info!("Asset Setter initialized...");
        AssetSetter { current_objects }
    }

    pub fn draw(&self, ctx: &mut Context, canvas: &mut Canvas, player: &Player) {
        self.current_objects.iter().for_each(|obj| {
            if obj.object_data().image.is_some() {
                obj.object_data().draw(ctx, canvas, player);
            }
        });
    }
}
