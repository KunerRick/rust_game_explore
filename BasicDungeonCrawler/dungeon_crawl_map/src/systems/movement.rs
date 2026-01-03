use crate::{components::wants_to_move::WantsToMove, prelude::*};

// for_each 是一种简写，只运行一条查询的系统可以这么简写，与手动遍历效果一致
#[system(for_each)]
#[read_component(Player)]
pub fn movement(
    entity: &Entity,
    want_move: &WantsToMove,
    #[resource] map: &Map,
    #[resource] camera: &mut Camera,
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer,
) {
    // 判断是否能移动，能移动则进行实体替换
    if map.can_enter_tile(want_move.destination) {
        // 含义： 给 实体 entity 增加 Point 组件，相当于修改实体的位置信息，如果已存在，会替换掉
        commands.add_component(want_move.entity, want_move.destination);
        // 如果是玩家，还需要更新相机信息
        if ecs
            .entry_ref(want_move.entity)
            .unwrap()
            .get_component::<Player>()
            .is_ok()
        {
            camera.on_player_move(want_move.destination);
        }
    }
    // 处理完成后，从commands中移除，防止被再次查询到又进行处理
    commands.remove(*entity);
}
