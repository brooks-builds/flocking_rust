use bbecs::components::CastComponents;
use bbecs::data_types::point::Point;
use bbecs::world::World;

use crate::component_names::ComponentNames;

pub fn update_rotations_system(world: &World) {
    let mut wrapped_rotations = world.query_one(ComponentNames::Rotation).borrow_mut();
    let rotations: &mut Vec<f32> = wrapped_rotations.cast_mut();
    let wrapped_velocities = world.query_one(ComponentNames::Velocity).borrow();
    let velocities: &Vec<Point> = wrapped_velocities.cast();

    rotations
        .iter_mut()
        .enumerate()
        .for_each(|(index, rotation)| {
            let velocity = velocities[index];

            *rotation = velocity.rotation();
        });
}
