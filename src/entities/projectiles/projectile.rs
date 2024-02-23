use log::info;

use crate::entities::{
    entity::{Direction, EntityData, EntityType, GameEntity},
    player::Player,
};

pub trait Projectile: GameEntity {
    fn set(
        &mut self,
        world_x: i32,
        world_y: i32,
        direction: Direction,
        alive: bool,
        throw_by: EntityType,
    ) {
        let entity_data = self.entity_data_mut();
        entity_data.world_x = world_x;
        entity_data.world_y = world_y;
        entity_data.direction = direction;
        entity_data.alive = alive;
        entity_data.projectile_thrown_by = Some(throw_by);
    }

    fn update(&mut self) {
        let entity_data = self.entity_data_mut();
        match entity_data.direction {
            Direction::UP => entity_data.world_y -= entity_data.speed,
            Direction::DOWN => entity_data.world_y += entity_data.speed,
            Direction::LEFT => entity_data.world_x -= entity_data.speed,
            Direction::RIGHT => entity_data.world_x += entity_data.speed,
        }

        entity_data.life -= 1;
        if entity_data.life <= 0 {
            entity_data.alive = false;
        }

        entity_data.sprite_counter += 1;
        if entity_data.sprite_counter > 12 {
            if entity_data.sprite_num == 1 {
                entity_data.sprite_num = 2;
            } else if entity_data.sprite_num == 2 {
                entity_data.sprite_num = 1;
            }
            entity_data.sprite_counter = 0;
        }
    }
}
