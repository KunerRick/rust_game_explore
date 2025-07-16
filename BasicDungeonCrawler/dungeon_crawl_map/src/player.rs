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
    LeftUp,
    RightUp,
    LeftDown,
    RightDown,
    Center,
}

fn mov(dir: Dir) -> Point {
    match dir {
        Dir::Left => Point::new(-1, 0),
        Dir::Right => Point::new(1, 0),
        Dir::Up => Point::new(0, -1),
        Dir::Down => Point::new(0, 1),
        Dir::LeftUp => Point::new(-1, -1),
        Dir::RightUp => Point::new(1, -1),
        Dir::LeftDown => Point::new(-1, 1),
        Dir::RightDown => Point::new(1, 1),
        Dir::Center => Point::zero(),
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

    pub fn update_handle(&mut self, ctx: &BTerm, map: &Map) {
        let mut active_gamepad = None;
        while let Some(ev) = self.gilrs.next_event() {
            match ev.event {
                EventType::AxisChanged(Axis::LeftStickX, val, _) => {
                    self.gilrs_axios.0 = val;
                }
                EventType::AxisChanged(Axis::LeftStickY, val, _) => {
                    self.gilrs_axios.1 = -val;
                }
                _ => (),
            }
            active_gamepad = Some(ev);
        }

        let (x, y) = self.gilrs_axios;
        if x.abs() != 0.0 || y.abs() != 0.0 {
            let px = match x {
                n if n > 0.2 => 1,
                n if n < -0.2 => -1,
                _ => 0,
            };
            let py = match y {
                n if n > 0.2 => 1,
                n if n < -0.2 => -1,
                _ => 0,
            };
            self.gilrs_delta = Point::new(px, py);

            println!(" axios {:?} {:?}", self.gilrs_delta, self.gilrs_axios);
        } else if let Some(Event {
            id, event, time, ..
        }) = active_gamepad
        {
            let delta: Option<Point> = match event {
                EventType::ButtonPressed(Button::DPadUp, _) => Some(mov(Dir::Up)),
                EventType::ButtonChanged(Button::DPadUp, val, _) => {
                    if val > 0.1 {
                        Some(mov(Dir::Up))
                    } else {
                        Some(mov(Dir::Center))
                    }
                }
                EventType::ButtonPressed(Button::DPadDown, _) => Some(mov(Dir::Down)),
                EventType::ButtonChanged(Button::DPadDown, val, _) => {
                    if val > 0.1 {
                        Some(mov(Dir::Down))
                    } else {
                        Some(mov(Dir::Center))
                    }
                }
                EventType::ButtonPressed(Button::DPadLeft, _) => Some(mov(Dir::Left)),
                EventType::ButtonChanged(Button::DPadLeft, val, _) => {
                    if val > 0.1 {
                        Some(mov(Dir::Left))
                    } else {
                        Some(mov(Dir::Center))
                    }
                }
                EventType::ButtonPressed(Button::DPadRight, _) => Some(mov(Dir::Right)),
                EventType::ButtonChanged(Button::DPadRight, val, _) => {
                    if val > 0.1 {
                        Some(mov(Dir::Right))
                    } else {
                        Some(mov(Dir::Center))
                    }
                }
                EventType::AxisChanged(Axis::LeftStickX, val, _) => Some(mov(Dir::Center)),
                EventType::AxisChanged(Axis::LeftStickY, val, _) => Some(mov(Dir::Center)),
                EventType::ButtonChanged(_, _, _) => None,
                _ => None,
            };

            if let Some(v) = delta {
                self.gilrs_delta = v
            }

            println!(" {:?} {:?}", self.gilrs_delta, self.gilrs_axios);
        }

        let p = self.position + self.gilrs_delta;
        let Point { x, y } = self.gilrs_delta;
        if x != 0 && y != 0 {
            // 三种情况都尝试一下
            let p = self.position + Point::new(x, y);
            println!("p1 {p:?}");
            if map.can_enter_tile(p) {
                return self.position = p;
            }
            let p = self.position + Point::new(x, 0);
            println!("p2 {p:?}");
            if map.can_enter_tile(p) {
                return self.position = p;
            }
            let p = self.position + Point::new(0, y);
            println!("p3 {p:?}");
            if map.can_enter_tile(p) {
                return self.position = p;
            }
        } else if map.can_enter_tile(p) {
            self.position = p;
        }
    }

    pub fn update(&mut self, ctx: &BTerm, map: &Map) {
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
            }
        }
    }
}
impl SceneComp for Player {
    fn render(&self, ctx: &mut BTerm) {
        ctx.set(
            self.position.x,
            self.position.y,
            WHITE,
            BLACK,
            to_cp437('@'),
        );
    }
}
