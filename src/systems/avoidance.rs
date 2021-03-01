use bbecs::data_types::point::Point;
use bbecs::world::World;

use crate::resource_names::ResourceNames;

/// we want all of the birds to avoid each other. We will be doing this by querying for all
/// of the birds within range of each other, then accelerating away from each of these birds.
pub fn avoidance_system(world: &World) {
    let avoid_range = world
        .get_resource(&crate::resource_names::ResourceNames::AvoidRange)
        .borrow()
        .cast_f32();
    let locations = world.query_one(&crate::component_names::ComponentNames::Location);
    let mut accelerations = world
        .query_one(&crate::component_names::ComponentNames::Acceleration)
        .borrow_mut();
    let turning_speed = world
        .get_resource(&ResourceNames::TurningSpeed)
        .borrow()
        .cast_f32();

    locations
        .clone()
        .borrow()
        .iter()
        .enumerate()
        .for_each(|(index, location)| {
            let my_location = location.cast_point();
            locations.clone().borrow().iter().enumerate().for_each(
                |(other_index, other_location)| {
                    if index == other_index {
                        return;
                    }
                    let other_location = other_location.cast_point();
                    let distance = *my_location - *other_location;
                    if my_location.distance_to(other_location) < avoid_range {
                        let acceleration = accelerations[index].cast_point_mut();
                        let force = create_avoidance_force(distance, turning_speed);
                        *acceleration += force;
                    }
                },
            )
        });
}

fn create_avoidance_force(distance: Point, turning_speed: f32) -> Point {
    let mut force = distance;
    force.normalize();
    force.multiply_scalar(turning_speed);
    force
}
