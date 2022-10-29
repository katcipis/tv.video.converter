use crossbeam_channel::bounded;
use crossbeam_channel::Receiver;
use crossbeam_channel::Sender;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::ExitCode;
use std::thread;
use std::vec::Vec;

#[derive(Debug)]
struct TranscodeJob {
    src: PathBuf,
    dest: PathBuf,
}

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();

    let (srcdir, destdir) = match args.len() {
        1 => {
            let src = env::current_dir().unwrap();
            let mut dest = src.clone();
            dest.push("transcoded");
            (src, dest)
        }
        3 => (PathBuf::from(&args[1]), PathBuf::from(&args[2])),
        _ => {
            println!("usage using current dir: {}", args[0]);
            println!("usage explicit dirs: {} <src dir> <dest dir>", args[0]);
            std::process::exit(1)
        }
    };

    println!(
        "videos src: {} dest: {}",
        srcdir.display(),
        destdir.display()
    );

    thread::scope(|s| {
        const NTHREADS: i32 = 4;

        let (job_sender, job_recv) = bounded::<TranscodeJob>(0);

        for i in 0..NTHREADS {
            let job_recv = job_recv.clone();
            s.spawn(move || transcoder(i, job_recv));
        }

        jobscheduler(srcdir.as_path(), job_sender);
    });

    println!("done");

    ExitCode::SUCCESS
}

fn jobscheduler(path: &Path, job_sender: Sender<TranscodeJob>) {
    println!("starting job scheduler");

    let entries = fs::read_dir(path).unwrap();
    let mut srcs = Vec::new();

    for entry in entries {
        match entry {
            Ok(entry) => {
                let path = entry.path();
                if let Some(extension) = path.extension() {
                    let extension = extension
                        .to_str()
                        .expect("file extensions should be valid utf-8");

                    match extension {
                        "avi" => {
                            srcs.push(path);
                        }
                        _ => {
                            println!("ignoring file with extension: {}", extension)
                        }
                    }
                }
            }
            Err(err) => {
                print!("error reading dir {}: {}", path.display(), err);
            }
        }
    }

    let job = TranscodeJob {
        src: PathBuf::from("src"),
        dest: PathBuf::from("dest"),
    };
    job_sender.send(job).expect("failed to send job to workers");
    drop(job_sender);
}

fn transcoder(id: i32, job_recv: Receiver<TranscodeJob>) {
    println!("starting transcoder {}", id);

    while let Ok(job) = job_recv.recv() {
        println!(
            "transcoder {} transcoding from {} to {}",
            id,
            job.src.display(),
            job.dest.display()
        );
    }

    println!("finishing transcoder {}", id);
}
