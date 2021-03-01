use bbecs::resources::resource::Resource;
use bbecs::world::World;
use ggez::graphics::{set_screen_coordinates, Rect};
use ggez::{Context, GameResult};

use crate::mesh;

pub fn handle_screen_size_change_system(world: &mut World, context: &mut Context) -> GameResult {
    {
        let clear_mesh = mesh::create_clear_mesh(context)?;
        world.add_resource(
            crate::resource_names::ResourceNames::ClearScreenMesh,
            Resource::Mesh(clear_mesh),
        );
    }

    let wrapped_arena_size = world
        .get_resource(&crate::resource_names::ResourceNames::ArenaSize)
        .borrow();
    let arena_size = wrapped_arena_size.cast_point();
    let arena_rect = Rect::new(0.0, 0.0, arena_size.x, arena_size.y);

    set_screen_coordinates(context, arena_rect)?;

    Ok(())
}
