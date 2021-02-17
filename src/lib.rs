mod systems;

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
    world: Rc<RefCell<World>>,
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

        world.insert_resource("target_fps", ResourcesData::U32(60));
        world.insert_resource("bird_mesh", ResourcesData::GgezMesh(bird_mesh));

        Ok(Self {
            background_color,
            world: Rc::new(RefCell::new(world)),
        })
    }
}

impl EventHandler for FlockingRustState {
    fn update(&mut self, context: &mut ggez::Context) -> GameResult {
        let cloned_world = self.world.clone();
        let borrowed_world = cloned_world.borrow();
        let target_fps = borrowed_world
            .get_resource("target_fps")
            .unwrap_or(&ResourcesData::U32(5))
            .extract_u32()
            .unwrap_or(1);

        while timer::check_update_time(context, target_fps) {
            update_locations_system(self.world.clone());
        }
        Ok(())
    }

    fn draw(&mut self, context: &mut ggez::Context) -> GameResult {
        graphics::clear(context, self.background_color);
        draw_birds_system(context, self.world.clone())?;
        graphics::present(context)
    }
}
