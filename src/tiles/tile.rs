use ggez::{
    glam::Vec2,
    graphics::{self, Canvas, Image},
    Context,
};
use log::info;

use crate::{MAX_SCREEN_COL, MAX_SCREEN_ROW, SCALE, TILE_SIZE};

#[derive(Debug, Default)]
pub struct TileData {
    pub image: Option<Image>,
    pub is_collidable: bool,
}

#[derive(Debug)]
pub struct TileManager {
    tiles: Vec<TileData>,
}

impl TileManager {
    pub fn new(ctx: &mut Context) -> Self {
        let mut tile_manager = TileManager {
            tiles: Vec::with_capacity(10),
        };
        tile_manager.get_tile_images(ctx);
        tile_manager
    }

    fn get_tile_images(&mut self, ctx: &mut Context) {
        info!("Loading tile images...");
        info!("Loading grass tile image");
        let grass_tile_image = graphics::Image::from_path(ctx, "/tiles/grass.png").unwrap();
        self.tiles.push(TileData {
            image: Some(grass_tile_image),
            is_collidable: false,
        });
        info!("Loading wall tile image");
        let wall_tile_image = graphics::Image::from_path(ctx, "/tiles/wall.png").unwrap();
        self.tiles.push(TileData {
            image: Some(wall_tile_image),
            is_collidable: false,
        });
        info!("Loading water tile image");
        let water_tile_image = graphics::Image::from_path(ctx, "/tiles/water.png").unwrap();
        self.tiles.push(TileData {
            image: Some(water_tile_image),
            is_collidable: false,
        });
        info!("Finished loading tile images...")
    }

    pub fn draw(&self, ctx: &Context, canvas: &mut Canvas) {
        // match self.tiles.first() {
        //     Some(tile_data) => match &tile_data.image {
        //         Some(tile_image) => canvas.draw(
        //             tile_image,
        //             graphics::DrawParam::new()
        //                 .dest(Vec2::new(0.0, 0.0))
        //                 .scale(Vec2::new(SCALE as f32, SCALE as f32)),
        //         ),
        //         None => todo!(),
        //     },
        //     None => {
        //         todo!()
        //     }
        // }

        // match self.tiles.get(1) {
        //     Some(tile_data) => match &tile_data.image {
        //         Some(tile_image) => canvas.draw(
        //             tile_image,
        //             graphics::DrawParam::new()
        //                 .dest(Vec2::new(48.0, 0.0))
        //                 .scale(Vec2::new(SCALE as f32, SCALE as f32)),
        //         ),
        //         None => todo!(),
        //     },
        //     None => {
        //         todo!()
        //     }
        // }

        // match self.tiles.get(2) {
        //     Some(tile_data) => match &tile_data.image {
        //         Some(tile_image) => canvas.draw(
        //             tile_image,
        //             graphics::DrawParam::new()
        //                 .dest(Vec2::new(96.0, 0.0))
        //                 .scale(Vec2::new(SCALE as f32, SCALE as f32)),
        //         ),
        //         None => todo!(),
        //     },
        //     None => {
        //         todo!()
        //     }
        // }

        let mut col: u32 = 0;
        let mut row: u32 = 0;
        let mut x: u32 = 0;
        let mut y: u32 = 0;

        while col < MAX_SCREEN_COL.into() && row < MAX_SCREEN_ROW.into() {
            match self.tiles.get(0) {
                Some(tile_data) => match &tile_data.image {
                    Some(tile_image) => canvas.draw(
                        tile_image,
                        graphics::DrawParam::new()
                            .dest(Vec2::new(x as f32, y as f32))
                            .scale(Vec2::new(SCALE as f32, SCALE as f32)),
                    ),
                    None => todo!(),
                },
                None => {
                    todo!()
                }
            }
            col += 1;
            x += TILE_SIZE as u32;

            if col == MAX_SCREEN_COL.into() {
                col = 0;
                x = 0;
                row += 1;
                y += TILE_SIZE as u32
            }
        }
    }
}
