use gilrs::{Axis, Button, Event, EventType, Gilrs};

use crate::prelude::*;

pub struct Player {
    pub position: Point,
    // 手柄事件
    gilrs: Gilrs,
    gilrs_delta: Point,
    gilrs_axios: (f32, f32),
}

enum Dir {
    Left,
    Right,
    Up,
    Down,
}

fn mov(dir: Dir) -> Point {
    match dir {
        Dir::Left => Point::new(-1, 0),
        Dir::Right => Point::new(1, 0),
        Dir::Up => Point::new(0, -1),
        Dir::Down => Point::new(0, 1),
    }
}

impl Player {
    pub fn new(point: Point) -> Self {
        Self {
            position: point,
            gilrs: Gilrs::new().unwrap(),
            gilrs_delta: Point::zero(),
            gilrs_axios: (0.0, 0.0),
        }
    }

    fn try_move(&mut self, map: &Map, camera: &mut Camera) {
        // 更新 gilrs_delta
        let px = match self.gilrs_axios.0 {
            n if n > 0.4 => 1,
            n if n < -0.4 => -1,
            _ => 0,
        };
        let py = match self.gilrs_axios.1 {
            n if n > 0.4 => 1,
            n if n < -0.4 => -1,
            _ => 0,
        };
        self.gilrs_delta = Point::new(px, py);

        // 尝试位移
        let Point { x, y } = self.gilrs_delta;
        let p = self.position + Point::new(x, y);
        if map.can_enter_tile(p) {
            self.position = p;
            camera.on_player_move(p);
            return;
        }
        let p = self.position + Point::new(x, 0);
        if map.can_enter_tile(p) {
            self.position = p;
            camera.on_player_move(p);
            return;
        }
        let p = self.position + Point::new(0, y);
        if map.can_enter_tile(p) {
            self.position = p;
            camera.on_player_move(p);
            return;
        }
    }

    pub fn update_handle(&mut self, ctx: &BTerm, map: &Map, camera: &mut Camera) {
        // 目前只用到了方向相关信息，后续完善
        while let Some(ev) = self.gilrs.next_event() {
            // 这里有可能包含了组合键，这里需要将其进一步归类为 方向控制，按键信息
            match ev.event {
                EventType::ButtonChanged(Button::DPadUp, val, _) => {
                    if val > 0.1 {
                        self.gilrs_axios.1 = -1.0;
                    } else {
                        self.gilrs_axios.1 = 0.0;
                    }
                }
                EventType::ButtonChanged(Button::DPadDown, val, _) => {
                    if val > 0.1 {
                        self.gilrs_axios.1 = 1.0;
                    } else {
                        self.gilrs_axios.1 = 0.0;
                    }
                }
                EventType::ButtonChanged(Button::DPadLeft, val, _) => {
                    if val > 0.1 {
                        self.gilrs_axios.0 = -1.0;
                    } else {
                        self.gilrs_axios.0 = 0.0;
                    }
                }
                EventType::ButtonChanged(Button::DPadRight, val, _) => {
                    if val > 0.1 {
                        self.gilrs_axios.0 = 1.0;
                    } else {
                        self.gilrs_axios.0 = 0.0;
                    }
                }
                EventType::AxisChanged(Axis::LeftStickX, val, _) => {
                    self.gilrs_axios.0 = val;
                }
                EventType::AxisChanged(Axis::LeftStickY, val, _) => {
                    self.gilrs_axios.1 = -val;
                }
                _ => (),
            }
        }
        self.try_move(map, camera);
        // TODO 其他按键
    }

    pub fn update(&mut self, ctx: &BTerm, map: &Map, camera: &mut Camera) {
        if let Some(key) = ctx.key {
            let delta = match key {
                VirtualKeyCode::Up | VirtualKeyCode::W => mov(Dir::Up),
                VirtualKeyCode::Right | VirtualKeyCode::D => mov(Dir::Right),
                VirtualKeyCode::Down | VirtualKeyCode::S => mov(Dir::Down),
                VirtualKeyCode::Left | VirtualKeyCode::A => mov(Dir::Left),
                _ => Point::zero(),
            };
            let new_pos = self.position + delta;
            if map.can_enter_tile(new_pos) {
                self.position = new_pos;
                camera.on_player_move(new_pos);
            }
        }
    }
}
impl SceneComp for Player {
    fn render(&self, ctx: &mut BTerm, camera: &Camera) {
        // 更新角色层
        ctx.set_active_console(1);

        // 更新用户信息
        ctx.set(
            self.position.x - camera.left_x,
            self.position.y - camera.top_y,
            WHITE,
            BLACK,
            to_cp437('@'),
        );
    }
}