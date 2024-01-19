use crate::{
    entities::{
        entity::{self, EntityData},
        object::HasObjectData,
    },
    tiles::tile::TileManager,
    TILE_SIZE,
};

pub struct CollisionChecker {}

impl CollisionChecker {
    pub fn check_tile(&self, entity: &mut EntityData, tile_manager: &TileManager) {
        let entity_left_world_x = entity.world_x as f32 + entity.solid_area.x;
        let entity_right_world_x =
            entity.world_x as f32 + entity.solid_area.x + entity.solid_area.w;
        let entity_top_world_y = entity.world_y as f32 + entity.solid_area.y;
        let entity_bottom_world_y =
            entity.world_y as f32 + entity.solid_area.y + entity.solid_area.h;

        let mut entity_left_col = entity_left_world_x / TILE_SIZE as f32;
        let mut entity_right_col = entity_right_world_x / TILE_SIZE as f32;
        let mut entity_top_row = entity_top_world_y / TILE_SIZE as f32;
        let mut entity_bottom_row = entity_bottom_world_y / TILE_SIZE as f32;

        let tile_num_1: u32;
        let tile_num_2: u32;

        match entity.direction {
            entity::Direction::Up => {
                entity_top_row = (entity_top_world_y - entity.speed as f32) / TILE_SIZE as f32;
                tile_num_1 =
                    tile_manager.map_tile_num[entity_top_row as usize][entity_left_col as usize];
                tile_num_2 =
                    tile_manager.map_tile_num[entity_top_row as usize][entity_right_col as usize];
                if tile_manager.tiles[tile_num_1 as usize].is_collidable
                    || tile_manager.tiles[tile_num_2 as usize].is_collidable
                {
                    entity.is_collision_on = true;
                }
            }
            entity::Direction::Down => {
                entity_bottom_row =
                    (entity_bottom_world_y + entity.speed as f32) / TILE_SIZE as f32;
                tile_num_1 =
                    tile_manager.map_tile_num[entity_bottom_row as usize][entity_left_col as usize];
                tile_num_2 = tile_manager.map_tile_num[entity_bottom_row as usize]
                    [entity_right_col as usize];
                if tile_manager.tiles[tile_num_1 as usize].is_collidable
                    || tile_manager.tiles[tile_num_2 as usize].is_collidable
                {
                    entity.is_collision_on = true;
                }
            }
            entity::Direction::Left => {
                entity_left_col = (entity_left_world_x - entity.speed as f32) / TILE_SIZE as f32;
                tile_num_1 =
                    tile_manager.map_tile_num[entity_top_row as usize][entity_left_col as usize];
                tile_num_2 =
                    tile_manager.map_tile_num[entity_bottom_row as usize][entity_left_col as usize];
                if tile_manager.tiles[tile_num_1 as usize].is_collidable
                    || tile_manager.tiles[tile_num_2 as usize].is_collidable
                {
                    entity.is_collision_on = true;
                }
            }
            entity::Direction::Right => {
                entity_right_col = (entity_right_world_x - entity.speed as f32) / TILE_SIZE as f32;
                tile_num_1 =
                    tile_manager.map_tile_num[entity_top_row as usize][entity_right_col as usize];
                tile_num_2 = tile_manager.map_tile_num[entity_bottom_row as usize]
                    [entity_right_col as usize];
                if tile_manager.tiles[tile_num_1 as usize].is_collidable
                    || tile_manager.tiles[tile_num_2 as usize].is_collidable
                {
                    entity.is_collision_on = true;
                }
            }
        }
    }
    pub fn check_object(
        &self,
        entity: &mut EntityData,
        is_entity_player: bool,
        objects: &mut [Box<dyn HasObjectData>],
    ) -> i32 {
        let mut index: i32 = 999;

        for (i, obj) in objects.iter_mut().enumerate() {
            entity.solid_area.x += entity.world_x as f32;
            entity.solid_area.y += entity.world_y as f32;

            obj.object_data_mut().solid_area.x =
                obj.object_data().world_x as f32 + obj.object_data().solid_area.x;
            obj.object_data_mut().solid_area.y =
                obj.object_data().world_y as f32 + obj.object_data().solid_area.y;

            match entity.direction {
                entity::Direction::Up => {
                    entity.solid_area.y -= entity.speed as f32;
                    if entity.solid_area.overlaps(&obj.object_data().solid_area) {
                        if obj.object_data().is_collidable {
                            entity.is_collision_on = true;
                        }
                        if is_entity_player {
                            index = i as i32;
                        }
                    }
                }
                entity::Direction::Down => {
                    entity.solid_area.y += entity.speed as f32;
                    if entity.solid_area.overlaps(&obj.object_data().solid_area) {
                        if obj.object_data().is_collidable {
                            entity.is_collision_on = true;
                        }
                        if is_entity_player {
                            index = i as i32;
                        }
                    }
                }
                entity::Direction::Left => {
                    entity.solid_area.x -= entity.speed as f32;
                    if entity.solid_area.overlaps(&obj.object_data().solid_area) {
                        if obj.object_data().is_collidable {
                            entity.is_collision_on = true;
                        }
                        if is_entity_player {
                            index = i as i32;
                        }
                    }
                }
                entity::Direction::Right => {
                    entity.solid_area.x += entity.speed as f32;
                    if entity.solid_area.overlaps(&obj.object_data().solid_area) {
                        if obj.object_data().is_collidable {
                            entity.is_collision_on = true;
                        }
                        if is_entity_player {
                            index = i as i32;
                        }
                    }
                }
            }
            entity.solid_area.x = entity.solid_area_default_x as f32;
            entity.solid_area.y = entity.solid_area_default_y as f32;
            obj.object_data_mut().solid_area.x = obj.object_data().solid_area_default_x as f32;
            obj.object_data_mut().solid_area.y = obj.object_data().solid_area_default_y as f32;
        }

        index
    }
}
