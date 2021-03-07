use std::cell::Ref;

use bbecs::components::{CastComponents, Components};
use bbecs::data_types::point::Point;
use bbecs::world::{World, WorldMethods};

use crate::resource_names::ResourceNames;

pub fn attraction_system(world: &World) {
    let sight_range: &f32 = world
        .get_resource::<ResourceNames>(crate::resource_names::ResourceNames::SightRange)
        .unwrap();
    let wrapped_locations = world
        .query_one(crate::component_names::ComponentNames::Location)
        .unwrap();
    let turning_speed: &f32 = world
        .get_resource::<ResourceNames>(crate::resource_names::ResourceNames::AttractionTurningSpeed)
        .unwrap();

    wrapped_locations
        .clone()
        .borrow()
        .cast()
        .unwrap()
        .iter()
        .enumerate()
        .for_each(|location: (_, &Point)| {
            handle_location(
                location,
                wrapped_locations.clone().borrow(),
                *sight_range,
                world,
                *turning_speed,
            )
        });
}

fn handle_location(
    (index, location): (usize, &Point),
    other_locations: Ref<Components>,
    sight_range: f32,
    world: &World,
    turning_speed: f32,
) {
    let boids_near_me = get_boids_near_me(index, other_locations.cast().unwrap(), sight_range);
    if let Some(average_location_of_other_boids) = calculate_average_locations(boids_near_me) {
        let mut force = average_location_of_other_boids - *location;
        let mut wrapped_accelerations = world
            .query_one(crate::component_names::ComponentNames::Acceleration)
            .unwrap()
            .borrow_mut();
        let accelerations: &mut Vec<Point> = wrapped_accelerations.cast_mut().unwrap();
        force.normalize();
        force.multiply_scalar(turning_speed);

        accelerations[index] += force;
    }
}

fn get_boids_near_me(index: usize, all_locations: &[Point], sight_range: f32) -> Vec<Point> {
    let my_location = all_locations[index];
    all_locations.iter().enumerate().fold(
        vec![],
        |mut boids_near_me, (other_index, wrapped_other_location)| {
            if index == other_index {
                boids_near_me
            } else {
                let other_location = wrapped_other_location;
                if my_location.distance_to(other_location) <= sight_range {
                    boids_near_me.push(*other_location);
                }
                boids_near_me
            }
        },
    )
}

fn calculate_average_locations(boids: Vec<Point>) -> Option<Point> {
    let boids_count = boids.len();
    if boids_count == 0 {
        None
    } else {
        let average_location: Point = boids.into_iter().sum();
        Some(average_location / boids_count)
    }
}
