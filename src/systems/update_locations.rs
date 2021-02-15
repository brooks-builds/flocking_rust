use bbecs::components::Component;
use bbecs::world::World;
use ggez::nalgebra::Vector2;

/// We need to update all of the birds locations. We'll do this by mutably querying for the
/// locations and velocities (the velocities don't need to be queries mutably, just the
/// locations) and then add the velocities to the locations.
pub fn update_locations_system(world: &mut World) {
    let velocities = world.query("velocity").unwrap().get_mut().clone();
    let locations = world.query("location").unwrap().get_mut();

    assert!(locations.len() == velocities.len());

    for index in 0..locations.len() {
        let velocity = velocities[index].get_vector_2().unwrap();
        let location = locations[index].get_vector_2_mut().unwrap();

        location.x += velocity.x;
        location.y += velocity.y;
    }
}
