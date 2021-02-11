use bbecs::world::query::Query;
use bbecs::world::World;
use ggez::graphics::{self, Mesh};
use ggez::{Context, GameResult};
use graphics::DrawParam;

use crate::components::{Location, Velocity};

pub fn draw_system(world: &World, context: &mut Context, mesh: &Mesh) -> GameResult {
    let results = world.query(Query::new().with_type::<Location>());
    let locations = results[0];
    for location in locations {
        if let Ok(location) = location.clone().downcast::<Location>() {
            graphics::draw(
                context,
                mesh,
                DrawParam::default().dest([location.x, location.y]),
            )?
        }
    }

    Ok(())
}

pub fn update_locations(world: &mut World) {
    let velocities = world.query(Query::new().with_type::<Velocity>())[0];
}
