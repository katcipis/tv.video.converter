use std::thread;

fn main() {
    const NTHREADS: i32 = 4;

    thread::scope(|s| {
        for i in 0..NTHREADS {
            s.spawn(move || {
                println!("starting worker {}", i);
            });
        }
        println!("starting job scheduler");
    });

    println!("done");
}

