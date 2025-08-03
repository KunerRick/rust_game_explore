use crate::prelude::*;

#[derive(Debug)]
pub struct Camera {
    pub left_x: i32,
    pub right_x: i32,
    pub top_y: i32,
    pub bottom_y: i32,
}

fn cal_camera_info(pos: Point) -> Camera {
    Camera {
        left_x: pos.x - DISPLAY_WIDTH / 2,
        right_x: pos.x + DISPLAY_WIDTH / 2,
        top_y: pos.y - DISPLAY_HEIGHT / 2,
        bottom_y: pos.y + DISPLAY_HEIGHT / 2,
    }
}

impl Camera {
    pub fn new(init_pos: Point) -> Self {
        cal_camera_info(init_pos)
    }

    fn update_pos(&mut self, pos: Point) {
        *self = cal_camera_info(pos);
    }

    pub fn on_player_move(&mut self, player_pos: Point) {
        self.update_pos(player_pos);
    }
}
