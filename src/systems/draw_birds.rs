use bbecs::world::World;
use ggez::graphics::{self, Mesh};
use ggez::{Context, GameResult};
use graphics::DrawParam;

/// Query for the locations and then draw them out using GGEZs draw method
pub fn draw_birds_system(context: &mut Context, mesh: &Mesh, world: &mut World) -> GameResult {
    if let Some(components) = world.query("location") {
        components.get_mut().iter_mut().try_for_each(|component| {
            if let Some(location) = component.get_vector_2() {
                graphics::draw(
                    context,
                    mesh,
                    DrawParam::default().dest([location.x, location.y]),
                )
            } else {
                Ok(())
            }
        })
    } else {
        Ok(())
    }
}
