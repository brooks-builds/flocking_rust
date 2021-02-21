use bbecs::components::point::Point;

use crate::WorldWrapper;

pub fn alignment_system(world: &WorldWrapper) {
    let locations_wrapper = world.query_one(&crate::component_names::ComponentNames::Location);
    let velocities_wrapper = world.query_one(&crate::component_names::ComponentNames::Velocity);
    let mut accelerations = world
        .query_one(&crate::component_names::ComponentNames::Acceleration)
        .borrow_mut();
    locations_wrapper
        .clone()
        .borrow()
        .iter()
        .enumerate()
        .for_each(|(my_index, my_location)| {
            let my_location = my_location.cast_point();
            let nearby_indexes =
                get_nearby_indexes(my_location, locations_wrapper.clone().borrow());
            let nearby_velocities =
                get_velocities_by_index(velocities_wrapper.borrow(), nearby_indexes);
            if !nearby_velocities.is_empty() {
                let mut average_velocity = calculate_average_velocity(nearby_velocities);
                average_velocity.normalize();
                average_velocity.multiply_scalar(0.1);
                accelerations[my_index]
                    .cast_point_mut()
                    .add(&average_velocity);
            }
        })
}

fn get_nearby_indexes(
    my_location: &Point,
    other_locations: std::cell::Ref<'_, std::vec::Vec<bbecs::components::Component>>,
) -> Vec<usize> {
    other_locations
        .iter()
        .enumerate()
        .filter_map(|(index, other_location)| {
            let other_location = other_location.cast_point();
            if *my_location == *other_location {
                return None;
            }

            if my_location.distance_to(other_location) > 100.0 {
                return None;
            }

            Some(index)
        })
        .collect()
}

fn get_velocities_by_index(
    velocities: std::cell::Ref<'_, std::vec::Vec<bbecs::components::Component>>,
    indexes: Vec<usize>,
) -> Vec<Point> {
    velocities
        .iter()
        .enumerate()
        .filter_map(|(index, velocity)| {
            if indexes.contains(&index) {
                Some(*velocity.cast_point())
            } else {
                None
            }
        })
        .collect()
}

fn calculate_average_velocity(velocities: Vec<Point>) -> Point {
    let summed_velocities = velocities
        .iter()
        .fold(Point::new(0.0, 0.0), |mut sum, velocity| {
            sum.add(velocity);
            sum
        });

    summed_velocities / velocities.len()
}
