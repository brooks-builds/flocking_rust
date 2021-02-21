use flocking_rust::FlockingRustState;
use ggez::conf::WindowMode;
use ggez::{event, ContextBuilder, GameResult};

fn main() -> GameResult {
    let window_mode = WindowMode::default().dimensions(1000.0, 1000.0);
    let (mut context, mut game_loop) = ContextBuilder::new("flocking_rust", "Brooks Patton")
        .window_mode(window_mode)
        .build()?;
    let mut game_state = FlockingRustState::new(&mut context)?;

    event::run(&mut context, &mut game_loop, &mut game_state)
}
