use bracket_lib::{
    color::{BLACK, CYAN, ColorPair, RED, RGB, WHITE, YELLOW},
    prelude::{BTerm, DrawBatch, GameState, Point, Rect, VirtualKeyCode, render_draw_buffer},
};

use crate::{
    constants::{FRAME_DURATION, SCREEN_HEIGHT, SCREEN_WIDTH},
    obstacle::Obstacle,
    player::Player,
};

enum GameMode {
    Menu,
    Playing,
    End,
}

pub(crate) struct State {
    mode: GameMode,
    player: Player,
    // 运行了多少帧
    frame_time: f32,
    obstacle: Obstacle,
    score: i32,
}

impl State {
    pub(crate) fn new() -> State {
        State {
            mode: GameMode::Menu,
            player: Player::new(5, 25),
            frame_time: 0.0,
            obstacle: Obstacle::new(SCREEN_WIDTH, 0),
            score: 0,
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

    fn render_bg(&mut self, ctx: &mut BTerm) {
        // 帧率
        let mut draw_batch = DrawBatch::new();

        // Show frame rate
        draw_batch.target(0);
        draw_batch.cls_color(RGB::from_u8(25, 161, 232));
        draw_batch.draw_double_box(
            Rect::with_size(39, 0, 20, 3),
            ColorPair::new(RGB::named(WHITE), RGB::named(BLACK)),
        );
        draw_batch.print_color(
            Point::new(40, 1),
            &format!("FPS: {}", ctx.fps),
            ColorPair::new(RGB::named(YELLOW), RGB::named(BLACK)),
        );
        draw_batch.print_color(
            Point::new(40, 2),
            &format!("Frame Time: {} ms", ctx.frame_time_ms),
            ColorPair::new(RGB::named(CYAN), RGB::named(BLACK)),
        );
        // Submission
        draw_batch.submit(0).expect("Batch error");
        render_draw_buffer(ctx).expect("Render error");
    }

    fn play(&mut self, ctx: &mut BTerm) {
        self.render_bg(ctx);
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
        // 渲染玩家
        self.player.render(ctx);
        // 渲染障碍物
        self.obstacle.render(ctx, self.player.x);

        // 得分逻辑
        if self.player.x > self.obstacle.x {
            self.score += 1;
            self.obstacle = Obstacle::new(self.player.x + SCREEN_WIDTH, self.score);
        }

        // 失败逻辑
        if self.player.y > SCREEN_HEIGHT || self.obstacle.hit_obstacle(&self.player) {
            self.mode = GameMode::End;
        }

        ctx.print(0, 0, "Press SPACE to flap");
        ctx.print(0, 1, format!("Score {}", self.score));
    }
    fn dead(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print_centered(5, "Game Over!");
        ctx.print_centered(7, format!("You earned {} points", self.score));
        ctx.print_centered(10, " (P) Play Again");
        ctx.print_centered(11, "(Q) Quit Game");

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
        self.mode = GameMode::Playing;
        self.player.reset();
        self.frame_time = 0.0;
        self.obstacle = Obstacle::new(SCREEN_WIDTH, 0);
        self.score = 0;
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
