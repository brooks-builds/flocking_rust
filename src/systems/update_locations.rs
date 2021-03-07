use bbecs::components::CastComponents;
use bbecs::data_types::point::Point;
use bbecs::world::World;

use crate::component_names::ComponentNames;

pub fn update_locations_system(world: &World) {
    let mut wrapped_locations = world
        .query_one(ComponentNames::Location)
        .unwrap()
        .borrow_mut();
    let locations = wrapped_locations.cast_mut().unwrap();
    let mut wrapped_velocities = world
        .query_one(ComponentNames::Velocity)
        .unwrap()
        .borrow_mut();
    let velocities: &mut Vec<Point> = wrapped_velocities.cast_mut().unwrap();
    let mut wrapped_accelerations = world
        .query_one(ComponentNames::Acceleration)
        .unwrap()
        .borrow_mut();
    let accelerations: &mut Vec<Point> = wrapped_accelerations.cast_mut().unwrap();

    locations
        .iter_mut()
        .enumerate()
        .for_each(|(index, location): (usize, &mut Point)| {
            let velocity = &mut velocities[index];
            let acceleration = &mut accelerations[index];

            *velocity += *acceleration;
            velocity.normalize();
            velocity.multiply_scalar(2.5);
            location.add(&velocity);
            acceleration.multiply_scalar(0.0);
        })
}
