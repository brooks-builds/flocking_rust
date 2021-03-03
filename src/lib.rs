mod component_names;
mod mesh;
mod resource_names;
mod systems;

use bbecs::data_types::point::Point;
use bbecs::resources::resource::Resource;
use bbecs::world::{World, WorldMethods};
use component_names::ComponentNames;
use ggez::conf::WindowMode;
use ggez::graphics::{drawable_size, Color, Rect};
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
use systems::handle_screen_size_change::handle_screen_size_change_system;
use systems::update_boid_color::update_boid_color_system;
use systems::update_locations::update_locations_system;
use systems::update_rotations::update_rotations_system;

pub struct FlockingRustState {
    world: World,
    has_resized: bool,
}

impl FlockingRustState {
    pub fn new(context: &mut Context) -> GameResult<Self> {
        let background_color = graphics::BLACK;
        let mut world = World::new();
        let boid_mesh = create_boid_mesh(context, 25.0)?;
        let sight_range = 50.0;

        world.register(
            ComponentNames::Acceleration,
            bbecs::components::Component::Point,
        );
        world.register(
            ComponentNames::Location,
            bbecs::components::Component::Point,
        );
        world.register(ComponentNames::Rotation, bbecs::components::Component::F32);
        world.register(
            ComponentNames::Velocity,
            bbecs::components::Component::Point,
        );

        world.add_resource(ResourceNames::BackgroundColor, background_color);
        world.add_resource(ResourceNames::BirdMesh, boid_mesh);
        let arena_size = graphics::drawable_size(context);
        world.add_resource(
            ResourceNames::ArenaSize,
            Point::new(arena_size.0, arena_size.1),
        );
        world.add_resource(ResourceNames::UpdateFps, 60_u32);
        world.add_resource(ResourceNames::SightRange, sight_range);
        world.add_resource(ResourceNames::AvoidRange, sight_range * 0.625);
        world.add_resource(ResourceNames::TurningSpeed, 0.5);
        world.add_resource(ResourceNames::AttractionTurningSpeed, 0.1);
        world.add_resource(ResourceNames::BoidColor, Color::new(0.0, 0.0, 0.0, 1.0));
        world.add_resource(ResourceNames::ColorChangeRate, 0.01);
        world.add_resource(ResourceNames::ColorChangeSpeed, 5_usize);
        world.add_resource(ResourceNames::ClearScreenMesh, create_clear_mesh(context)?);

        // Spawn the birds
        for _ in 0..500 {
            world
                .spawn_entity()
                .with_component(
                    ComponentNames::Location,
                    Point::new(
                        random::<f32>() * arena_size.0,
                        random::<f32>() * arena_size.1,
                    ),
                )
                .with_component(
                    ComponentNames::Velocity,
                    Point::new(
                        (rand::random::<f32>() - 0.5) * 5.0,
                        (rand::random::<f32>() - 0.5) * 5.0,
                    ),
                )
                .with_component(ComponentNames::Acceleration, Point::new(0.0, 0.0))
                .with_component(ComponentNames::Rotation, 0.0_f32);
        }

        Ok(Self {
            world,
            has_resized: false,
        })
    }
}

impl EventHandler for FlockingRustState {
    fn update(&mut self, context: &mut ggez::Context) -> GameResult {
        handle_screen_size_change_system(&mut self.world, context)?;
        let update_fps: &u32 = self
            .world
            .get_resource::<ResourceNames>(ResourceNames::UpdateFps);
        while timer::check_update_time(context, *update_fps) {
            update_boid_color_system(&mut self.world, timer::ticks(context));
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
        if self.has_resized {
            let background_color: &Color = self
                .world
                .get_resource::<ResourceNames>(ResourceNames::BackgroundColor);
            graphics::clear(context, *background_color);
            self.has_resized = false;
        }
        clear_screen_system(&self.world, context)?;
        draw_birds_system(context, &self.world)?;
        // visualize_ranges_system(&self.world, context)?;
        graphics::present(context)
    }

    fn resize_event(&mut self, context: &mut Context, width: f32, height: f32) {
        let mut screen_size: &mut Point = self
            .world
            .get_resource_mut::<ResourceNames>(ResourceNames::ArenaSize);

        screen_size.x = width;
        screen_size.y = height;
        self.has_resized = true;
    }
}
