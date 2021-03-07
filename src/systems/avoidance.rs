use bbecs::components::CastComponents;
use bbecs::data_types::point::Point;
use bbecs::world::{World, WorldMethods};

use crate::resource_names::ResourceNames;

/// we want all of the birds to avoid each other. We will be doing this by querying for all
/// of the birds within range of each other, then accelerating away from each of these birds.
pub fn avoidance_system(world: &World) {
    let avoid_range: &f32 = world
        .get_resource(crate::resource_names::ResourceNames::AvoidRange)
        .unwrap();
    let locations = world
        .query_one(crate::component_names::ComponentNames::Location)
        .unwrap();
    let mut wrapped_accelerations = world
        .query_one(crate::component_names::ComponentNames::Acceleration)
        .unwrap()
        .borrow_mut();
    let accelerations: &mut Vec<Point> = wrapped_accelerations.cast_mut().unwrap();
    let turning_speed: &f32 = world.get_resource(ResourceNames::TurningSpeed).unwrap();

    locations
        .clone()
        .borrow()
        .cast()
        .unwrap()
        .iter()
        .enumerate()
        .for_each(|(index, location): (usize, &Point)| {
            let my_location = location;
            locations
                .clone()
                .borrow()
                .cast()
                .unwrap()
                .iter()
                .enumerate()
                .for_each(|(other_index, other_location): (usize, &Point)| {
                    if index == other_index {
                        return;
                    }
                    let other_location = other_location;
                    let distance = *my_location - *other_location;
                    if my_location.distance_to(other_location) < *avoid_range {
                        let acceleration = &mut accelerations[index];
                        let force = create_avoidance_force(distance, *turning_speed);
                        *acceleration += force;
                    }
                })
        });
}

fn create_avoidance_force(distance: Point, turning_speed: f32) -> Point {
    let mut force = distance;
    force.normalize();
    force.multiply_scalar(turning_speed);
    force
}
