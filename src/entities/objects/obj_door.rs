use ggez::{graphics, Context};

use crate::entities::entity::{EntityData, GameEntity};

pub struct ObjDoor {
    pub entity_data: EntityData,
}

impl ObjDoor {
    pub fn new(ctx: &mut Context, world_x: i32, world_y: i32) -> Self {
        ObjDoor {
            entity_data: EntityData {
                image: Some(graphics::Image::from_path(ctx, "/objects/door.png").unwrap()),
                name: "Door".to_string(),
                is_collidable: true,
                world_x,
                world_y,
                ..Default::default()
            },
        }
    }
}

impl GameEntity for ObjDoor {
    fn entity_data(&self) -> &EntityData {
        &self.entity_data
    }

    fn entity_data_mut(&mut self) -> &mut EntityData {
        &mut self.entity_data
    }
}
