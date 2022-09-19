use std::thread;
use std::path::Path;
use crossbeam_channel;

fn main() {
    const NTHREADS: i32 = 4;

    let (job_sender, job_recv) = crossbeam_channel::bounded::<&Path>(0);

    thread::scope(|s| {
        for _ in 0..NTHREADS {
            s.spawn(|| {
                println!("starting worker");
                while let Ok(job) = job_recv.recv() {
                    println!("got job {}", job.display());
                }
                println!("worker is done");
            });
        }
        println!("starting job scheduler");

        job_sender.send(Path::new("hi")).expect("failed to send job to workers");
        job_sender.send(Path::new("hi 2")).expect("failed to send job to workers");
        drop(job_sender);
    });

    println!("done");
}

