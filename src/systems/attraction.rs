use std::borrow::BorrowMut;
use std::cell::{Ref, RefMut};
use std::ops::{Deref, DerefMut};

use bbecs::components::point::Point;
use bbecs::components::Component;

use crate::WorldWrapper;

pub fn attraction_system(world: &WorldWrapper) {
    let sight_range = world
        .get_resource(&crate::resource_names::ResourceNames::SightRange)
        .borrow()
        .cast_f32()
        * 1.25;
    let wrapped_locations = world.query_one(&crate::component_names::ComponentNames::Location);

    wrapped_locations
        .clone()
        .borrow()
        .iter()
        .enumerate()
        .for_each(|location| {
            handle_location(
                location,
                wrapped_locations.clone().borrow(),
                sight_range,
                world,
            )
        });
}

fn handle_location(
    (index, location): (usize, &Component),
    other_locations: Ref<Vec<Component>>,
    sight_range: f32,
    world: &WorldWrapper,
) {
    let location = location.cast_point();
    let boids_near_me = get_boids_near_me(index, other_locations, sight_range);
    if let Some(average_location_of_other_boids) = calculate_average_locations(boids_near_me) {
        let mut force = *location - average_location_of_other_boids;
        let mut accelerations = world
            .query_one(&crate::component_names::ComponentNames::Acceleration)
            .deref()
            .borrow_mut();
        force.normalize();
        force.multiply_scalar(0.5);

        *accelerations[index].cast_point_mut() += force;
    }
}

fn get_boids_near_me(
    index: usize,
    all_locations: Ref<Vec<Component>>,
    sight_range: f32,
) -> Vec<Point> {
    let my_location = all_locations[index].cast_point();
    all_locations.iter().enumerate().fold(
        vec![],
        |mut boids_near_me, (other_index, wrapped_other_location)| {
            if index == other_index {
                boids_near_me
            } else {
                let other_location = wrapped_other_location.cast_point();
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
