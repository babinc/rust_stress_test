use std::{process, thread};
use device_query::{DeviceQuery, DeviceState, Keycode};

fn main() {
    let num_threads = num_cpus::get();
    println!("Threads in this system: {}", num_threads + 1);
    println!("Using {} threads for stress test", num_threads + 1);
    for i in 1..num_threads {
        println!("Spawning thread number {}", i);
        thread::spawn (|| {
            worker();
        });
    }
    println!("Using main as last thread");
    worker();
}

fn worker() {
    let mut _x = 0;
    loop {
        let keys: Vec<Keycode> = DeviceState.get_keys();
        for key in keys.iter() {
            if *key == Keycode::Q {
                process::exit(0);
            }
        }
        _x += 1;
        _x -= 1;
    }
}
