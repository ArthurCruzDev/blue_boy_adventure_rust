use ggez::{
    graphics::{self, Rect},
    Context,
};
use log::info;
use rand::Rng;

use crate::entities::entity::{Direction, EntityData, EntityType, GameEntity};

pub struct MonGreenSlime {
    pub entity_data: EntityData,
}

impl MonGreenSlime {
    pub fn new(ctx: &mut Context) -> Self {
        let mut mon_green_slime = MonGreenSlime {
            entity_data: EntityData {
                name: "Green Slime".to_string(),
                speed: 1,
                max_life: 4,
                life: 4,
                solid_area: Rect::new(3.0, 18.0, 42.0, 30.0),
                solid_area_default_x: 3,
                solid_area_default_y: 18,
                entity_type: EntityType::MONSTER,
                ..Default::default()
            },
        };

        mon_green_slime.get_mon_green_slime_images(ctx);

        mon_green_slime
    }

    fn get_mon_green_slime_images(&mut self, ctx: &mut Context) {
        info!("Loading Monster Green Slime images...");
        info!("Loading Monster Green Slime up_1 image");
        let up1 = graphics::Image::from_path(ctx, "/monsters/greenslime_down_1.png").unwrap();
        self.entity_data.up_1 = Some(up1);
        info!("Loading Monster Green Slime up_2 image");
        let up2 = graphics::Image::from_path(ctx, "/monsters/greenslime_down_2.png").unwrap();
        self.entity_data.up_2 = Some(up2);
        info!("Loading Monster Green Slime down_1 image");
        let down1 = graphics::Image::from_path(ctx, "/monsters/greenslime_down_1.png").unwrap();
        self.entity_data.down_1 = Some(down1);
        info!("Loading Monster Green Slime down_2 image");
        let down2 = graphics::Image::from_path(ctx, "/monsters/greenslime_down_2.png").unwrap();
        self.entity_data.down_2 = Some(down2);
        info!("Loading Monster Green Slime left_1 image");
        let left1 = graphics::Image::from_path(ctx, "/monsters/greenslime_down_1.png").unwrap();
        self.entity_data.left_1 = Some(left1);
        info!("Loading Monster Green Slime left_2 image");
        let left2 = graphics::Image::from_path(ctx, "/monsters/greenslime_down_2.png").unwrap();
        self.entity_data.left_2 = Some(left2);
        info!("Loading Monster Green Slime right_1 image");
        let right1 = graphics::Image::from_path(ctx, "/monsters/greenslime_down_1.png").unwrap();
        self.entity_data.right_1 = Some(right1);
        info!("Loading Monster Green Slime right_2 image");
        let right2 = graphics::Image::from_path(ctx, "/monsters/greenslime_down_2.png").unwrap();
        self.entity_data.right_2 = Some(right2);
        info!("Finished loading Monster Green Slime images...")
    }
}

impl GameEntity for MonGreenSlime {
    fn entity_data(&self) -> &EntityData {
        &self.entity_data
    }

    fn entity_data_mut(&mut self) -> &mut EntityData {
        &mut self.entity_data
    }

    fn set_action(&mut self) {
        self.entity_data_mut().action_lock_counter += 1;

        if self.entity_data().action_lock_counter == 120 {
            let mut rng = rand::prelude::thread_rng();
            let random_number: u32 = rng.gen_range(1..101);

            if random_number <= 25 {
                self.entity_data_mut().direction = crate::entities::entity::Direction::UP;
            } else if random_number <= 50 {
                self.entity_data_mut().direction = crate::entities::entity::Direction::DOWN;
            } else if random_number <= 75 {
                self.entity_data_mut().direction = crate::entities::entity::Direction::LEFT;
            } else {
                self.entity_data_mut().direction = crate::entities::entity::Direction::RIGHT;
            }
            self.entity_data_mut().action_lock_counter = 0;
        }
    }

    fn speak(
        &mut self,
        game_handlers: &mut crate::GameHandlers,
        player: &crate::entities::player::Player,
    ) {
        game_handlers.ui_handler.current_dialogue = self
            .entity_data()
            .dialogues
            .get(self.entity_data().dialogue_index)
            .unwrap()
            .to_string();
        self.entity_data_mut().dialogue_index += 1;
        if self.entity_data_mut().dialogue_index == self.entity_data().dialogues.len() {
            self.entity_data_mut().dialogue_index = 0;
        }

        match player.entity.direction {
            crate::entities::entity::Direction::UP => {
                self.entity_data_mut().direction = crate::entities::entity::Direction::DOWN
            }
            crate::entities::entity::Direction::DOWN => {
                self.entity_data_mut().direction = crate::entities::entity::Direction::UP
            }
            crate::entities::entity::Direction::LEFT => {
                self.entity_data_mut().direction = crate::entities::entity::Direction::RIGHT
            }
            crate::entities::entity::Direction::RIGHT => {
                self.entity_data_mut().direction = crate::entities::entity::Direction::LEFT
            }
        }
    }

    fn damage_reaction(&mut self, damage_direction: Direction) {
        self.entity_data_mut().action_lock_counter = 0;
        self.entity_data_mut().direction = damage_direction;
    }
}
