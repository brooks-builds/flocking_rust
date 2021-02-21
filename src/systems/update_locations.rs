use bbecs::world::World;

use crate::component_names::ComponentNames;
use crate::resource_names::ResourceNames;

pub fn update_locations_system(world: &World<ComponentNames, ResourceNames>) {
    let mut locations = world.query_one(&ComponentNames::Location).borrow_mut();
    let mut velocities = world.query_one(&ComponentNames::Velocity).borrow_mut();
    let mut accelerations = world.query_one(&ComponentNames::Acceleration).borrow_mut();

    locations
        .iter_mut()
        .enumerate()
        .for_each(|(index, location_component)| {
            let location = location_component.cast_point_mut();
            let velocity = velocities[index].cast_point_mut();
            let acceleration = accelerations[index].cast_point_mut();

            *velocity += *acceleration;
            velocity.normalize();
            velocity.multiply_scalar(2.5);
            location.add(velocity);
            acceleration.multiply_scalar(0.0);
        })
}
