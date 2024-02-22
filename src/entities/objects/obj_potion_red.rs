use ggez::{
    graphics::{self, Rect},
    Context,
};

use crate::{
    entities::entity::{EntityData, EntityType, GameEntity},
    utils::{
        game_state_handler::{GameState, GameStateHandler},
        sound_handler::SoundHandler,
        ui::UIHandler,
    },
};

pub struct ObjPotionRed {
    pub entity_data: EntityData,
    pub value: i32,
}

impl ObjPotionRed {
    pub fn new(ctx: &mut Context) -> Self {
        ObjPotionRed {
            entity_data: EntityData {
                down_1: Some(graphics::Image::from_path(ctx, "/objects/potion_red.png").unwrap()),
                name: "Red Potion".to_string(),
                is_collidable: false,
                entity_type: EntityType::CONSUMABLE,
                description: "[Red Potion]\nHeals your life by 5.".to_owned(),
                attack_area: Rect {
                    x: 0.0,
                    y: 0.0,
                    w: 30.0,
                    h: 30.0,
                },
                ..Default::default()
            },
            value: 5,
        }
    }
    pub fn new_positioned(ctx: &mut Context, world_x: i32, world_y: i32) -> Self {
        let mut axe = ObjPotionRed::new(ctx);
        axe.entity_data.world_x = world_x;
        axe.entity_data.world_y = world_y;
        axe
    }
}

impl GameEntity for ObjPotionRed {
    fn entity_data(&self) -> &EntityData {
        &self.entity_data
    }

    fn entity_data_mut(&mut self) -> &mut EntityData {
        &mut self.entity_data
    }
    fn use_item(
        &mut self,
        ctx: &mut Context,
        entity: &mut EntityData,
        game_state_handler: &mut GameStateHandler,
        ui_handler: &mut UIHandler,
        sound_handler: &mut SoundHandler,
    ) {
        game_state_handler.game_state = GameState::Dialogue;
        ui_handler.current_dialogue = format!(
            "You drink the {}!\nYour life has been recovered by {}.",
            self.entity_data.name, self.value
        );
        entity.life += self.value;
        if entity.life > entity.max_life {
            entity.life = entity.max_life;
        }
        sound_handler.play_sound_effect(ctx, 2);
    }
}
