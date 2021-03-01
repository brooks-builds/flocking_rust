use bbecs::data_types::point::Point;
use bbecs::world::World;

use crate::component_names::ComponentNames;
use crate::resource_names::ResourceNames;

pub fn handle_arena_edges_system(world: &World) {
    let borrowed_arena_size = world.get_resource(&ResourceNames::ArenaSize).borrow();
    let arena_size = borrowed_arena_size.cast_point();
    let locations = world.query_one(&ComponentNames::Location).borrow();
    let velocities = world.query_one(&ComponentNames::Velocity).borrow();
    let mut accelerations = world.query_one(&ComponentNames::Acceleration).borrow_mut();
    let sight_range = world
        .get_resource(&ResourceNames::SightRange)
        .borrow()
        .cast_f32();
    let turning_speed = world
        .get_resource(&ResourceNames::TurningSpeed)
        .borrow()
        .cast_f32();

    locations.iter().enumerate().for_each(|(index, location)| {
        let location = location.cast_point();
        let velocity = velocities[index].cast_point();
        let acceleration = accelerations[index].cast_point_mut();

        let mut force = Point::default();
        if location.x > arena_size.x - sight_range {
            if velocity.y >= 0.0 {
                // We are going to turn right
                force = velocity.to_perpendicular_right();
            } else {
                // We are going to turn left
                force = velocity.to_perpendicular_left();
            }
        } else if location.x < sight_range {
            if velocity.y >= 0.0 {
                // We are going to turn left because it will be faster to avoid the wall
                force = velocity.to_perpendicular_left();
            } else {
                // We are going to turn right
                force = velocity.to_perpendicular_right();
            }
        }

        if location.y < sight_range {
            if velocity.x >= 0.0 {
                force = velocity.to_perpendicular_right();
            } else {
                force = velocity.to_perpendicular_left();
            }
        } else if location.y > arena_size.y - sight_range {
            if velocity.x >= 0.0 {
                force = velocity.to_perpendicular_left();
            } else {
                force = velocity.to_perpendicular_right();
            }
        }
        force.normalize();
        force.multiply_scalar(turning_speed);
        acceleration.add(&force);
    });
}
