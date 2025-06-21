use bracket_lib::{
    color::NAVY,
    prelude::{BTerm, GameState, VirtualKeyCode},
};

use crate::player::Player;

enum GameMode {
    Menu,
    Playing,
    End,
}

const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;
// 人为的降低 运动的帧
const FRAME_DURATION: f32 = 60.0;

pub(crate) struct State {
    mode: GameMode,
    player: Player,
    // 运行了多少帧
    frame_time: f32,
}

impl State {
    pub(crate) fn new() -> State {
        State {
            mode: GameMode::Menu,
            player: Player::new(5, 25),
            frame_time: 0.0,
        }
    }

    fn main_menu(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print_centered(5, "Welcome to Flappy Dragon");
        ctx.print_centered(8, "(P) Play Game");
        ctx.print_centered(10, "(Q) Quit Game");

        // 监听键盘事件
        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::P => self.restart(),
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => (),
            }
        }
    }
    fn play(&mut self, ctx: &mut BTerm) {
        ctx.cls_bg(NAVY);
        self.frame_time += ctx.frame_time_ms;
        // 为游戏减速？
        if self.frame_time > FRAME_DURATION {
            self.frame_time = 0.0;
            self.player.gravity_and_move();
        }
        // 提升
        if let Some(VirtualKeyCode::Space) = ctx.key {
            self.player.flap();
        }
        // render
        self.player.render(ctx);
        ctx.print(0, 0, "Press SPACE to flap");
        if self.player.y > SCREEN_HEIGHT {
            self.mode = GameMode::End;
        }
    }
    fn dead(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print_centered(5, "Game Over!");
        ctx.print_centered(8, " (P) Play Again");
        ctx.print_centered(10, "(Q) Quit Game");

        // 监听键盘事件
        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::P => self.restart(),
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => (),
            }
        }
    }
    fn restart(&mut self) {
        // TODO:
        self.mode = GameMode::Playing;
        self.player.reset();
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut bracket_lib::prelude::BTerm) {
        match self.mode {
            GameMode::Menu => self.main_menu(ctx),
            GameMode::Playing => self.play(ctx),
            GameMode::End => self.dead(ctx),
        }
    }
}
