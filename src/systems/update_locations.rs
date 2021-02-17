use bbecs::world::World;

pub fn update_locations_system(world: &World) {
    let mut locations = world.query_one("location").borrow_mut();
    let velocities = world.query_one("velocity").borrow();

    locations
        .iter_mut()
        .enumerate()
        .for_each(|(index, location_component)| {
            let location = location_component.cast_point_mut();
            let velocity = velocities[index].cast_point();

            location.add(velocity);
        })
}
