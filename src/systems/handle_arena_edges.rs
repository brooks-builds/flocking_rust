use bbecs::components::point::Point;
use bbecs::world::World;

use crate::component_names::ComponentNames;
use crate::resource_names::ResourceNames;

pub fn handle_arena_edges_system(world: &World<ComponentNames, ResourceNames>) {
    let borrowed_arena_size = world.get_resource(&ResourceNames::ArenaSize).borrow();
    let arena_size = borrowed_arena_size.cast_point();
    let locations = world.query_one(&ComponentNames::Location).borrow();
    let velocities = world.query_one(&ComponentNames::Velocity).borrow();
    let mut accelerations = world.query_one(&ComponentNames::Acceleration).borrow_mut();
    let margin = 50.0;

    locations.iter().enumerate().for_each(|(index, location)| {
        let location = location.cast_point();
        let velocity = velocities[index].cast_point();
        let acceleration = accelerations[index].cast_point_mut();

        let mut force = Point::default();
        if location.x > arena_size.x - margin {
            if velocity.y >= 0.0 {
                // We are going to turn right
                force = velocity.to_perpendicular_right();
            } else {
                // We are going to turn left
                force = velocity.to_perpendicular_left();
            }
        } else if location.x < margin {
            if velocity.y >= 0.0 {
                // We are going to turn left because it will be faster to avoid the wall
                force = velocity.to_perpendicular_left();
            } else {
                // We are going to turn right
                force = velocity.to_perpendicular_right();
            }
        }

        if location.y < margin {
            if velocity.x >= 0.0 {
                force = velocity.to_perpendicular_right();
            } else {
                force = velocity.to_perpendicular_left();
            }
        } else if location.y > arena_size.y - margin {
            if velocity.x >= 0.0 {
                force = velocity.to_perpendicular_left();
            } else {
                force = velocity.to_perpendicular_right();
            }
        }
        force.normalize();
        force.multiply_scalar(0.1);
        acceleration.add(&force);
    });
}
