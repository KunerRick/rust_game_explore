use crate::prelude::*;

const NUM_TILES: usize = (SCREEN_WIDTH * SCREEN_HEIGHT) as usize;

#[derive(Copy, Clone, PartialEq)]
pub enum TileType {
    Wall,
    Floor,
}

/// 行优先存储
///
/// 0 1 2 3
///
/// 4 5 6 7
pub struct Map {
    pub tiles: Vec<TileType>,
}

impl Map {
    pub fn new() -> Self {
        Map {
            tiles: vec![TileType::Floor; NUM_TILES],
        }
    }

    pub fn in_bounds(&self, point: Point) -> bool {
        point.x >= 0 && point.x < SCREEN_WIDTH && point.y >= 0 && point.y < SCREEN_HEIGHT
    }

    pub fn can_enter_tile(&self, point: Point) -> bool {
        self.in_bounds(point) && self.tiles[map_idx(point.x, point.y)] == TileType::Floor
    }

    pub fn try_index(&self, point: Point) -> Option<usize> {
        if !self.in_bounds(point) {
            None
        } else {
            Some(map_idx(point.x, point.y))
        }
    }
}

impl SceneComp for Map {
    fn render(&self, ctx: &mut BTerm, camera: &Camera) {
        // 激活地图层
        ctx.set_active_console(0);
        // 遍历相机范围内所有的点，判断是否还在地图上；
        for y in camera.top_y..camera.bottom_y {
            for x in camera.left_x..camera.right_x {
                // 这里x,y 是相对于地图的坐标点
                if self.in_bounds(Point::new(x, y)) {
                    let idx = map_idx(x, y);
                    // 获取id
                    match self.tiles[idx] {
                        TileType::Floor => {
                            // 这里x,y是世界坐标系
                            // 但视图绘制是从相机边界开始的，要转换到相机的坐标系上
                            // 如： x:[51， 91] ，那么实际绘制得从0开始，即
                            //  x: [0, 40], 那其实就是相对于左边界求差值。
                            ctx.set(
                                x - camera.left_x,
                                y - camera.top_y,
                                RGB::from_u8(100, 150, 130),
                                BLACK,
                                to_cp437('.'),
                            )
                        }
                        TileType::Wall => ctx.set(
                            x - camera.left_x,
                            y - camera.top_y,
                            RGB::from_u8(151, 72, 0),
                            BLACK,
                            to_cp437('#'),
                        ),
                    }
                }
            }
        }
    }
}

/**
 * 左上角为原点，y 向下，x向右
 */
pub fn map_idx(x: i32, y: i32) -> usize {
    (y * SCREEN_WIDTH + x) as usize
}
