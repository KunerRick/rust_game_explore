use crate::{components::wants_to_move::WantsToMove, prelude::*};

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
    commands: &mut CommandBuffer,
    #[resource] key: &Option<VirtualKeyCode>,
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
            let mut players = <(Entity, &Point)>::query().filter(component::<Player>());
            players.iter(ecs).for_each(|(entity, pos)| {
                let destination = *pos + delta;
                // 名义上是发送意图，但实现其实是把一个新的 实体插入到world中，留待movement系统去查询处理。
                commands.push((
                    (),
                    WantsToMove {
                        entity: *entity,
                        destination,
                    },
                ));
            });
        }
        // 只要有用户输入，就转换回合状态,输入回合 -> 用户回合
        *turn_state = TurnState::PlayerTurn;
    }
}
