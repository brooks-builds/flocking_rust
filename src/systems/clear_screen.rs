use ggez::graphics::{draw, DrawParam};
use ggez::{Context, GameResult};

use crate::WorldWrapper;

pub fn clear_screen_system(world: &WorldWrapper, context: &mut Context) -> GameResult {
    let wrapped_mesh = world
        .get_resource(&crate::resource_names::ResourceNames::ClearScreenMesh)
        .borrow();
    let mesh = wrapped_mesh.cast_mesh();
    draw(context, mesh, DrawParam::default())
}
