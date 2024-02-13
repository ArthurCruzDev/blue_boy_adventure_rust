use ggez::{graphics, Context};

use crate::entities::entity::{EntityData, EntityType, GameEntity};

pub struct ObjKey {
    pub entity_data: EntityData,
}

impl ObjKey {
    pub fn new(ctx: &mut Context, world_x: i32, world_y: i32) -> Self {
        ObjKey {
            entity_data: EntityData {
                down_1: Some(graphics::Image::from_path(ctx, "/objects/key.png").unwrap()),
                name: "Key".to_string(),
                is_collidable: false,
                world_x,
                world_y,
                entity_type: EntityType::OBJECT,
                description: "[Key]\nIt opens a door.".to_owned(),
                ..EntityData::default()
            },
        }
    }
}

impl GameEntity for ObjKey {
    fn entity_data(&self) -> &EntityData {
        &self.entity_data
    }

    fn entity_data_mut(&mut self) -> &mut EntityData {
        &mut self.entity_data
    }
}
