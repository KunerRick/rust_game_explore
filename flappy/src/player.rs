use bracket_lib::{
    color::{BLACK, YELLOW},
    prelude::{BTerm, to_cp437},
};

pub(crate) struct Player {
    x: i32,
    pub(crate) y: i32,
    init_x: i32,
    init_y: i32,
    velocity: f32,
}

impl Player {
    pub(crate) fn new(x: i32, y: i32) -> Self {
        Player {
            x,
            y,
            init_x: x,
            init_y: y,
            velocity: 0.0,
        }
    }

    pub(crate) fn render(&mut self, ctx: &mut BTerm) {
        ctx.set(0, self.y, YELLOW, BLACK, to_cp437('@'));
    }

    /// 最大速度2.0
    /// 没有达到最大速度时，要累加速度
    pub(crate) fn gravity(&mut self) {
        if self.velocity < 2.0 {
            self.velocity += 0.2;
        }
        // 调整y值
        self.y += self.velocity as i32;
        if self.y < 0 {
            self.y = 0;
        }

        // TODO: 似乎没有最大y的控制？
    }

    /// 水平移动
    pub(crate) fn horizontal_move(&mut self) {
        // 保持移动
        self.x += 1;
    }

    pub(crate) fn gravity_and_move(&mut self) {
        self.gravity();
        self.horizontal_move();
    }

    pub(crate) fn flap(&mut self) {
        // 直接给最小的加速度？
        self.velocity = -2.0;
    }

    pub(crate) fn reset(&mut self) {
        self.x = self.init_x;
        self.y = self.init_y;
        self.velocity = 0.0;
    }
}
