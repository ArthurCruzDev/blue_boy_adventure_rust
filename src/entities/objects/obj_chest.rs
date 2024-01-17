use ggez::{graphics, Context};

use crate::entities::object::{HasObjectData, ObjectData};

pub struct ObjChest {
    pub object_data: ObjectData,
}

impl ObjChest {
    pub fn new(ctx: &mut Context, world_x: i32, world_y: i32) -> Self {
        ObjChest {
            object_data: ObjectData {
                image: Some(graphics::Image::from_path(ctx, "/objects/chest.png").unwrap()),
                name: "Chest".to_string(),
                is_collidable: false,
                world_x,
                world_y,
            },
        }
    }
}

impl HasObjectData for ObjChest {
    fn object_data(&self) -> &ObjectData {
        &self.object_data
    }

    fn object_data_mut(&mut self) -> &mut ObjectData {
        &mut self.object_data
    }
}
