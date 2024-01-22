use ggez::{graphics, Context};

use crate::entities::object::{HasObjectData, ObjectData};

pub struct ObjHeart {
    pub object_data: ObjectData,
}

impl ObjDoor {
    pub fn new(ctx: &mut Context, world_x: i32, world_y: i32) -> Self {
        ObjDoor {
            object_data: ObjectData {
                image: Some(graphics::Image::from_path(ctx, "/objects/heart_full.png").unwrap()),
                image2: Some(graphics::Image::from_path(ctx, "/objects/heart_half.png").unwrap()),
                image3: Some(graphics::Image::from_path(ctx, "/objects/heart_blank.png").unwrap()),
                name: "Heart".to_string(),
                is_collidable: true,
                world_x,
                world_y,
                ..Default::default()
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
