use ggez::{graphics, Context};

use crate::entities::entity::{EntityData, EntityType, GameEntity};

pub struct ObjShieldWood {
    pub entity_data: EntityData,
}

impl ObjShieldWood {
    pub fn new(ctx: &mut Context) -> Self {
        ObjShieldWood {
            entity_data: EntityData {
                down_1: Some(graphics::Image::from_path(ctx, "/objects/shield_wood.png").unwrap()),
                name: "Wood Sword".to_string(),
                is_collidable: false,
                entity_type: EntityType::OBJECT,
                defense_value: 1,
                description: "[Wood Sword]\nMade of wood.".to_owned(),
                ..Default::default()
            },
        }
    }
}

impl GameEntity for ObjShieldWood {
    fn entity_data(&self) -> &crate::entities::entity::EntityData {
        &self.entity_data
    }

    fn entity_data_mut(&mut self) -> &mut crate::entities::entity::EntityData {
        &mut self.entity_data
    }
}
