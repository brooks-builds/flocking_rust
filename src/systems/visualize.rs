use ggez::graphics::{draw, Color, DrawMode, DrawParam, Mesh, MeshBuilder, WHITE};
use ggez::{Context, GameResult};

use crate::WorldWrapper;

#[allow(dead_code)]
pub fn visualize_ranges_system(world: &WorldWrapper, context: &mut Context) -> GameResult {
    let locations = world
        .query_one(&crate::component_names::ComponentNames::Location)
        .borrow();
    let sight_range = world
        .get_resource(&crate::ResourceNames::SightRange)
        .borrow()
        .cast_f32();
    let avoidance_range = world
        .get_resource(&crate::resource_names::ResourceNames::AvoidRange)
        .borrow()
        .cast_f32();
    let location = locations[0].cast_point().to_array();

    let sight_range_mesh = create_range_mesh(context, sight_range, WHITE, location)?;
    let avoid_range_mesh = create_range_mesh(
        context,
        avoidance_range,
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
