use crate::component_names::ComponentNames;
use crate::WorldWrapper;

pub fn update_rotations_system(world: &WorldWrapper) {
    let mut rotations = world.query_one(&ComponentNames::Rotation).borrow_mut();
    let velocities = world.query_one(&ComponentNames::Velocity).borrow();

    rotations
        .iter_mut()
        .enumerate()
        .for_each(|(index, rotation)| {
            let rotation = rotation.cast_f32_mut();
            let velocity = velocities[index].cast_point();

            *rotation = velocity.rotation();
        });
}
