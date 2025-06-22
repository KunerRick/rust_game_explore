mod constants;
mod obstacle;
mod player;
mod state;

use crate::state::State;
use bracket_lib::prelude::{BError, BTermBuilder, main_loop};

fn main() -> BError {
    let mut context = BTermBuilder::simple80x50()
        .with_fancy_console(80, 50, "terminal8x8.png")
        .with_title("Bracket Terminal - Fancy Consoles")
        .with_vsync(false)
        .build()?;
    main_loop(context, State::new())
}
