mod component_names;
mod mesh;
mod resource_names;
mod systems;

use bbecs::components::point::Point;
use bbecs::components::Component;
use bbecs::resources::resource::Resource;
use bbecs::world::World;
use component_names::ComponentNames;
use ggez::conf::WindowMode;
use ggez::graphics::Color;
use ggez::timer;
use ggez::{
    event::EventHandler,
    graphics::{self},
    Context, GameResult,
};
use mesh::{create_boid_mesh, create_clear_mesh};
use rand::random;
use resource_names::ResourceNames;
use systems::alignment::alignment_system;
use systems::attraction::attraction_system;
use systems::avoidance::avoidance_system;
use systems::clear_screen::clear_screen_system;
use systems::draw_birds::draw_birds_system;
use systems::handle_arena_edges::handle_arena_edges_system;
use systems::update_boid_color::update_boid_color_system;
use systems::update_locations::update_locations_system;
use systems::update_rotations::update_rotations_system;

type WorldWrapper = World<ComponentNames, ResourceNames>;

pub struct FlockingRustState {
    world: WorldWrapper,
}

impl FlockingRustState {
    pub fn new(context: &mut Context) -> GameResult<Self> {
        let background_color = graphics::BLACK;
        let mut world = World::new();
        let boid_mesh = create_boid_mesh(context, 25.0)?;
        let sight_range = 50.0;

        world.add_resource(
            ResourceNames::BackgroundColor,
            Resource::Color(background_color),
        );
        world.add_resource(ResourceNames::BirdMesh, Resource::Mesh(boid_mesh));
        let arena_size = graphics::drawable_size(context);
        world.add_resource(
            ResourceNames::ArenaSize,
            Resource::Point(Point::new(arena_size.0, arena_size.1)),
        );
        world.add_resource(ResourceNames::UpdateFps, Resource::U32(60));
        world.add_resource(ResourceNames::SightRange, Resource::F32(sight_range));
        world.add_resource(
            ResourceNames::AvoidRange,
            Resource::F32(sight_range * 0.625),
        );
        world.add_resource(ResourceNames::TurningSpeed, Resource::F32(0.5));
        world.add_resource(ResourceNames::AttractionTurningSpeed, Resource::F32(0.1));
        world.add_resource(
            ResourceNames::BoidColor,
            Resource::Color(Color::new(0.0, 0.0, 0.0, 1.0)),
        );
        world.add_resource(ResourceNames::ColorChangeRate, Resource::F32(0.01));
        world.add_resource(ResourceNames::ColorChangeSpeed, Resource::Usize(5));
        world.add_resource(
            ResourceNames::ClearScreenMesh,
            Resource::Mesh(create_clear_mesh(context)?),
        );

        // Spawn the birds
        for _ in 0..100 {
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
                )
                .with_component(ComponentNames::Rotation, Component::F32(0.0));
        }

        Ok(Self { world })
    }
}

impl EventHandler for FlockingRustState {
    fn update(&mut self, context: &mut ggez::Context) -> GameResult {
        let borrowed_update_fps = self.world.get_resource(&ResourceNames::UpdateFps).borrow();
        let update_fps = borrowed_update_fps.cast_u32();
        while timer::check_update_time(context, update_fps) {
            update_boid_color_system(&self.world, timer::ticks(context));
            handle_arena_edges_system(&self.world);
            avoidance_system(&self.world);
            alignment_system(&self.world);
            attraction_system(&self.world);
            update_locations_system(&self.world);
            update_rotations_system(&self.world);
        }
        Ok(())
    }

    fn draw(&mut self, context: &mut ggez::Context) -> GameResult {
        // let background_color = {
        //     let resource = self
        //         .world
        //         .get_resource(&ResourceNames::BackgroundColor)
        //         .borrow();
        //     *resource.cast_color()
        // };
        // graphics::clear(context, background_color);
        clear_screen_system(&self.world, context)?;
        draw_birds_system(context, &self.world)?;
        // visualize_ranges_system(&self.world, context)?;
        graphics::present(context)
    }

    fn resize_event(&mut self, context: &mut Context, width: f32, height: f32) {
        // graphics::set_drawable_size(context, width, height).unwrap();
    }
}
