use ggez::{
    glam::Vec2,
    graphics::{self, Canvas, Image},
    Context,
};

use crate::{SCALE, TILE_SIZE};

use super::player::Player;

pub struct ObjectData {
    pub image: Option<Image>,
    pub name: String,
    pub is_collidable: bool,
    pub world_x: i32,
    pub world_y: i32,
}

impl ObjectData {
    pub fn draw(&self, ctx: &Context, canvas: &mut Canvas, player: &Player) {
        let screen_x = self.world_x - player.entity.world_x + player.screen_x as i32;
        let screen_y = self.world_y - player.entity.world_y + player.screen_y as i32;

        if self.world_x + (TILE_SIZE as i32) > player.entity.world_x - player.screen_x as i32
            && self.world_x - (TILE_SIZE as i32) < player.entity.world_x + player.screen_x as i32
            && self.world_y + (TILE_SIZE as i32) > player.entity.world_y - player.screen_y as i32
            && self.world_y - (TILE_SIZE as i32) < player.entity.world_y + player.screen_y as i32
        {
            match &self.image {
                Some(image) => canvas.draw(
                    image,
                    graphics::DrawParam::new()
                        .dest(Vec2::new(screen_x as f32, screen_y as f32))
                        .scale(Vec2::new(SCALE as f32, SCALE as f32)),
                ),
                None => {
                    todo!()
                }
            }
        }
    }
}

pub trait HasObjectData {
    fn object_data(&self) -> &ObjectData;
    fn object_data_mut(&mut self) -> &mut ObjectData;
}
