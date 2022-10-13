use std::thread;
use std::path::PathBuf;
use crossbeam_channel;
use std::env;
use std::fs;

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
            s.spawn(move || transcoder(i, job_recv));
        }

        jobscheduler(path, job_sender);
    });

    println!("done");
}

fn jobscheduler(path: PathBuf, job_sender: crossbeam_channel::Sender<TranscodeJob>) {
        println!("starting job scheduler");

        let entries = fs::read_dir(path).unwrap();

        let job = TranscodeJob{
            src: PathBuf::from("src"),
            dest: PathBuf::from("dest"),
        };
        job_sender.send(job).expect("failed to send job to workers");
        drop(job_sender);
}

fn transcoder(id: i32, job_recv: crossbeam_channel::Receiver<TranscodeJob>) {
    println!("starting transcoder {}", id);

    while let Ok(job) = job_recv.recv() {
        println!("transcoder {} transcoding from {} to {}", id,
            job.src.display(), job.dest.display());
    }

    println!("finishing transcoder {}", id);
}
