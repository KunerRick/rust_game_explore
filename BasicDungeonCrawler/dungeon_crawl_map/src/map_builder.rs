use crate::prelude::*;

pub struct MapBuilder {
    pub map: Map,
    pub rooms: Vec<Rect>,
    pub player_start: Point,
}

impl MapBuilder {
    pub fn new(rng: &mut RandomNumberGenerator) -> Self {
        let mut mb = MapBuilder {
            map: Map::new(),
            rooms: Vec::new(),
            player_start: Point::zero(),
        };
        mb.fill(TileType::Wall);
        mb.build_random(rng);
        mb.build_corridors(rng);
        mb.player_start = mb.rooms[0].center();
        mb
    }

    fn fill(&mut self, tile: TileType) {
        self.map.tiles.iter_mut().for_each(|t| *t = tile);
    }
    fn build_random(&mut self, rng: &mut RandomNumberGenerator) {
        while self.rooms.len() < NUM_ROOMS {
            // 这里的10 怎么得到的？rect size?
            let room = Rect::with_size(
                rng.range(1, SCREEN_WIDTH - 10),
                rng.range(1, SCREEN_HEIGHT - 10),
                rng.range(2, 10),
                rng.range(2, 10),
            );
            let mut overlap = false;
            for r in self.rooms.iter() {
                if r.intersect(&room) {
                    overlap = true;
                    break;
                }
            }

            if !overlap {
                // 这是在遍历react中的每个点
                room.for_each(|p| {
                    // 保证房间的墙还在
                    if p.x > 0 && p.x < SCREEN_WIDTH - 1 && p.y > 0 && p.y < SCREEN_HEIGHT - 1 {
                        let idx = map_idx(p.x, p.y);
                        self.map.tiles[idx] = TileType::Floor;
                    }
                });
                self.rooms.push(room);
            }
        }
    }

    /// 开凿两点间的走廊的垂直部分
    fn apply_vertical_tunnel(&mut self, y1: i32, y2: i32, x: i32) {
        use std::cmp::{max, min};

        for y in min(y1, y2)..=max(y1, y2) {
            if let Some(idx) = self.map.try_index(Point::new(x, y)) {
                self.map.tiles[idx] = TileType::Floor;
            }
        }
    }

    /// 开凿两点间的走廊的水平部分
    fn apply_horizontal_tunnel(&mut self, x1: i32, x2: i32, y: i32) {
        use std::cmp::{max, min};
        for x in min(x1, x2)..=max(x1, x2) {
            if let Some(idx) = self.map.try_index(Point::new(x, y)) {
                self.map.tiles[idx] = TileType::Floor;
            }
        }
    }
    // 构建走廊
    fn build_corridors(&mut self, rng: &mut RandomNumberGenerator) {
        let mut rooms = self.rooms.clone();
        // 按中心点进行从左到右排序，来保证是相邻两个房间在挖走廊
        rooms.sort_by(|a, b| a.center().x.cmp(&b.center().x));

        for (i, room) in rooms.iter().enumerate().skip(1) {
            let prev = rooms[i - 1].center();
            let cur = room.center();
            // 决定先横着挖还是先竖着挖
            if rng.range(0, 2) == 1 {
                self.apply_horizontal_tunnel(prev.x, cur.x, prev.y);
                self.apply_vertical_tunnel(prev.y, cur.y, cur.x);
            } else {
                self.apply_vertical_tunnel(prev.y, cur.y, cur.x);
                self.apply_horizontal_tunnel(prev.x, cur.x, prev.y);
            }
        }
    }
}
