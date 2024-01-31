use ggez::{graphics, Context};

use crate::entities::entity::{EntityData, EntityType, GameEntity};

pub struct ObjHeart {
    pub entity_data: EntityData,
}

impl ObjHeart {
    pub fn new(ctx: &mut Context, world_x: i32, world_y: i32) -> Self {
        ObjHeart {
            entity_data: EntityData {
                image: Some(graphics::Image::from_path(ctx, "/objects/heart_full.png").unwrap()),
                image2: Some(graphics::Image::from_path(ctx, "/objects/heart_half.png").unwrap()),
                image3: Some(graphics::Image::from_path(ctx, "/objects/heart_blank.png").unwrap()),
                name: "Heart".to_string(),
                is_collidable: true,
                world_x,
                world_y,
                entity_type: EntityType::OBJECT,
                ..Default::default()
            },
        }
    }
}

impl GameEntity for ObjHeart {
    fn entity_data(&self) -> &EntityData {
        &self.entity_data
    }

    fn entity_data_mut(&mut self) -> &mut EntityData {
        &mut self.entity_data
    }
}
