use bbecs::data_types::point::Point;
use bbecs::world::{World, WorldMethods};
use ggez::graphics::{set_screen_coordinates, Rect};
use ggez::{Context, GameResult};

use crate::resource_names::ResourceNames;

pub fn handle_screen_size_change_system(
    world: &mut World,
    context: &mut Context,
    has_resized: &mut bool,
) -> GameResult {
    let arena_size: &Point =
        world.get_resource::<ResourceNames>(crate::resource_names::ResourceNames::ArenaSize);
    let arena_rect = Rect::new(0.0, 0.0, arena_size.x, arena_size.y);

    set_screen_coordinates(context, arena_rect)?;

    *has_resized = false;

    Ok(())
}
