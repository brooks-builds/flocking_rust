mod systems;

use bbecs::components::Component;
use bbecs::world::World;
use ggez::{
    event::EventHandler,
    graphics::{self, DrawMode},
    Context, GameResult,
};
use graphics::{Color, Mesh, MeshBuilder};
use systems::draw_birds::draw_birds_system;
use systems::update_locations::update_locations_system;

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

        world
            .spawn_entity()
            .with_component("location", Component::create_vector_2(50.0, 50.0))
            .with_component("velocity", Component::create_vector_2(0.5, 0.5));

        world
            .spawn_entity()
            .with_component("location", Component::create_vector_2(150.0, 150.0))
            .with_component("velocity", Component::create_vector_2(0.5, 0.5));

        Ok(Self {
            background_color,
            world,
            bird_mesh,
        })
    }
}

impl EventHandler for FlockingRustState {
    fn update(&mut self, _ctx: &mut ggez::Context) -> GameResult {
        update_locations_system(&mut self.world);
        Ok(())
    }

    fn draw(&mut self, context: &mut ggez::Context) -> GameResult {
        graphics::clear(context, self.background_color);
        draw_birds_system(context, &self.bird_mesh, &mut self.world)?;
        graphics::present(context)
    }
}
