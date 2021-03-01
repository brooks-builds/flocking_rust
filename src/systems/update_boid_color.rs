use bbecs::world::World;

pub fn update_boid_color_system(world: &World, ticks: usize) {
    let color_change_every = world
        .get_resource(&crate::resource_names::ResourceNames::ColorChangeSpeed)
        .borrow()
        .cast_usize();
    let color_change_rate = world
        .get_resource(&crate::resource_names::ResourceNames::ColorChangeRate)
        .borrow()
        .cast_f32();

    if ticks % color_change_every == 0 {
        let mut wrapped_boid_color = world
            .get_resource(&crate::resource_names::ResourceNames::BoidColor)
            .borrow_mut();
        let boid_color = wrapped_boid_color.cast_color_mut();

        if boid_color.r < 1.0 && boid_color.g <= 0.0 && boid_color.b <= 0.0 {
            boid_color.r += color_change_rate;
        } else if boid_color.r >= 1.0 && boid_color.g < 1.0 && boid_color.b <= 0.0 {
            boid_color.g += color_change_rate;
        } else if boid_color.r >= 1.0 && boid_color.g >= 1.0 && boid_color.b < 1.0 {
            boid_color.b += color_change_rate;
        } else if boid_color.r > 0.0 && boid_color.g >= 1.0 && boid_color.b >= 1.0 {
            boid_color.r -= color_change_rate;
        } else if boid_color.r <= 0.0 && boid_color.g > 0.0 && boid_color.b >= 1.0 {
            boid_color.g -= color_change_rate;
        } else if boid_color.r <= 0.0 && boid_color.g <= 0.0 && boid_color.b > 0.0 {
            boid_color.b -= color_change_rate;
        }
    }
}
