mod systems;

use bbecs::components::point::Point;
use bbecs::components::Component;
use bbecs::resources::ResourcesData;
use bbecs::world::World;
use ggez::timer;
use ggez::{
    event::EventHandler,
    graphics::{self, DrawMode},
    Context, GameResult,
};
use graphics::{Color, Mesh, MeshBuilder};
use std::cell::RefCell;
use std::rc::Rc;
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
            .with_component("location", Component::Point(Point::new(50.0, 50.0)))
            .with_component("velocity", Component::Point(Point::new(1.0, 1.0)));

        world
            .spawn_entity()
            .with_component("location", Component::Point(Point::new(75.0, 150.0)))
            .with_component("velocity", Component::Point(Point::new(1.0, 1.0)));

        Ok(Self {
            background_color,
            world,
            bird_mesh,
        })
    }
}

impl EventHandler for FlockingRustState {
    fn update(&mut self, context: &mut ggez::Context) -> GameResult {
        update_locations_system(&self.world);
        Ok(())
    }

    fn draw(&mut self, context: &mut ggez::Context) -> GameResult {
        graphics::clear(context, self.background_color);
        draw_birds_system(context, &self.bird_mesh, &self.world)?;
        graphics::present(context)
    }
}
