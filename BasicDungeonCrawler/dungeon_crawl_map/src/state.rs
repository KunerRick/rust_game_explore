use crate::prelude::*;

pub struct State {
    ecs: World,
    resources: Resources,
    systems: Schedule,
}

impl State {
    pub fn new() -> Self {
        let mut ecs = World::default();
        let mut resources = Resources::default();
        // 生成地图
        let mut rng = RandomNumberGenerator::new();
        let map_builder = MapBuilder::new(&mut rng);
        // 将地图插入资源
        resources.insert(map_builder.map);
        // 将相机插入资源
        resources.insert(Camera::new(map_builder.player_start));
        // 生成一个玩家角色
        spawn_player(&mut ecs, map_builder.player_start);
        // 生成怪物,跳过第一个房间
        map_builder
            .rooms
            .iter()
            .skip(1)
            .map(|r| r.center())
            .for_each(|pos| spawn_monster(&mut ecs, &mut rng, pos));

        Self {
            ecs,
            resources,
            systems: build_scheduler(),
        }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(0);
        ctx.cls();
        ctx.set_active_console(1);
        ctx.cls();
        // 每次tick都要插入？
        self.resources.insert(ctx.key);
        // 执行？
        self.systems.execute(&mut self.ecs, &mut self.resources);
        // 批量绘制
        render_draw_buffer(ctx).expect("render error");
    }
}
