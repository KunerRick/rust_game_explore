use crate::prelude::*;

pub struct State {
    map: Map,
    player: Player,
}

impl State {
    pub fn new() -> Self {
        // 生成地图
        let mut rng = RandomNumberGenerator::new();
        let map_builder = MapBuilder::new(&mut rng);

        Self {
            map: map_builder.map,
            player: Player::new(map_builder.player_start),
        }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        // 监听按键
        self.player.update(ctx, &self.map);
        self.player.update_handle(ctx, &self.map);
        self.map.render(ctx);
        self.player.render(ctx);
    }
}
