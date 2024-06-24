use crate::world::World;

mod bunny;
mod constants;
mod utils;
mod world;

fn main() {
    let mut bunny_world = World::new();

    loop {
        match bunny_world.progress() {
            Ok(events) => {}
            Err(error) => {}
        }
    }
}
