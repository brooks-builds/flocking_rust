mod components;

use std::fmt::Debug;

use bbecs::world::World;
use components::Location;
use ggez::{
    event::EventHandler,
    graphics::{self, DrawMode, DrawParam},
    Context, GameResult,
};
use graphics::{Color, Mesh, MeshBuilder};

pub struct FlockingRustState {
    background_color: Color,
    world: World,
    bird_mesh: Mesh,
}

impl FlockingRustState {
    pub fn new(context: &mut Context) -> GameResult<Self> {
        let background_color = graphics::BLACK;
        let mut world = World::new();
        let bird_mesh = MeshBuilder::new()
            .circle(DrawMode::fill(), [0.0, 0.0], 5.0, 0.1, graphics::WHITE)
            .build(context)?;

        let location = Location::new(50.9, 50.0);
        world.insert_entity(vec![location]);

        Ok(Self {
            background_color,
            world,
            bird_mesh,
        })
    }
}

impl EventHandler for FlockingRustState {
    fn update(&mut self, _ctx: &mut ggez::Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, context: &mut ggez::Context) -> GameResult {
        graphics::clear(context, self.background_color);
        graphics::draw(context, &self.bird_mesh, DrawParam::default())?;
        graphics::present(context)
    }
}
