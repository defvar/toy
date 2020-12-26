use crate::{Handler, TailContext, TailError};
use notify::event::{ModifyKind, RenameMode};
use notify::{Event, EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use std::path::{Path, PathBuf};

pub fn watch<P: AsRef<Path>, T: Handler>(
    path: P,
    prefix: &str,
    ctx: &mut TailContext<T>,
) -> Result<(), TailError> {
    let (tx, rx) = std::sync::mpsc::channel();

    let mut watcher: RecommendedWatcher = Watcher::new_immediate(move |res| tx.send(res).unwrap())?;
    let path_prefix = path.as_ref().join(prefix);
    let path_prefix_str = path_prefix.to_str().unwrap();
    watcher.watch(path.as_ref(), RecursiveMode::Recursive)?;

    for res in rx {
        match res {
            Ok(event) => match event {
                Event {
                    kind: EventKind::Create(_),
                    paths,
                    ..
                } => {
                    on_event(path_prefix_str, paths, ctx, |p, ctx| {
                        println!("new reader. path={}", p.to_str().unwrap_or(""));
                        ctx.follow(p, false)?;
                        ctx.read(&p)?;
                        Ok(())
                    })?;
                }
                Event {
                    kind: EventKind::Modify(ModifyKind::Data(_)),
                    paths,
                    ..
                } => {
                    on_event(path_prefix_str, paths, ctx, |p, ctx| {
                        if ctx.is_reading(&p) {
                            ctx.read(&p)?;
                        } else {
                            ctx.follow(&p, true)?;
                            ctx.read(&p)?;
                        }
                        Ok(())
                    })?;
                }
                Event {
                    kind: EventKind::Modify(ModifyKind::Name(RenameMode::To)),
                    paths,
                    ..
                } => {
                    on_event(path_prefix_str, paths, ctx, |p, ctx| {
                        println!("file rename or move? new reader.");
                        ctx.follow(&p, false)?;
                        ctx.read(&p)?;
                        Ok(())
                    })?;
                }
                Event {
                    kind: EventKind::Remove(_),
                    paths,
                    ..
                } => {
                    on_event(path_prefix_str, paths, ctx, |p, ctx| {
                        println!("file remove?");
                        ctx.remove(p);
                        Ok(())
                    })?;
                }
                _ => (),
            },
            Err(e) => println!("watch error: {:?}", e),
        }
    }

    Ok(())
}

fn on_event<F, H>(
    path_prefix: &str,
    paths: Vec<PathBuf>,
    ctx: &mut TailContext<H>,
    f: F,
) -> Result<(), TailError>
where
    F: Fn(&Path, &mut TailContext<H>) -> Result<(), TailError>,
{
    for p in paths
        .iter()
        .filter(|x| x.to_str().unwrap_or("").starts_with(path_prefix))
    {
        f(p, ctx)?;
    }
    Ok(())
}
