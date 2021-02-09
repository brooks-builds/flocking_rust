use flocking_rust::FlockingRustState;
use ggez::{event, ContextBuilder, GameResult};

fn main() -> GameResult {
    let (mut context, mut game_loop) =
        ContextBuilder::new("flocking_rust", "Brooks Patton").build()?;
    let mut game_state = FlockingRustState::new(&mut context)?;

    event::run(&mut context, &mut game_loop, &mut game_state)
}
