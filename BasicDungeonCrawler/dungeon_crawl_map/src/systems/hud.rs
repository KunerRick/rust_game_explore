use crate::prelude::*;

#[system]
#[read_component(Health)]
#[read_component(Player)]
pub fn hud(ecs: &SubWorld) {
    // 玩家血条的查询器
    let mut health_query = <&Health>::query().filter(component::<Player>());
    // 把玩家的血条实体查询出来（一定有玩家，所以直接unwrap）
    let player_health = health_query.iter(ecs).nth(0).unwrap();

    let mut draw_batch = DrawBatch::new();
    draw_batch.target(2);
    draw_batch.print_centered(1, "Explore the Dungeon. Cursor keys to move.");
    // 血条绘制
    draw_batch.bar_horizontal(
        Point::zero(),
        SCREEN_WIDTH * 2, // FIXME: 这里设置的2倍，后续待理解，测试了下似乎1倍才正常
        player_health.current,
        player_health.max,
        ColorPair::new(RED, GREEN),
    );
    // 血量文字
    draw_batch.print_color_centered(
        0,
        format!(
            " Health: {} / {} ",
            player_health.current, player_health.max
        ),
        ColorPair::new(WHITE, RED),
    );

    draw_batch.submit(10000).expect("Batch error");
}
