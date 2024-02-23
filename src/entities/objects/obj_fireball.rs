use ggez::{graphics, Context};

use crate::entities::{
    entity::{EntityData, EntityType, GameEntity},
    projectiles::projectile::Projectile,
};

pub struct ObjFireball {
    pub entity_data: EntityData,
}

impl ObjFireball {
    pub fn new(ctx: &mut Context) -> Self {
        ObjFireball {
            entity_data: EntityData {
                down_1: Some(
                    graphics::Image::from_path(ctx, "/projectile/fireball_down_1.png").unwrap(),
                ),
                down_2: Some(
                    graphics::Image::from_path(ctx, "/projectile/fireball_down_2.png").unwrap(),
                ),
                up_1: Some(
                    graphics::Image::from_path(ctx, "/projectile/fireball_up_1.png").unwrap(),
                ),
                up_2: Some(
                    graphics::Image::from_path(ctx, "/projectile/fireball_up_2.png").unwrap(),
                ),
                left_1: Some(
                    graphics::Image::from_path(ctx, "/projectile/fireball_left_1.png").unwrap(),
                ),
                left_2: Some(
                    graphics::Image::from_path(ctx, "/projectile/fireball_left_2.png").unwrap(),
                ),
                right_1: Some(
                    graphics::Image::from_path(ctx, "/projectile/fireball_right_1.png").unwrap(),
                ),
                right_2: Some(
                    graphics::Image::from_path(ctx, "/projectile/fireball_right_2.png").unwrap(),
                ),
                name: "Fireball".to_string(),
                speed: 10,
                max_life: 80,
                life: 80,
                attack: 2,
                use_cost: 1,
                alive: false,
                is_collidable: true,
                entity_type: EntityType::SHIELD,
                ..Default::default()
            },
        }
    }
    pub fn new_positioned(ctx: &mut Context, world_x: i32, world_y: i32) -> Self {
        let mut fireball = ObjFireball::new(ctx);
        fireball.entity_data.world_x = world_x;
        fireball.entity_data.world_y = world_y;
        fireball
    }
}

impl GameEntity for ObjFireball {
    fn entity_data(&self) -> &EntityData {
        &self.entity_data
    }

    fn entity_data_mut(&mut self) -> &mut EntityData {
        &mut self.entity_data
    }
}

impl Projectile for ObjFireball {}
