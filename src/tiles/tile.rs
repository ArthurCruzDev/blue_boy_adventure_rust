use std::io::{BufRead, BufReader};

use ggez::{
    glam::Vec2,
    graphics::{self, Canvas, Image, InstanceArray},
    Context,
};
use log::{error, info};

use crate::{
    entities::player::Player, MAX_SCREEN_COL, MAX_SCREEN_ROW, MAX_WORLD_COL, MAX_WORLD_ROW, SCALE,
    TILE_SIZE,
};

#[derive(Debug, Default)]
pub struct TileData {
    pub image: Option<Image>,
    pub is_collidable: bool,
}

#[derive(Debug)]
pub struct TileManager {
    pub tiles: Vec<TileData>,
    pub map_tile_num: [[u32; MAX_WORLD_COL as usize]; MAX_WORLD_ROW as usize],
}

impl TileManager {
    pub fn new(ctx: &mut Context) -> Self {
        let mut tile_manager = TileManager {
            tiles: Vec::with_capacity(50),
            map_tile_num: [[0; MAX_WORLD_COL as usize]; MAX_WORLD_ROW as usize],
        };
        tile_manager.get_tile_images(ctx);
        tile_manager.load_map(ctx, "/maps/worldV2.txt");
        tile_manager
    }

    fn get_tile_images(&mut self, ctx: &mut Context) {
        info!("Loading tile images...");
        let grass_tile_image = graphics::Image::from_path(ctx, "/tiles/grass00.png").unwrap();
        //PLACEHOLDER TILES
        self.tiles.push(TileData {
            image: Some(grass_tile_image.clone()),
            is_collidable: false,
        });
        self.tiles.push(TileData {
            image: Some(grass_tile_image.clone()),
            is_collidable: false,
        });
        self.tiles.push(TileData {
            image: Some(grass_tile_image.clone()),
            is_collidable: false,
        });
        self.tiles.push(TileData {
            image: Some(grass_tile_image.clone()),
            is_collidable: false,
        });
        self.tiles.push(TileData {
            image: Some(grass_tile_image.clone()),
            is_collidable: false,
        });
        self.tiles.push(TileData {
            image: Some(grass_tile_image.clone()),
            is_collidable: false,
        });
        self.tiles.push(TileData {
            image: Some(grass_tile_image.clone()),
            is_collidable: false,
        });
        self.tiles.push(TileData {
            image: Some(grass_tile_image.clone()),
            is_collidable: false,
        });
        self.tiles.push(TileData {
            image: Some(grass_tile_image.clone()),
            is_collidable: false,
        });
        self.tiles.push(TileData {
            image: Some(grass_tile_image.clone()),
            is_collidable: false,
        });
        //END PLACEHOLDER TILES
        //GRASS
        self.tiles.push(TileData {
            image: Some(grass_tile_image.clone()),
            is_collidable: false,
        });
        let grass_tile_image_01 = graphics::Image::from_path(ctx, "/tiles/grass01.png").unwrap();
        self.tiles.push(TileData {
            image: Some(grass_tile_image_01),
            is_collidable: false,
        });
        //WATER
        let water_tile_image_00 = graphics::Image::from_path(ctx, "/tiles/water00.png").unwrap();
        self.tiles.push(TileData {
            image: Some(water_tile_image_00),
            is_collidable: true,
        });
        let water_tile_image_01 = graphics::Image::from_path(ctx, "/tiles/water01.png").unwrap();
        self.tiles.push(TileData {
            image: Some(water_tile_image_01),
            is_collidable: true,
        });
        let water_tile_image_02 = graphics::Image::from_path(ctx, "/tiles/water02.png").unwrap();
        self.tiles.push(TileData {
            image: Some(water_tile_image_02),
            is_collidable: true,
        });
        let water_tile_image_03 = graphics::Image::from_path(ctx, "/tiles/water03.png").unwrap();
        self.tiles.push(TileData {
            image: Some(water_tile_image_03),
            is_collidable: true,
        });
        let water_tile_image_04 = graphics::Image::from_path(ctx, "/tiles/water04.png").unwrap();
        self.tiles.push(TileData {
            image: Some(water_tile_image_04),
            is_collidable: true,
        });
        let water_tile_image_05 = graphics::Image::from_path(ctx, "/tiles/water05.png").unwrap();
        self.tiles.push(TileData {
            image: Some(water_tile_image_05),
            is_collidable: true,
        });
        let water_tile_image_06 = graphics::Image::from_path(ctx, "/tiles/water06.png").unwrap();
        self.tiles.push(TileData {
            image: Some(water_tile_image_06),
            is_collidable: true,
        });
        let water_tile_image_07 = graphics::Image::from_path(ctx, "/tiles/water07.png").unwrap();
        self.tiles.push(TileData {
            image: Some(water_tile_image_07),
            is_collidable: true,
        });
        let water_tile_image_08 = graphics::Image::from_path(ctx, "/tiles/water08.png").unwrap();
        self.tiles.push(TileData {
            image: Some(water_tile_image_08),
            is_collidable: true,
        });
        let water_tile_image_09 = graphics::Image::from_path(ctx, "/tiles/water09.png").unwrap();
        self.tiles.push(TileData {
            image: Some(water_tile_image_09),
            is_collidable: true,
        });
        let water_tile_image_10 = graphics::Image::from_path(ctx, "/tiles/water10.png").unwrap();
        self.tiles.push(TileData {
            image: Some(water_tile_image_10),
            is_collidable: true,
        });
        let water_tile_image_11 = graphics::Image::from_path(ctx, "/tiles/water11.png").unwrap();
        self.tiles.push(TileData {
            image: Some(water_tile_image_11),
            is_collidable: true,
        });
        let water_tile_image_12 = graphics::Image::from_path(ctx, "/tiles/water12.png").unwrap();
        self.tiles.push(TileData {
            image: Some(water_tile_image_12),
            is_collidable: true,
        });
        let water_tile_image_13 = graphics::Image::from_path(ctx, "/tiles/water13.png").unwrap();
        self.tiles.push(TileData {
            image: Some(water_tile_image_13),
            is_collidable: true,
        });
        //ROAD
        let road_tile_image_00 = graphics::Image::from_path(ctx, "/tiles/road00.png").unwrap();
        self.tiles.push(TileData {
            image: Some(road_tile_image_00),
            is_collidable: false,
        });
        let road_tile_image_01 = graphics::Image::from_path(ctx, "/tiles/road01.png").unwrap();
        self.tiles.push(TileData {
            image: Some(road_tile_image_01),
            is_collidable: false,
        });
        let road_tile_image_02 = graphics::Image::from_path(ctx, "/tiles/road02.png").unwrap();
        self.tiles.push(TileData {
            image: Some(road_tile_image_02),
            is_collidable: false,
        });
        let road_tile_image_03 = graphics::Image::from_path(ctx, "/tiles/road03.png").unwrap();
        self.tiles.push(TileData {
            image: Some(road_tile_image_03),
            is_collidable: false,
        });
        let road_tile_image_04 = graphics::Image::from_path(ctx, "/tiles/road04.png").unwrap();
        self.tiles.push(TileData {
            image: Some(road_tile_image_04),
            is_collidable: false,
        });
        let road_tile_image_05 = graphics::Image::from_path(ctx, "/tiles/road05.png").unwrap();
        self.tiles.push(TileData {
            image: Some(road_tile_image_05),
            is_collidable: false,
        });
        let road_tile_image_06 = graphics::Image::from_path(ctx, "/tiles/road06.png").unwrap();
        self.tiles.push(TileData {
            image: Some(road_tile_image_06),
            is_collidable: false,
        });
        let road_tile_image_07 = graphics::Image::from_path(ctx, "/tiles/road07.png").unwrap();
        self.tiles.push(TileData {
            image: Some(road_tile_image_07),
            is_collidable: false,
        });
        let road_tile_image_08 = graphics::Image::from_path(ctx, "/tiles/road08.png").unwrap();
        self.tiles.push(TileData {
            image: Some(road_tile_image_08),
            is_collidable: false,
        });
        let road_tile_image_09 = graphics::Image::from_path(ctx, "/tiles/road09.png").unwrap();
        self.tiles.push(TileData {
            image: Some(road_tile_image_09),
            is_collidable: false,
        });
        let road_tile_image_10 = graphics::Image::from_path(ctx, "/tiles/road10.png").unwrap();
        self.tiles.push(TileData {
            image: Some(road_tile_image_10),
            is_collidable: false,
        });
        let road_tile_image_11 = graphics::Image::from_path(ctx, "/tiles/road11.png").unwrap();
        self.tiles.push(TileData {
            image: Some(road_tile_image_11),
            is_collidable: false,
        });
        let road_tile_image_12 = graphics::Image::from_path(ctx, "/tiles/road12.png").unwrap();
        self.tiles.push(TileData {
            image: Some(road_tile_image_12),
            is_collidable: false,
        });
        //EARTH
        let earth_tile_image = graphics::Image::from_path(ctx, "/tiles/earth.png").unwrap();
        self.tiles.push(TileData {
            image: Some(earth_tile_image),
            is_collidable: false,
        });
        //WALL
        let wall_tile_image = graphics::Image::from_path(ctx, "/tiles/wall.png").unwrap();
        self.tiles.push(TileData {
            image: Some(wall_tile_image),
            is_collidable: true,
        });
        //TREE
        let tree_tile_image = graphics::Image::from_path(ctx, "/tiles/tree.png").unwrap();
        self.tiles.push(TileData {
            image: Some(tree_tile_image),
            is_collidable: true,
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

        while col < MAX_WORLD_COL && row < MAX_WORLD_ROW {
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

    pub fn draw(&self, ctx: &Context, canvas: &mut Canvas, player: &Player) {
        let mut world_col: u32 = 0;
        let mut world_row: u32 = 0;

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

        while world_col < MAX_WORLD_COL && world_row < MAX_WORLD_ROW {
            let tileNum = self.map_tile_num[world_row as usize][world_col as usize];

            let world_x = world_col as i32 * TILE_SIZE as i32;
            let world_y = world_row as i32 * TILE_SIZE as i32;
            let screen_x = world_x - player.entity.world_x + player.screen_x as i32;
            let screen_y = world_y - player.entity.world_y + player.screen_y as i32;

            if world_x + (TILE_SIZE as i32) > player.entity.world_x - player.screen_x as i32
                && world_x - (TILE_SIZE as i32) < player.entity.world_x + player.screen_x as i32
                && world_y + (TILE_SIZE as i32) > player.entity.world_y - player.screen_y as i32
                && world_y - (TILE_SIZE as i32) < player.entity.world_y + player.screen_y as i32
            {
                match instance_arrays.get_mut(tileNum as usize) {
                    Some(instance_array) => instance_array.push(
                        graphics::DrawParam::new()
                            .dest(Vec2::new(screen_x as f32, screen_y as f32))
                            .scale(Vec2::new(SCALE as f32, SCALE as f32)),
                    ),
                    None => {
                        info!("{} tile instance array not found", tileNum);
                        todo!()
                    }
                }
            }

            world_col += 1;

            if world_col == MAX_WORLD_COL {
                world_col = 0;
                world_row += 1;
            }
        }
        instance_arrays
            .iter()
            .for_each(|instance_array| canvas.draw(instance_array, graphics::DrawParam::new()));
    }
}
