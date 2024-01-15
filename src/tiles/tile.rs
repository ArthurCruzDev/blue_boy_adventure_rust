use std::io::{BufRead, BufReader};

use ggez::{
    glam::Vec2,
    graphics::{self, Canvas, Image, InstanceArray},
    Context,
};
use log::{error, info};

use crate::{MAX_SCREEN_COL, MAX_SCREEN_ROW, SCALE, TILE_SIZE};

#[derive(Debug, Default)]
pub struct TileData {
    pub image: Option<Image>,
    pub is_collidable: bool,
}

#[derive(Debug)]
pub struct TileManager {
    tiles: Vec<TileData>,
    map_tile_num: [[u32; MAX_SCREEN_COL as usize]; MAX_SCREEN_ROW as usize],
}

impl TileManager {
    pub fn new(ctx: &mut Context) -> Self {
        let mut tile_manager = TileManager {
            tiles: Vec::with_capacity(10),
            map_tile_num: [[0; MAX_SCREEN_COL as usize]; MAX_SCREEN_ROW as usize],
        };
        tile_manager.get_tile_images(ctx);
        tile_manager.load_map(ctx, "/maps/map01.txt");
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
        info!("Loading earth tile image");
        let earth_tile_image = graphics::Image::from_path(ctx, "/tiles/earth.png").unwrap();
        self.tiles.push(TileData {
            image: Some(earth_tile_image),
            is_collidable: false,
        });
        info!("Loading tree tile image");
        let tree_tile_image = graphics::Image::from_path(ctx, "/tiles/tree.png").unwrap();
        self.tiles.push(TileData {
            image: Some(tree_tile_image),
            is_collidable: false,
        });
        info!("Loading sand tile image");
        let sand_tile_image = graphics::Image::from_path(ctx, "/tiles/sand.png").unwrap();
        self.tiles.push(TileData {
            image: Some(sand_tile_image),
            is_collidable: false,
        });
        info!("Finished loading tile images...")
    }

    fn load_map(&mut self, ctx: &Context, map_path: &str) {
        info!("Loading the world Map...");
        let map_file = ctx.fs.open(map_path).unwrap();

        let mut reader = BufReader::new(map_file);

        let mut col: u32 = 0;
        let mut row: u32 = 0;

        let mut line: String = String::default();

        while col < MAX_SCREEN_COL.into() && row < MAX_SCREEN_ROW.into() {
            line.clear();
            match reader.read_line(&mut line) {
                Ok(_bytes) => {}
                Err(_) => {
                    error!("Failed to read map file");
                }
            };
            let line_numbers = line
                .trim_end()
                .split(' ')
                .map(|num| num.parse::<u32>().unwrap())
                .collect::<Vec<u32>>();

            for i in 0..line_numbers.len() {
                self.map_tile_num[row as usize][col as usize] = *line_numbers.get(i).unwrap();
                col += 1;
            }
            row += 1;
            col = 0;
        }
        info!("Finished loading the world Map")
    }

    pub fn draw(&self, ctx: &Context, canvas: &mut Canvas) {
        let mut col: u32 = 0;
        let mut row: u32 = 0;
        let mut x: u32 = 0;
        let mut y: u32 = 0;

        let mut instance_arrays: Vec<InstanceArray> = self
            .tiles
            .iter()
            .map(|tile_data| {
                InstanceArray::new(
                    ctx,
                    match &tile_data.image {
                        Some(image) => image.clone(),
                        None => {
                            todo!()
                        }
                    },
                )
            })
            .collect::<Vec<InstanceArray>>();

        while col < MAX_SCREEN_COL.into() && row < MAX_SCREEN_ROW.into() {
            let tileNum = self.map_tile_num[row as usize][col as usize];

            match instance_arrays.get_mut(tileNum as usize) {
                Some(instance_array) => instance_array.push(
                    graphics::DrawParam::new()
                        .dest(Vec2::new(x as f32, y as f32))
                        .scale(Vec2::new(SCALE as f32, SCALE as f32)),
                ),
                None => todo!(),
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
        instance_arrays
            .iter()
            .for_each(|instance_array| canvas.draw(instance_array, graphics::DrawParam::new()));
    }
}
