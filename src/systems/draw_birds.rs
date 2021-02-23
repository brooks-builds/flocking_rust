use bbecs::world::World;
use ggez::{graphics, Context, GameResult};
use graphics::DrawParam;

use crate::component_names::ComponentNames;
use crate::resource_names::ResourceNames;

/// Query for the locations and then draw them out using GGEZ's draw method
pub fn draw_birds_system(
    context: &mut Context,
    world: &World<ComponentNames, ResourceNames>,
) -> GameResult {
    let borrowed_mesh = world.get_resource(&ResourceNames::BirdMesh).borrow();
    let mesh = borrowed_mesh.cast_mesh();
    let locations = world.query_one(&ComponentNames::Location).borrow();
    let rotations = world.query_one(&ComponentNames::Rotation).borrow();
    locations
        .iter()
        .enumerate()
        .try_for_each(|(index, location)| {
            let location = location.cast_point();
            let rotation = rotations[index].cast_f32();
            graphics::draw(
                context,
                mesh,
                DrawParam::default()
                    .dest(location.to_array())
                    .rotation(*rotation),
            )
        })
}
