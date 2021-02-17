use bbecs::world::World;
use ggez::graphics::{self, Mesh};
use ggez::{Context, GameResult};
use graphics::DrawParam;
use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;

/// Query for the locations and then draw them out using GGEZs draw method
pub fn draw_birds_system(context: &mut Context, world: Rc<RefCell<World>>) -> GameResult {
    let cloned_world = world.clone();
    let borrowed_world = cloned_world.borrow();
    let mesh = borrowed_world
        .get_resource("bird_mesh")
        .unwrap()
        .extract_ggez_mesh()
        .unwrap();
    let mut borrowed_world = world.deref().borrow_mut();
    if let Some(components) = borrowed_world.query("location") {
        components.get_mut().iter_mut().try_for_each(|component| {
            if let Some(location) = component.get_vector_2() {
                graphics::draw(
                    context,
                    mesh,
                    DrawParam::default().dest([location.x, location.y]),
                )
            } else {
                Ok(())
            }
        })
    } else {
        Ok(())
    }
}
