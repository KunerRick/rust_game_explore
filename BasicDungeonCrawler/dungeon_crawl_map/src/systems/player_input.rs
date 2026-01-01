use crate::prelude::*;

trait Test {
    fn hello();
}

impl<'a, T> Test for T {
    fn hello() {
        todo!()
    }
}

fn test() {
    <&String>::hello();
}

#[system]
#[write_component(Point)]
#[read_component(Player)]
pub fn player_input(
    ecs: &mut SubWorld,
    #[resource] map: &Map,
    #[resource] key: &Option<VirtualKeyCode>,
    #[resource] camera: &mut Camera,
    #[resource] turn_state: &mut TurnState,
) {
    if let Some(key) = key {
        let delta = match key {
            VirtualKeyCode::Left | VirtualKeyCode::A => Point::new(-1, 0),
            VirtualKeyCode::Up | VirtualKeyCode::W => Point::new(0, -1),
            VirtualKeyCode::Right | VirtualKeyCode::D => Point::new(1, 0),
            VirtualKeyCode::Down | VirtualKeyCode::S => Point::new(0, 1),
            _ => Point::new(0, 0),
        };
        if delta.x != 0 || delta.y != 0 {
            let mut players = <&mut Point>::query().filter(component::<Player>());
            players.iter_mut(ecs).for_each(|pos| {
                let dest = *pos + delta;
                if map.can_enter_tile(dest) {
                    *pos = dest;
                    camera.on_player_move(dest);
                }
            });
        }
        // 只要有用户输入，就转换回合状态,输入回合 -> 用户回合
        *turn_state = TurnState::PlayerTurn;
    }
}
