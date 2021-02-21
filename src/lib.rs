mod component_names;
mod resource_names;
mod systems;

use bbecs::components::point::Point;
use bbecs::components::Component;
use bbecs::resources::resource::Resource;
use bbecs::world::World;
use component_names::ComponentNames;
use ggez::timer;
use ggez::{
    event::EventHandler,
    graphics::{self, DrawMode},
    Context, GameResult,
};
use graphics::MeshBuilder;
use rand::random;
use resource_names::ResourceNames;
use systems::avoidance::avoidance_system;
use systems::draw_birds::draw_birds_system;
use systems::handle_arena_edges::handle_arena_edges_system;
use systems::update_locations::update_locations_system;

type WorldWrapper = World<ComponentNames, ResourceNames>;

pub struct FlockingRustState {
    world: WorldWrapper,
}

impl FlockingRustState {
    pub fn new(context: &mut Context) -> GameResult<Self> {
        let background_color = graphics::BLACK;
        let mut world = World::new();
        let bird_mesh = MeshBuilder::new()
            .circle(DrawMode::fill(), [0.0, 0.0], 5.0, 0.1, graphics::WHITE)
            .build(context)?;

        world.add_resource(
            ResourceNames::BackgroundColor,
            Resource::Color(background_color),
        );
        world.add_resource(ResourceNames::BirdMesh, Resource::Mesh(bird_mesh));
        let arena_size = graphics::drawable_size(context);
        world.add_resource(
            ResourceNames::ArenaSize,
            Resource::Point(Point::new(arena_size.0, arena_size.1)),
        );
        world.add_resource(ResourceNames::UpdateFps, Resource::U32(60));

        // Spawn the birds
        for _ in 0..25 {
            world
                .spawn_entity()
                .with_component(
                    ComponentNames::Location,
                    Component::Point(Point::new(
                        random::<f32>() * arena_size.0,
                        random::<f32>() * arena_size.1,
                    )),
                )
                .with_component(
                    ComponentNames::Velocity,
                    Component::Point(Point::new(
                        (rand::random::<f32>() - 0.5) * 5.0,
                        (rand::random::<f32>() - 0.5) * 5.0,
                    )),
                )
                .with_component(
                    ComponentNames::Acceleration,
                    Component::Point(Point::new(0.0, 0.0)),
                );
        }

        Ok(Self { world })
    }
}

impl EventHandler for FlockingRustState {
    fn update(&mut self, context: &mut ggez::Context) -> GameResult {
        let borrowed_update_fps = self.world.get_resource(&ResourceNames::UpdateFps).borrow();
        let update_fps = borrowed_update_fps.cast_u32();
        while timer::check_update_time(context, update_fps) {
            handle_arena_edges_system(&self.world);
            avoidance_system(&self.world);
            update_locations_system(&self.world);
        }
        Ok(())
    }

    fn draw(&mut self, context: &mut ggez::Context) -> GameResult {
        let background_color = {
            let resource = self
                .world
                .get_resource(&ResourceNames::BackgroundColor)
                .borrow();
            *resource.cast_color()
        };
        graphics::clear(context, background_color);
        draw_birds_system(context, &self.world)?;
        graphics::present(context)
    }
}
