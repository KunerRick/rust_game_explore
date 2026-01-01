mod collisions;
mod end_turn;
mod entity_render;
mod map_render;
mod player_input;
mod random_move;
mod movement;
use crate::prelude::*;

pub fn build_input_scheduler() -> Schedule {
    // 输入调度，用户输入、渲染地图、渲染实体
    Schedule::builder()
        .add_system(player_input::player_input_system())
        .flush()
        .add_system(map_render::map_render_system())
        .add_system(entity_render::entity_render_system())
        .build()
}

pub fn build_player_scheduler()-> Schedule {
    // 碰撞系统判定、渲染地图、渲染实体、回合轮转
    Schedule::builder()
    .add_system(movement::movement_system())
    .flush()
    .add_system(collisions::collisions_system())
    .flush()
    .add_system(map_render::map_render_system())
    .add_system(entity_render::entity_render_system())
    .add_system(end_turn::end_turn_system())
    .build()
}

pub fn build_monster_scheduler()-> Schedule {
    // 随机移动、碰撞检测、地图、实体、回合轮转
    Schedule::builder()
    .add_system(random_move::random_move_system())
    .flush()
    .add_system(movement::movement_system())
    .flush()
    .add_system(collisions::collisions_system())
    .flush()
    .add_system(map_render::map_render_system())
    .add_system(entity_render::entity_render_system())
    .add_system(end_turn::end_turn_system())
    .build()
}