use bracket_lib::prelude::{BError, BTermBuilder, GameState, main_loop};

struct State {}

impl GameState for State {
    fn tick(&mut self, ctx: &mut bracket_lib::prelude::BTerm) {
        ctx.cls();
        ctx.print(1, 1, "Hello,Bracket Terminal!");
    }
}

fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("Flappy Dragon")
        .build()?;
    main_loop(context, State {})
}
