use ggez::{graphics, Context};

use crate::entities::entity::{EntityData, EntityType, GameEntity};

pub struct ObjChest {
    pub entity_data: EntityData,
}

impl ObjChest {
    pub fn new(ctx: &mut Context, world_x: i32, world_y: i32) -> Self {
        ObjChest {
            entity_data: EntityData {
                image: Some(graphics::Image::from_path(ctx, "/objects/chest.png").unwrap()),
                name: "Chest".to_string(),
                is_collidable: false,
                world_x,
                world_y,
                entity_type: EntityType::OBJECT,
                ..Default::default()
            },
        }
    }
}

impl GameEntity for ObjChest {
    fn entity_data(&self) -> &crate::entities::entity::EntityData {
        &self.entity_data
    }

    fn entity_data_mut(&mut self) -> &mut crate::entities::entity::EntityData {
        &mut self.entity_data
    }
}
