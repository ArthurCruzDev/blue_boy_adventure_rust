use ggez::{
    graphics::{self, Rect},
    Context,
};
use log::info;
use rand::Rng;

use crate::entities::entity::{Direction, EntityData, GameEntity};

pub struct NPCOldMan {
    pub screen_x: u32,
    pub screen_y: u32,
    pub entity_data: EntityData,
}

impl NPCOldMan {
    pub fn new(ctx: &mut Context) -> Self {
        let mut npc_old_man = NPCOldMan {
            screen_x: 0,
            screen_y: 0,
            entity_data: EntityData {
                direction: Direction::Down,
                speed: 1,
                solid_area_default_x: 0,
                solid_area_default_y: 24,
                solid_area: Rect::new(0.0, 24.0, 48.0, 24.0),
                ..Default::default()
            },
        };
        npc_old_man.get_npcoldman_images(ctx);
        npc_old_man.set_dialogue();
        npc_old_man
    }
    fn get_npcoldman_images(&mut self, ctx: &mut Context) {
        info!("Loading NPC Old Man images...");
        info!("Loading NPC Old Man up_1 image");
        let up1 = graphics::Image::from_path(ctx, "/npc/oldman_up_1.png").unwrap();
        self.entity_data.up_1 = Some(up1);
        info!("Loading NPC Old Man up_2 image");
        let up2 = graphics::Image::from_path(ctx, "/npc/oldman_up_2.png").unwrap();
        self.entity_data.up_2 = Some(up2);
        info!("Loading NPC Old Man down_1 image");
        let down1 = graphics::Image::from_path(ctx, "/npc/oldman_down_1.png").unwrap();
        self.entity_data.down_1 = Some(down1);
        info!("Loading NPC Old Man down_2 image");
        let down2 = graphics::Image::from_path(ctx, "/npc/oldman_down_2.png").unwrap();
        self.entity_data.down_2 = Some(down2);
        info!("Loading NPC Old Man left_1 image");
        let left1 = graphics::Image::from_path(ctx, "/npc/oldman_left_1.png").unwrap();
        self.entity_data.left_1 = Some(left1);
        info!("Loading NPC Old Man left_2 image");
        let left2 = graphics::Image::from_path(ctx, "/npc/oldman_left_2.png").unwrap();
        self.entity_data.left_2 = Some(left2);
        info!("Loading NPC Old Man right_1 image");
        let right1 = graphics::Image::from_path(ctx, "/npc/oldman_right_1.png").unwrap();
        self.entity_data.right_1 = Some(right1);
        info!("Loading NPC Old Man right_2 image");
        let right2 = graphics::Image::from_path(ctx, "/npc/oldman_right_2.png").unwrap();
        self.entity_data.right_2 = Some(right2);
        info!("Finished loading NPC Old Man images...")
    }

    fn set_dialogue(&mut self) {
        self.entity_data.dialogues = vec![
            String::from("Hello, lad."),
            String::from("So you've como to this island to find the treasure?"),
            String::from(
                "I used to be a great wizard but now... I'm a bit too old for taking adventure.",
            ),
            String::from("Well, good luck on you."),
        ];
    }
}

impl GameEntity for NPCOldMan {
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
                self.entity_data_mut().direction = crate::entities::entity::Direction::Up;
            } else if random_number <= 50 {
                self.entity_data_mut().direction = crate::entities::entity::Direction::Down;
            } else if random_number <= 75 {
                self.entity_data_mut().direction = crate::entities::entity::Direction::Left;
            } else {
                self.entity_data_mut().direction = crate::entities::entity::Direction::Right;
            }
            self.entity_data_mut().action_lock_counter = 0;
        }
    }
}
