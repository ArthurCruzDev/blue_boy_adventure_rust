use ggez::{graphics, Context};

use crate::entities::entity::{EntityData, GameEntity};

pub struct ObjBoots {
    pub entity_data: EntityData,
}

impl ObjBoots {
    pub fn new(ctx: &mut Context, world_x: i32, world_y: i32) -> Self {
        ObjBoots {
            entity_data: EntityData {
                image: Some(graphics::Image::from_path(ctx, "/objects/boots.png").unwrap()),
                name: "Boots".to_string(),
                is_collidable: false,
                world_x,
                world_y,
                ..EntityData::default()
            },
        }
    }
}

impl GameEntity for ObjBoots {
    fn entity_data(&self) -> &EntityData {
        &self.entity_data
    }

    fn entity_data_mut(&mut self) -> &mut EntityData {
        &mut self.entity_data
    }
}
