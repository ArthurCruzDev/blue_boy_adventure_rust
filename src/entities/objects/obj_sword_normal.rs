use ggez::{
    graphics::{self, Rect},
    Context,
};

use crate::entities::entity::{EntityData, EntityType, GameEntity};

pub struct ObjSwordNormal {
    pub entity_data: EntityData,
}

impl ObjSwordNormal {
    pub fn new(ctx: &mut Context) -> Self {
        ObjSwordNormal {
            entity_data: EntityData {
                down_1: Some(graphics::Image::from_path(ctx, "/objects/sword_normal.png").unwrap()),
                name: "Normal Sword".to_string(),
                is_collidable: false,
                attack_value: 1,
                description: "[Normal Sword]\nAn old sword.".to_owned(),
                attack_area: Rect {
                    x: 0.0,
                    y: 0.0,
                    w: 36.0,
                    h: 36.0,
                },
                entity_type: EntityType::SWORD,
                ..Default::default()
            },
        }
    }
}

impl GameEntity for ObjSwordNormal {
    fn entity_data(&self) -> &EntityData {
        &self.entity_data
    }

    fn entity_data_mut(&mut self) -> &mut EntityData {
        &mut self.entity_data
    }
}
