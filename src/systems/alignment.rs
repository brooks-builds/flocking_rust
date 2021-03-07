use bbecs::components::CastComponents;
use bbecs::data_types::point::Point;
use bbecs::world::{World, WorldMethods};

use crate::resource_names::ResourceNames;

pub fn alignment_system(world: &World) {
    let locations_wrapper = world
        .query_one(crate::component_names::ComponentNames::Location)
        .unwrap();
    let velocities_wrapper = world
        .query_one(crate::component_names::ComponentNames::Velocity)
        .unwrap();
    let mut wrapped_accelerations = world
        .query_one(crate::component_names::ComponentNames::Acceleration)
        .unwrap()
        .borrow_mut();
    let accelerations: &mut Vec<Point> = wrapped_accelerations.cast_mut().unwrap();
    let sight_range: &f32 = world
        .get_resource::<ResourceNames>(crate::resource_names::ResourceNames::SightRange)
        .unwrap();
    let turning_speed: &f32 = world
        .get_resource::<ResourceNames>(ResourceNames::TurningSpeed)
        .unwrap();

    locations_wrapper
        .clone()
        .borrow()
        .cast()
        .unwrap()
        .iter()
        .enumerate()
        .for_each(|(my_index, my_location): (usize, &Point)| {
            let nearby_indexes = get_nearby_indexes(
                my_location,
                locations_wrapper.clone().borrow().cast().unwrap(),
                *sight_range,
            );
            let nearby_velocities = get_velocities_by_index(
                velocities_wrapper.borrow().cast().unwrap(),
                nearby_indexes,
            );
            if !nearby_velocities.is_empty() {
                let mut average_velocity = calculate_average_velocity(nearby_velocities);
                average_velocity.normalize();
                average_velocity.multiply_scalar(*turning_speed);
                accelerations[my_index].add(&average_velocity);
            }
        })
}

fn get_nearby_indexes(
    my_location: &Point,
    other_locations: &[Point],
    sight_range: f32,
) -> Vec<usize> {
    other_locations
        .iter()
        .enumerate()
        .filter_map(|(index, other_location)| {
            let other_location = other_location;
            if *my_location == *other_location {
                return None;
            }

            if my_location.distance_to(other_location) > sight_range {
                return None;
            }

            Some(index)
        })
        .collect()
}

fn get_velocities_by_index(velocities: &[Point], indexes: Vec<usize>) -> Vec<Point> {
    velocities
        .iter()
        .enumerate()
        .filter_map(|(index, velocity)| {
            if indexes.contains(&index) {
                Some(*velocity)
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
