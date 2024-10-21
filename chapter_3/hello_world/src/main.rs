use std::str;

use mpi::traits::{Communicator, Destination, Source};

fn main() {
    let universe = mpi::initialize().expect("unable to initialize mpi universe");
    let world = universe.world();
    let comm_size = world.size();
    let my_rank = world.rank();

    if my_rank != 0 {
        let greeting = format!("Greetings from process {my_rank} of {comm_size}!");
        let msg = greeting.as_bytes();
        world.process_at_rank(0).send(msg);
    } else {
        println!("Greetings from process {my_rank} of {comm_size}!");
        for q in 1..comm_size {
            let (msg, status) = world.process_at_rank(q).receive_vec::<u8>();
            println!("Process {my_rank} received a message with status {status:?}");
            let greeting = str::from_utf8(&msg).unwrap();
            println!("{greeting}");
        }
    }
}
