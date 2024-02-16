use ggez::{graphics, Context};

use crate::entities::entity::{EntityData, EntityType, GameEntity};

pub struct ObjShieldBlue {
    pub entity_data: EntityData,
}

impl ObjShieldBlue {
    pub fn new(ctx: &mut Context) -> Self {
        ObjShieldBlue {
            entity_data: EntityData {
                down_1: Some(graphics::Image::from_path(ctx, "/objects/shield_blue.png").unwrap()),
                name: "Blue Shield".to_string(),
                is_collidable: false,
                entity_type: EntityType::SHIELD,
                defense_value: 2,
                description: "[Blue Shield]\nA shiny blue shield.".to_owned(),
                ..Default::default()
            },
        }
    }
    pub fn new_positioned(ctx: &mut Context, world_x: i32, world_y: i32) -> Self {
        let mut shield = ObjShieldBlue::new(ctx);
        shield.entity_data.world_x = world_x;
        shield.entity_data.world_y = world_y;
        shield
    }
}

impl GameEntity for ObjShieldBlue {
    fn entity_data(&self) -> &crate::entities::entity::EntityData {
        &self.entity_data
    }

    fn entity_data_mut(&mut self) -> &mut crate::entities::entity::EntityData {
        &mut self.entity_data
    }
}
