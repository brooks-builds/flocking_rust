use bbecs::components::CastComponents;
use bbecs::data_types::point::Point;
use bbecs::world::{World, WorldMethods};
use ggez::graphics::{draw, Color, DrawMode, DrawParam, Mesh, MeshBuilder, WHITE};
use ggez::{Context, GameResult};

use crate::resource_names::ResourceNames;

#[allow(dead_code)]
pub fn visualize_ranges_system(world: &World, context: &mut Context) -> GameResult {
    let wrapped_locations = world
        .query_one(crate::component_names::ComponentNames::Location)
        .borrow();
    let locations: &Vec<Point> = wrapped_locations.cast();
    let sight_range: &f32 = world.get_resource::<ResourceNames>(crate::ResourceNames::SightRange);
    let avoidance_range: &f32 =
        world.get_resource::<ResourceNames>(crate::resource_names::ResourceNames::AvoidRange);
    let location = locations[0].to_array();

    let sight_range_mesh = create_range_mesh(context, *sight_range, WHITE, location)?;
    let avoid_range_mesh = create_range_mesh(
        context,
        *avoidance_range,
        Color::new(1.0, 0.0, 0.0, 1.0),
        location,
    )?;

    draw(context, &sight_range_mesh, DrawParam::default())?;
    draw(context, &avoid_range_mesh, DrawParam::default())?;
    Ok(())
}

fn create_range_mesh(
    context: &mut Context,
    radius: f32,
    color: Color,
    location: [f32; 2],
) -> GameResult<Mesh> {
    MeshBuilder::new()
        .circle(DrawMode::stroke(1.0), location, radius, 0.1, color)
        .build(context)
}
