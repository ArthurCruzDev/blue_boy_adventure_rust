use crate::{
    entities::{
        entity::{self, EntityData, GameEntity},
        player::Player,
    },
    tiles::tile::TileManager,
    TILE_SIZE,
};

pub fn check_tile(entity: &EntityData, tile_manager: &TileManager) -> bool {
    let entity_left_world_x = entity.world_x as f32 + entity.solid_area.x;
    let entity_right_world_x = entity.world_x as f32 + entity.solid_area.x + entity.solid_area.w;
    let entity_top_world_y = entity.world_y as f32 + entity.solid_area.y;
    let entity_bottom_world_y = entity.world_y as f32 + entity.solid_area.y + entity.solid_area.h;

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
                return true;
            }
        }
        entity::Direction::Down => {
            entity_bottom_row = (entity_bottom_world_y + entity.speed as f32) / TILE_SIZE as f32;
            tile_num_1 =
                tile_manager.map_tile_num[entity_bottom_row as usize][entity_left_col as usize];
            tile_num_2 =
                tile_manager.map_tile_num[entity_bottom_row as usize][entity_right_col as usize];
            if tile_manager.tiles[tile_num_1 as usize].is_collidable
                || tile_manager.tiles[tile_num_2 as usize].is_collidable
            {
                return true;
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
                return true;
            }
        }
        entity::Direction::Right => {
            entity_right_col = (entity_right_world_x - entity.speed as f32) / TILE_SIZE as f32;
            tile_num_1 =
                tile_manager.map_tile_num[entity_top_row as usize][entity_right_col as usize];
            tile_num_2 =
                tile_manager.map_tile_num[entity_bottom_row as usize][entity_right_col as usize];
            if tile_manager.tiles[tile_num_1 as usize].is_collidable
                || tile_manager.tiles[tile_num_2 as usize].is_collidable
            {
                return true;
            }
        }
    }
    false
}

pub fn check_object(entity: &EntityData, objects: &[Box<dyn GameEntity>]) -> Option<i32> {
    for (i, obj) in objects.iter().enumerate() {
        let mut entity_solid_area = entity.solid_area;
        entity_solid_area.x += entity.world_x as f32;
        entity_solid_area.y += entity.world_y as f32;

        let mut obj_solid_area = obj.entity_data().solid_area;
        obj_solid_area.x += obj.entity_data().world_x as f32;
        obj_solid_area.y += obj.entity_data().world_y as f32;

        match entity.direction {
            entity::Direction::Up => {
                entity_solid_area.y -= entity.speed as f32;
            }
            entity::Direction::Down => {
                entity_solid_area.y += entity.speed as f32;
            }
            entity::Direction::Left => {
                entity_solid_area.x -= entity.speed as f32;
            }
            entity::Direction::Right => {
                entity_solid_area.x += entity.speed as f32;
            }
        }
        if entity_solid_area.overlaps(&obj_solid_area) && obj.entity_data().is_collidable {
            return Some(i as i32);
        }
    }

    None
}

pub fn check_entity(entity: &EntityData, targets: &[Box<dyn GameEntity>]) -> Option<i32> {
    for (i, target) in targets.iter().enumerate() {
        let mut entity_solid_area = entity.solid_area;
        entity_solid_area.x += entity.world_x as f32;
        entity_solid_area.y += entity.world_y as f32;

        let mut target_solid_area = target.entity_data().solid_area;
        target_solid_area.x += target.entity_data().world_x as f32;
        target_solid_area.y += target.entity_data().world_y as f32;

        match entity.direction {
            entity::Direction::Up => {
                entity_solid_area.y -= entity.speed as f32;
            }
            entity::Direction::Down => {
                entity_solid_area.y += entity.speed as f32;
            }
            entity::Direction::Left => {
                entity_solid_area.x -= entity.speed as f32;
            }
            entity::Direction::Right => {
                entity_solid_area.x += entity.speed as f32;
            }
        }

        if entity_solid_area.overlaps(&target_solid_area) && entity != target.entity_data() {
            return Some(i as i32);
        }
    }

    None
}

pub fn check_player(entity: &EntityData, player: &Player) -> bool {
    let mut entity_solid_area = entity.solid_area;
    entity_solid_area.x += entity.world_x as f32;
    entity_solid_area.y += entity.world_y as f32;

    let mut player_solid_area = player.entity_data().solid_area;
    player_solid_area.x += player.entity_data().world_x as f32;
    player_solid_area.y += player.entity_data().world_y as f32;

    match entity.direction {
        entity::Direction::Up => {
            entity_solid_area.y -= entity.speed as f32;
            if entity_solid_area.overlaps(&player_solid_area) {
                return true;
            }
        }
        entity::Direction::Down => {
            entity_solid_area.y += entity.speed as f32;
            if entity_solid_area.overlaps(&player_solid_area) {
                return true;
            }
        }
        entity::Direction::Left => {
            entity_solid_area.x -= entity.speed as f32;
            if entity_solid_area.overlaps(&player_solid_area) {
                return true;
            }
        }
        entity::Direction::Right => {
            entity_solid_area.x += entity.speed as f32;
            if entity_solid_area.overlaps(&player_solid_area) {
                return true;
            }
        }
    }
    false
}
