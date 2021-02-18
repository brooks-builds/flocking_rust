use bbecs::world::World;
use ggez::graphics::{self, Mesh};
use ggez::{Context, GameResult};
use graphics::DrawParam;

use crate::component_names::ComponentNames;

/// Query for the locations and then draw them out using GGEZ's draw method
pub fn draw_birds_system(
    context: &mut Context,
    mesh: &Mesh,
    world: &World<ComponentNames>,
) -> GameResult {
    let locations = world.query_one(&ComponentNames::Location).borrow();
    locations.iter().try_for_each(|component| {
        let location = component.cast_point();
        graphics::draw(
            context,
            mesh,
            DrawParam::default().dest(location.to_array()),
        )
    })
}
