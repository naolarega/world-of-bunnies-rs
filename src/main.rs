#![allow(unused_attributes, dead_code)]

use std::{thread::sleep, time::Duration};

use crate::{
    constants::WorldError,
    world::{Events, World},
};

mod bunny;
mod constants;
mod utils;
mod world;

fn main() {
    let mut bunny_world = World::new();

    loop {
        match bunny_world.progress() {
            Ok(events) => {
                print_events(events);
                bunny_world.report();
                sleep(Duration::from_secs(2));
            }
            Err(WorldError::AllBunniesDead) => {
                eprintln!("All bunnies died");

                return;
            }
            Err(_) => {}
        }
    }
}

fn print_events(events: Vec<Events>) {
    use Events::*;

    println!("## World events ##\n");

    if events.is_empty() {
        println!("** No events **");
    } else {
        for event in events {
            let (name, radioactive, life_event) = match event {
                Born { name, radioactive } => (name, radioactive, "is born"),
                Died { name, radioactive } => (name, radioactive, "died"),
            };

            if radioactive {
                print!("Radioactive Mutant Vampire ");
            }

            println!("Bunny {} {}!", name, life_event);
        }
    }

    println!("\n## End of world events ##");
}
