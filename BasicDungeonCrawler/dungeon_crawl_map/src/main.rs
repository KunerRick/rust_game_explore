mod map;
mod prelude;
mod state;

use crate::{prelude::*, state::State};

fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("Dungeon Crawler")
        .with_fps_cap(30.0)
        .build()?;
    main_loop(context, State::new())
}
