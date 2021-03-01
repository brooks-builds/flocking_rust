use bbecs::world::World;
use ggez::graphics::{self, draw, DrawParam, BLACK};
use ggez::{Context, GameResult};

pub fn clear_screen_system(world: &World, context: &mut Context) -> GameResult {
    // let wrapped_mesh = world
    //     .get_resource(&crate::resource_names::ResourceNames::ClearScreenMesh)
    //     .borrow();
    // let mesh = wrapped_mesh.cast_mesh();
    // draw(context, mesh, DrawParam::default())
    graphics::clear(context, BLACK);
    Ok(())
}
