use ggez::graphics::{Mesh, MeshBuilder, WHITE};
use ggez::{Context, GameResult};

pub fn create_boid_mesh(context: &mut Context, boid_size: f32) -> GameResult<Mesh> {
    let radius = boid_size / 2.0;
    let triangles = [
        [radius, 0.0],
        [-radius, -radius / 2.0],
        [-radius, radius / 2.0],
    ];
    MeshBuilder::new()
        .triangles(&triangles, WHITE)?
        .build(context)
}
