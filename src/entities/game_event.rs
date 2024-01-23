use ggez::{glam::Vec2, graphics::Rect};

pub struct GameEvent {
    pub event_rect: Rect,
    pub event_default_x: i32,
    pub event_default_y: i32,
    pub event_done: bool,
}

impl Default for GameEvent {
    fn default() -> Self {
        GameEvent {
            event_rect: Rect {
                x: 0.0,
                y: 0.0,
                w: 2.0,
                h: 2.0,
            },
            event_default_x: 0,
            event_default_y: 0,
            event_done: false,
        }
    }
}

impl From<Vec2> for GameEvent {
    fn from(value: Vec2) -> Self {
        GameEvent {
            event_rect: Rect {
                x: value.x,
                y: value.y,
                w: 2.0,
                h: 2.0,
            },
            event_default_x: value.x as i32,
            event_default_y: value.y as i32,
            event_done: false,
        }
    }
}

impl GameEvent {}
