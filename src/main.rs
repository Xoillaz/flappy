use bracket_lib::prelude::*;

// All the data between frames is the game's state.
struct State {}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        // 'context' provides functions for interacting with the game display.
        ctx.cls();
        // (1,1) represents the screen-space coordinate you want the text to appear.
        //
        // The function converts text into the appropriate sprites.
        ctx.print(1, 1, "Hello, Bracket Terminal!");
    }
}

fn main() -> BError {
    let context = BTermBuilder::simple80x50().with_title("Flappy").build()?;

    main_loop(context, State {})
}
