use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Player)]
#[read_component(Enemy)]
pub fn collisions(ecs: &mut SubWorld, commands: &mut CommandBuffer) {
    let mut player_pos = Point::zero();
    let mut players = <&Point>::query().filter(component::<Player>());
    players.iter(ecs).for_each(|v| player_pos = *v);
    // 这里用Entity 是为了后续使用
    let mut enemies = <(Entity, &Point)>::query().filter(component::<Enemy>());
    // 找到和角色重合的，移除掉 
    enemies
        .iter(ecs)
        .filter(|(_, p)| player_pos == **p)
        .for_each(|(e, _)| commands.remove(*e));
}
