use bbecs::components::CastComponents;
use bbecs::data_types::point::Point;
use bbecs::world::{World, WorldMethods};

use crate::component_names::ComponentNames;
use crate::resource_names::ResourceNames;

pub fn handle_arena_edges_system(world: &World) {
    let arena_size: &Point = world.get_resource(ResourceNames::ArenaSize).unwrap();
    let wrapped_locations = world.query_one(ComponentNames::Location).unwrap().borrow();
    let locations: &Vec<Point> = wrapped_locations.cast().unwrap();
    let wrapped_velocities = world.query_one(ComponentNames::Velocity).unwrap().borrow();
    let velocities: &Vec<Point> = wrapped_velocities.cast().unwrap();
    let mut wrapped_accelerations = world
        .query_one(ComponentNames::Acceleration)
        .unwrap()
        .borrow_mut();
    let accelerations: &mut Vec<Point> = wrapped_accelerations.cast_mut().unwrap();
    let sight_range: &f32 = world
        .get_resource::<ResourceNames>(ResourceNames::SightRange)
        .unwrap();
    let turning_speed: &f32 = world
        .get_resource::<ResourceNames>(ResourceNames::TurningSpeed)
        .unwrap();

    locations.iter().enumerate().for_each(|(index, location)| {
        let velocity = velocities[index];
        let acceleration = &mut accelerations[index];

        let mut force = Point::default();
        if location.x > arena_size.x - sight_range {
            if velocity.y >= 0.0 {
                // We are going to turn right
                force = velocity.to_perpendicular_right();
            } else {
                // We are going to turn left
                force = velocity.to_perpendicular_left();
            }
        } else if location.x < *sight_range {
            if velocity.y >= 0.0 {
                // We are going to turn left because it will be faster to avoid the wall
                force = velocity.to_perpendicular_left();
            } else {
                // We are going to turn right
                force = velocity.to_perpendicular_right();
            }
        }

        if location.y < *sight_range {
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
        force.multiply_scalar(*turning_speed);
        acceleration.add(&force);
    });
}
