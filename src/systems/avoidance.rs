use bbecs::components::point::Point;

use crate::WorldWrapper;

/// we want all of the birds to avoid each other. We will be doing this by querying for all
/// of the birds within range of each other, then accelerating away from each of these birds.
pub fn avoidance_system(world: &WorldWrapper) {
    let sight_range = world
        .get_resource(&crate::resource_names::ResourceNames::SightRange)
        .borrow()
        .cast_f32()
        * 0.005;
    let locations = world.query_one(&crate::component_names::ComponentNames::Location);
    let mut accelerations = world
        .query_one(&crate::component_names::ComponentNames::Acceleration)
        .borrow_mut();
    let velocities = world
        .query_one(&crate::component_names::ComponentNames::Velocity)
        .borrow_mut();

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
                    let distance = *other_location - *my_location;
                    if distance.length() < sight_range {
                        let acceleration = accelerations[index].cast_point_mut();
                        acceleration.add(&create_avoidance_force(velocities[index].cast_point()));
                    }
                },
            )
        });
}

fn create_avoidance_force(velocity: &Point) -> Point {
    let mut force = velocity.to_perpendicular_right();
    force.normalize();
    force.multiply_scalar(0.1);
    force
}
