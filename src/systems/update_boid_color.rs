use bbecs::world::{World, WorldMethods};
use ggez::graphics::Color;

use crate::resource_names::ResourceNames;

pub fn update_boid_color_system(world: &mut World, ticks: usize) {
    let color_change_every: &usize =
        world.get_resource::<ResourceNames>(crate::resource_names::ResourceNames::ColorChangeSpeed);
    let color_change_rate: &f32 =
        world.get_resource::<ResourceNames>(crate::resource_names::ResourceNames::ColorChangeRate);
    let color_change_rate = color_change_rate.clone();

    if ticks % color_change_every == 0 {
        let mut boid_color: &mut Color = world
            .get_resource_mut::<ResourceNames>(crate::resource_names::ResourceNames::BoidColor);

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
