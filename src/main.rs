use std::cell::UnsafeCell;

use unsafe_cell_ecs::{data::Data, World};

fn main() {
    let mut world = World::new();
    world.insert_resource("world size", Data::F32(UnsafeCell::new(100.0)));

    let world_size = world.get_resource("world size").unwrap();
    let mut data = world_size.cast_f32();
    *data += 10.0;
    dbg!(world.get_resource("world size").unwrap().cast_f32());
}
