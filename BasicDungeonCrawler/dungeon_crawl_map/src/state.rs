use crate::prelude::*;

pub struct State {
    ecs: World,
    resources: Resources,
    input_systems: Schedule,
    player_systems: Schedule,
    monster_systems: Schedule
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
        // 将回合的初始态插入
        resources.insert(TurnState::AwaitingInput);
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
            input_systems: build_input_scheduler(),
            player_systems: build_player_scheduler(),
            monster_systems: build_monster_scheduler()
        }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(0);
        ctx.cls();
        ctx.set_active_console(1);
        ctx.cls();
        ctx.set_active_console(2);
        ctx.cls();
        // 每次tick都要插入，是的，每次都要传入当前帧用户的按键输入，以便有系统进行处理
        self.resources.insert(ctx.key);
        // 根据回合状态使用不同的调度器
        let current_state = self.resources.get::<TurnState>().unwrap().clone();
        match current_state {
            TurnState::AwaitingInput=> self.input_systems.execute(&mut self.ecs, &mut self.resources),
            TurnState::PlayerTurn=>self.player_systems.execute(&mut self.ecs, &mut self.resources),
            TurnState::MonsterTurn=>self.monster_systems.execute(&mut self.ecs, &mut self.resources),
        }
        // 批量绘制
        render_draw_buffer(ctx).expect("render error");
    }
}
