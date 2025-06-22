use bracket_lib::{
    color::RGB,
    prelude::{BTerm, to_cp437},
    random::RandomNumberGenerator,
};

use crate::{constants::SCREEN_HEIGHT, player::Player};

pub(crate) struct Obstacle {
    pub(crate) x: i32,
    gap_y: i32,
    size: i32,
}

impl Obstacle {
    pub(crate) fn new(x: i32, score: i32) -> Self {
        let mut random = RandomNumberGenerator::new();
        println!("{score} {}", i32::max(2, 20 - score));
        Obstacle {
            x,
            gap_y: random.range(10, 40),   // 缺口中心位置
            size: i32::max(2, 20 - score), // 缝隙口不小于2
        }
    }

    pub(crate) fn render(&mut self, ctx: &mut BTerm, player_x: i32) {
        let screen_x = self.x - player_x;
        let half_size = self.size / 2;

        // 障碍物上半部分
        for y in 0..self.gap_y - half_size {
            ctx.set(
                screen_x,
                y,
                RGB::from_u8(206, 77, 8),
                RGB::from_u8(255, 190, 181),
                to_cp437('|'),
            );
        }
        // 障碍物下半部分
        for y in self.gap_y + half_size..SCREEN_HEIGHT {
            ctx.set(
                screen_x,
                y,
                RGB::from_u8(206, 77, 8),
                RGB::from_u8(255, 190, 181),
                to_cp437('|'),
            );
        }
    }

    pub(crate) fn hit_obstacle(&self, player: &Player) -> bool {
        let half_size = self.size / 2;
        let eq_x = player.x == self.x;
        if !eq_x {
            return false;
        };
        let player_above_gap = player.y < self.gap_y - half_size;
        let player_below_gap = player.y > self.gap_y + half_size;
        player_above_gap || player_below_gap
    }
}
