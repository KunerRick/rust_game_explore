use crate::{components::wants_to_move::WantsToMove, prelude::*};

#[system]
#[write_component(Point)]
#[read_component(MovingRandomly)]
pub fn random_move(ecs: &mut SubWorld, commands: &mut CommandBuffer) {
    let mut movers = <(&Entity, &Point, &MovingRandomly)>::query();
    movers.iter(ecs).for_each(|(entity, pos, _)| {
        let mut rng = RandomNumberGenerator::new();
        // 上 右 下 左
        let destination = match rng.range(0, 4) {
            0 => Point::new(0, -1),
            1 => Point::new(1, 0),
            2 => Point::new(0, 1),
            3 => Point::new(-1, 0),
            _ => Point::new(0, 0),
        } + *pos;

        commands.push((
            (),
            WantsToMove {
                entity: *entity,
                destination,
            },
        ));
    });
}
