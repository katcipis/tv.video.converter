use std::thread;
use std::path::PathBuf;
use crossbeam_channel;
use std::env;

#[derive(Debug)]
struct TranscodeJob {
    src: PathBuf,
    dest: PathBuf,
}

fn main() {
    const NTHREADS: i32 = 4;

    let args: Vec<String> = env::args().collect();

    let path = match args.len() {
        2 => PathBuf::from(&args[1]),
        _ => env::current_dir().unwrap(),
    };

    println!("looking for videos at: {}", path.display());

    thread::scope(|s| {
        let (job_sender, job_recv) = crossbeam_channel::bounded::<TranscodeJob>(0);

        for i in 0..NTHREADS {
            let job_recv = job_recv.clone();

            s.spawn(move || {
                println!("starting worker {}", i);
                while let Ok(job) = job_recv.recv() {
                    println!("worker {} transcoding from {} to {}", i,
                        job.src.display(), job.dest.display());
                }
                println!("worker {} is done", i);
            });
        }
        println!("starting job scheduler");

        let job = TranscodeJob{
            src: PathBuf::from("src"),
            dest: PathBuf::from("dest"),
        };
        job_sender.send(job).expect("failed to send job to workers");
        drop(job_sender);
    });

    println!("done");
}
