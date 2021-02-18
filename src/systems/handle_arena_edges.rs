use bbecs::world::World;

use crate::component_names::ComponentNames;
use crate::resource_names::ResourceNames;

pub fn handle_arena_edges_system(world: &World<ComponentNames, ResourceNames>) {
    let borrowed_arena_size = world.get_resource(&ResourceNames::ArenaSize).borrow();
    let arena_size = borrowed_arena_size.cast_point();
    let mut locations = world.query_one(&ComponentNames::Location).borrow_mut();
    let mut velocities = world.query_one(&ComponentNames::Velocity).borrow_mut();

    locations
        .iter_mut()
        .enumerate()
        .for_each(|(index, location)| {
            let mut location = location.cast_point_mut();
            let mut velocity = velocities[index].cast_point_mut();

            if location.y < 0.0 {
                location.y = 0.0;
                velocity.y *= -1.0;
            } else if location.y > arena_size.y {
                location.y = arena_size.y;
                velocity.y *= -1.0;
            }

            if location.x < 0.0 {
                location.x = 0.0;
                velocity.x *= -1.0;
            } else if location.x > arena_size.x {
                location.x = arena_size.x;
                velocity.x *= -1.0;
            }
        });
}
