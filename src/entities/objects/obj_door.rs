use ggez::{graphics, Context};

use crate::entities::object::{HasObjectData, ObjectData};

pub struct ObjDoor {
    pub object_data: ObjectData,
}

impl ObjDoor {
    pub fn new(ctx: &mut Context, world_x: i32, world_y: i32) -> Self {
        ObjDoor {
            object_data: ObjectData {
                image: Some(graphics::Image::from_path(ctx, "/objects/door.png").unwrap()),
                name: "Door".to_string(),
                is_collidable: false,
                world_x,
                world_y,
            },
        }
    }
}

impl HasObjectData for ObjDoor {
    fn object_data(&self) -> &ObjectData {
        &self.object_data
    }

    fn object_data_mut(&mut self) -> &mut ObjectData {
        &mut self.object_data
    }
}
