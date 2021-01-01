use crate::{TailContext, TailError};
use notify::event::{ModifyKind, RenameMode};
use notify::{Event, EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use std::path::Path;

pub async fn watch<P: AsRef<Path>>(
    path: P,
    prefix: &str,
    ctx: &mut TailContext,
) -> Result<(), TailError> {
    let names = ctx.handler_names().await;
    tracing::info!("enable handler:{:?}", names);

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
                    for p in paths
                        .iter()
                        .filter(|x| x.to_str().unwrap_or("").starts_with(path_prefix_str))
                    {
                        tracing::info!("new reader. path={}", p.to_str().unwrap_or(""));
                        ctx.follow(p, false)?;
                        ctx.read(&p).await?;
                    }
                }
                Event {
                    kind: EventKind::Modify(ModifyKind::Data(_)),
                    paths,
                    ..
                } => {
                    for p in paths
                        .iter()
                        .filter(|x| x.to_str().unwrap_or("").starts_with(path_prefix_str))
                    {
                        if ctx.is_reading(&p) {
                            ctx.read(&p).await?;
                        } else {
                            ctx.follow(&p, true)?;
                            ctx.read(&p).await?;
                        }
                    }
                }
                Event {
                    kind: EventKind::Modify(ModifyKind::Name(RenameMode::To)),
                    paths,
                    ..
                } => {
                    for p in paths
                        .iter()
                        .filter(|x| x.to_str().unwrap_or("").starts_with(path_prefix_str))
                    {
                        tracing::info!("file rename or move? new reader.");
                        ctx.follow(&p, false)?;
                        ctx.read(&p).await?;
                    }
                }
                Event {
                    kind: EventKind::Remove(_),
                    paths,
                    ..
                } => {
                    for p in paths
                        .iter()
                        .filter(|x| x.to_str().unwrap_or("").starts_with(path_prefix_str))
                    {
                        tracing::info!("file remove?");
                        ctx.remove(p);
                    }
                }
                _ => (),
            },
            Err(e) => tracing::error!("watch error: {:?}", e),
        }
    }

    Ok(())
}
