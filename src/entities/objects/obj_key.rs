use ggez::{graphics, Context};

use crate::entities::object::{HasObjectData, ObjectData};

pub struct ObjKey {
    pub object_data: ObjectData,
}

impl ObjKey {
    pub fn new(ctx: &mut Context, world_x: i32, world_y: i32) -> Self {
        ObjKey {
            object_data: ObjectData {
                image: Some(graphics::Image::from_path(ctx, "/objects/key.png").unwrap()),
                name: "Key".to_string(),
                is_collidable: false,
                world_x,
                world_y,
                ..ObjectData::default()
            },
        }
    }
}

impl HasObjectData for ObjKey {
    fn object_data(&self) -> &ObjectData {
        &self.object_data
    }

    fn object_data_mut(&mut self) -> &mut ObjectData {
        &mut self.object_data
    }
}
