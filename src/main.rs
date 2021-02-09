use flocking_rust::FlockingRustState;
use ggez::{event, ContextBuilder, GameResult};

fn main() -> GameResult {
    let (mut context, game_loop) = ContextBuilder::new("flocking_rust", "Brooks Patton").build()?;
    let game_state = FlockingRustState::new(&mut context)?;

    event::run(context, game_loop, game_state)
}
