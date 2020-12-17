use notify::event::{ModifyKind, RenameMode};
use notify::{Event, EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use std::fs::File;
use std::io::{BufRead, BufReader, Seek, SeekFrom};
use std::path::Path;

mod tail;

struct TailContext {
    position: u64,
}

fn get_reader<P: AsRef<Path>>(
    path: P,
    tail: bool,
    ctx: &TailContext,
) -> Result<BufReader<File>, std::io::Error> {
    let mut file = File::open(&path)?;
    let seek = if tail {
        SeekFrom::End(0)
    } else {
        SeekFrom::Start(ctx.position)
    };
    file.seek(seek)?;
    Ok(BufReader::new(file))
}

fn read<T: BufRead>(
    reader: &mut Result<T, std::io::Error>,
    ctx: &mut TailContext,
) -> std::io::Result<()> {
    match reader {
        Ok(ref mut r) => {
            let buf = r.fill_buf()?;
            let size = buf.len();
            if size > 0 {
                println!("read: {:?}", size);
                println!("{:?}", std::str::from_utf8(buf).map_err(|_| "er").unwrap());
                r.consume(size);
                ctx.position += size as u64;
            }
            Ok(())
        }
        Err(_) => {
            println!("no reader...");
            Ok(())
        }
    }
}

fn watch<P: AsRef<Path>>(path: P, file_name: &str, ctx: &mut TailContext) -> notify::Result<()> {
    let (tx, rx) = std::sync::mpsc::channel();

    // Automatically select the best implementation for your platform.
    // You can also access each implementation directly e.g. INotifyWatcher.
    let mut watcher: RecommendedWatcher = Watcher::new_immediate(move |res| tx.send(res).unwrap())?;

    // Add a path to be watched. All files and directories at that path and
    // below will be monitored for changes.
    watcher.watch(path.as_ref(), RecursiveMode::Recursive)?;

    let full_path = path.as_ref().join(file_name);
    let mut reader = get_reader(&full_path, true, ctx);

    for res in rx {
        match res {
            Ok(event) => match event {
                Event {
                    kind: EventKind::Create(_),
                    paths,
                    ..
                } if paths.contains(&full_path) => {
                    println!("new reader.");
                    reader = get_reader(&full_path, false, ctx);
                    read(&mut reader, ctx)?;
                }
                Event {
                    kind: EventKind::Modify(ModifyKind::Data(_)),
                    paths,
                    ..
                } if paths.contains(&full_path) => {
                    read(&mut reader, ctx)?;
                }
                Event {
                    kind: EventKind::Modify(ModifyKind::Name(RenameMode::To)),
                    paths,
                    ..
                } if paths.contains(&full_path) => {
                    println!("file rename or move?");
                    println!("new reader.");
                    reader = get_reader(&full_path, false, ctx);
                    read(&mut reader, ctx)?;
                }
                Event {
                    kind: EventKind::Remove(_),
                    paths,
                    ..
                } if paths.contains(&full_path) => {
                    println!("file remove?");
                }
                _ => (),
            },
            Err(e) => println!("watch error: {:?}", e),
        }
    }

    Ok(())
}

fn main() {
    let path = "/tmp/toy";
    println!("watching {}", path);
    let mut ctx = TailContext { position: 0 };
    match watch(path, "hello.example.log.2020-12-08-14", &mut ctx) {
        Ok(_) => {
            println!("watch end.");
        }
        Err(e) => {
            println!("error: {:?}", e);
        }
    }
}
