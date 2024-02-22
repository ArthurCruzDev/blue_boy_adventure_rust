use ggez::{
    graphics::{self, Rect},
    Context,
};

use crate::entities::entity::{EntityData, EntityType, GameEntity};

pub struct ObjAxe {
    pub entity_data: EntityData,
}

impl ObjAxe {
    pub fn new(ctx: &mut Context) -> Self {
        ObjAxe {
            entity_data: EntityData {
                down_1: Some(graphics::Image::from_path(ctx, "/objects/axe.png").unwrap()),
                name: "Woodcutter`s Axe".to_string(),
                is_collidable: false,
                entity_type: EntityType::AXE,
                attack_value: 2,
                description: "[Woodcutter`s Axe]\nA bit rusty but still can cut some trees."
                    .to_owned(),
                attack_area: Rect {
                    x: 0.0,
                    y: 0.0,
                    w: 30.0,
                    h: 30.0,
                },
                ..Default::default()
            },
        }
    }
    pub fn new_positioned(ctx: &mut Context, world_x: i32, world_y: i32) -> Self {
        let mut axe = ObjAxe::new(ctx);
        axe.entity_data.world_x = world_x;
        axe.entity_data.world_y = world_y;
        axe
    }
}

impl GameEntity for ObjAxe {
    fn entity_data(&self) -> &EntityData {
        &self.entity_data
    }

    fn entity_data_mut(&mut self) -> &mut EntityData {
        &mut self.entity_data
    }
}
