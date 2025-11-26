mod collisions;
mod end_turn;
mod entity_render;
mod map_render;
mod player_input;
mod random_move;
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

//TODO:调度器拆分

pub fn build_scheduler() -> Schedule {
    Schedule::builder()
        .add_system(player_input::player_input_system())
        .add_system(collisions::collisions_system())
        // 注意这里
        .flush()
        .add_system(map_render::map_render_system())
        .add_system(entity_render::entity_render_system())
        .add_system(random_move::random_move_system())
        .build()
}
