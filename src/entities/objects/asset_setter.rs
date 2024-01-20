use ggez::{graphics::Canvas, Context};
use log::info;

use crate::{
    entities::{
        object::HasObjectData,
        objects::{obj_boots::ObjBoots, obj_chest::ObjChest, obj_door::ObjDoor},
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
        let current_objects: Vec<Box<dyn HasObjectData>> = vec![];
        info!("Finished creating initial objects...");
        info!("Asset Setter initialized...");
        AssetSetter { current_objects }
    }

    pub fn draw(&self, canvas: &mut Canvas, player: &Player) {
        self.current_objects.iter().for_each(|obj| {
            if obj.object_data().image.is_some() {
                obj.object_data().draw(canvas, player);
            }
        });
    }
}
