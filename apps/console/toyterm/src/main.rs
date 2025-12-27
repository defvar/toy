use clap::Parser;
use error::Error;
use ratatui::{
    Terminal,
    backend::{Backend, CrosstermBackend},
    crossterm::{
        event::{self, DisableMouseCapture, EnableMouseCapture, Event, poll},
        execute,
        terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
    },
};
use std::fs::File;
use std::io;
use std::io::Read;
use std::time::Duration;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};
use tokio_util::sync::CancellationToken;
use toy::api_client::http::HttpApiClient;
use toy::api_client::toy_api::authentication::Claims;
use toy_jwt::Algorithm;

mod app;
mod error;
mod opts;
mod state;
mod states;
mod ui;
mod views;

use crate::opts::Opts;
use crate::states::AppActions;
use crate::views::ViewContainer;
use crate::{app::App, ui::ui};

#[tokio::main]
async fn main() -> io::Result<()> {
    dotenv::dotenv().ok();

    let opts: Opts = Opts::parse();

    let token = get_credential(&opts.config.user, &opts.config.kid, &opts.config.credential)
        .map_err(|e| Error::read_credential_error(&opts.config.credential, e));
    if let Err(e) = token {
        eprintln!("{}", e);
        return Err(io::Error::other("credential error"));
    }

    let auth = toy::api_client::Auth::with_bearer_token(&opts.config.user, &token.unwrap());
    let client = HttpApiClient::new(&opts.config.api_root, auth).unwrap();

    enable_raw_mode()?;

    let mut stderr = io::stderr();
    execute!(stderr, EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stderr);
    let mut terminal = Terminal::new(backend)?;
    let (tx, rx) = tokio::sync::mpsc::unbounded_channel();
    let mut app = App::with(tx.clone(), client);
    run_app(&mut terminal, &mut app, rx).await?;

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}

async fn event_loop(tx: UnboundedSender<Event>, token: CancellationToken) -> io::Result<()> {
    loop {
        if token.is_cancelled() {
            return Ok(());
        }
        if poll(Duration::from_millis(250))? {
            let e = event::read()?;
            tx.send(e).unwrap();
        }
    }
}

async fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    app: &mut App,
    mut rx: UnboundedReceiver<AppActions>,
) -> io::Result<()> {
    let mut views = ViewContainer::default();
    let token = CancellationToken::new();
    let (event_tx, mut event_rx) = tokio::sync::mpsc::unbounded_channel::<Event>();

    /* event loop */
    let handle = toy_rt::spawn(event_loop(event_tx.clone(), token.clone()));

    let mut need_draw = true;
    /* render loop */
    loop {
        let page = views.view_mut(app.state().current_view());
        if need_draw {
            terminal.draw(|f| ui(f, app, &mut **page))?;
            need_draw = false;
        }

        if let Ok(Event::Key(key)) = event_rx.try_recv() {
            if key.kind == event::KeyEventKind::Release {
                // Skip events that are not KeyEventKind::Press
                continue;
            }

            page.handle(key, app);
        }

        while let Ok(action) = rx.try_recv() {
            app.update(action);
            need_draw = true;
        }

        if app.should_quit() {
            token.cancel();
            let mut counter = 0;
            while !handle.is_finished() {
                toy_rt::sleep(10).await;
                counter += 1;
                if counter > 50 {
                    handle.abort();
                }
                if counter > 100 {
                    eprintln!("Failed to abort task in 100 milliseconds for unknown reason");
                    break;
                }
            }
            return Ok(());
        }
    }
}

fn get_credential(user: &str, kid: &str, path_string: &str) -> Result<String, Error> {
    let mut f = File::open(path_string)?;
    let mut buffer = Vec::new();
    f.read_to_end(&mut buffer)?;

    let claims = Claims::new(user);

    let token =
        toy_jwt::encode::from_rsa_pem(&claims, Algorithm::RS256, Some(kid.to_owned()), &buffer)?;

    Ok(token)
}
