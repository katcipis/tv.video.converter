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

fn main() -> Result<(), ExitCode> {
    let (srcdir, destdir) = parse_args()?;

    println!(
        "videos src: {} dest: {}",
        srcdir.display(),
        destdir.display()
    );

    if srcdir == destdir {
        println!("src and dest dirs must differ");
        return Err(ExitCode::FAILURE);
    }

    thread::scope(|s| {
        const NTHREADS: i32 = 4;

        let (job_sender, job_recv) = bounded::<TranscodeJob>(0);

        for i in 0..NTHREADS {
            let job_recv = job_recv.clone();
            s.spawn(move || transcoder(i, job_recv));
        }

        jobscheduler(srcdir.as_path(), destdir.as_path(), job_sender);
    });

    println!("done");

    Ok(())
}

fn jobscheduler(srcdir: &Path, destdir: &Path, job_sender: Sender<TranscodeJob>) {
    println!("starting job scheduler");

    let entries = fs::read_dir(srcdir).unwrap();
    let mut srcs = Vec::new();

    for entry in entries {
        match entry {
            Ok(entry) => {
                let srcdir = entry.path();
                if let Some(extension) = srcdir.extension() {
                    let extension = extension
                        .to_str()
                        .expect("file extensions should be valid utf-8");

                    match extension {
                        "avi" => {
                            srcs.push(srcdir);
                        }
                        _ => {
                            println!("ignoring file with extension: {}", extension)
                        }
                    }
                }
            }
            Err(err) => {
                print!("error reading dir {}: {}", srcdir.display(), err);
            }
        }
    }

    for src in srcs {
        let mut dest = destdir.to_path_buf();
        // We already validated srcs, so they always have a filename
        dest.push(src.file_name().unwrap());

        let job = TranscodeJob {
            src: src,
            dest: dest,
        };
        job_sender.send(job).expect("failed to send job to workers");
    }

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

fn parse_args() -> Result<(PathBuf, PathBuf), ExitCode> {
    let args: Vec<String> = env::args().collect();

    return match args.len() {
        1 => {
            let src = env::current_dir().unwrap();
            let mut dest = src.clone();
            dest.push("transcoded");
            Ok((src, dest))
        }
        3 => Ok((PathBuf::from(&args[1]), PathBuf::from(&args[2]))),
        _ => {
            println!("usage using current dir: {}", args[0]);
            println!("usage explicit dirs: {} <src dir> <dest dir>", args[0]);
            Err(ExitCode::FAILURE)
        }
    };
}
