use bbecs::world::World;
use ggez::graphics::{self, Mesh};
use ggez::{Context, GameResult};
use graphics::DrawParam;

/// Query for the locations and then draw them out using GGEZs draw method
pub fn draw_birds_system(context: &mut Context, mesh: &Mesh, world: &World) -> GameResult {
    let locations = world.query_one("location").borrow();
    locations.iter().try_for_each(|component| {
        let location = component.cast_point();
        graphics::draw(
            context,
            mesh,
            DrawParam::default().dest(location.to_array()),
        )
    })
}
