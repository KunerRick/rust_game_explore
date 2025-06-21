

mod state;
mod player;

use crate::state::State;
use bracket_lib::prelude::{BError, BTermBuilder, main_loop};

fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("Flappy Dragon")
        .build()?;
    main_loop(context, State::new())
}
