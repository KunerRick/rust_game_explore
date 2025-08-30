use crate::prelude::*;

pub struct State {
    ecs: World,
    resources: Resources,
    systems: Schedule,
}

impl State {
    pub fn new() -> Self {
        let mut esc = World::default();
        let mut resources = Resources::default();
        // 生成地图
        let mut rng = RandomNumberGenerator::new();
        let map_builder = MapBuilder::new(&mut rng);
        // 将地图插入资源
        resources.insert(map_builder.map);
        // 将相机插入资源
        resources.insert(Camera::new(map_builder.player_start));
        todo!()
        // Self { ecs, resources, systems: build_scheduler() }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(0);
        ctx.cls();
        ctx.set_active_console(1);
        ctx.cls();
        //TODO: Execute Systems
    }
}
