use ggez::graphics::{drawable_size, Color, DrawMode, Mesh, MeshBuilder, Rect};
use ggez::{Context, GameResult};

pub fn create_boid_mesh(context: &mut Context, boid_size: f32) -> GameResult<Mesh> {
    let radius = boid_size / 2.0;
    let triangles = [
        [radius, 0.0],
        [-radius, -radius / 2.0],
        [-radius, radius / 2.0],
    ];
    let color = Color::new(1.0, 1.0, 1.0, 1.0);
    MeshBuilder::new()
        .triangles(&triangles, color)?
        .build(context)
}

pub fn create_clear_mesh(context: &mut Context) -> GameResult<Mesh> {
    let screen_size = drawable_size(context);
    let rect = Rect::new(0.0, 0.0, screen_size.0, screen_size.1);
    let color = Color::new(0.0, 0.0, 0.0, 0.04);
    MeshBuilder::new()
        .rectangle(DrawMode::fill(), rect, color)
        .build(context)
}
