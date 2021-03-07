use bbecs::components::CastComponents;
use bbecs::data_types::point::Point;
use bbecs::world::{World, WorldMethods};
use ggez::graphics::{Color, Mesh};
use ggez::{graphics, Context, GameResult};
use graphics::DrawParam;

use crate::component_names::ComponentNames;
use crate::resource_names::ResourceNames;

/// Query for the locations and then draw them out using GGEZ's draw method
pub fn draw_birds_system(context: &mut Context, world: &World) -> GameResult {
    let mesh: &Mesh = world.get_resource(ResourceNames::BirdMesh).unwrap();
    let color: &Color = world.get_resource(ResourceNames::BoidColor).unwrap();
    let wrapped_locations = world
        .query_one::<ComponentNames>(ComponentNames::Location)
        .unwrap()
        .borrow();
    let locations: &Vec<Point> = wrapped_locations.cast().unwrap();
    let wrapped_rotations = world
        .query_one::<ComponentNames>(ComponentNames::Rotation)
        .unwrap()
        .borrow();
    let rotations: &Vec<f32> = wrapped_rotations.cast().unwrap();
    locations
        .iter()
        .enumerate()
        .try_for_each(|(index, location)| {
            let rotation = rotations[index];
            graphics::draw(
                context,
                mesh,
                DrawParam::default()
                    .dest(location.to_array())
                    .rotation(rotation)
                    .color(*color),
            )
        })
}
