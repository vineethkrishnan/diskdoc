use clap::Parser;
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use diskdoc::application::app::{App, AppMode};
use diskdoc::domain::ports::Scanner; // To use scan method
use diskdoc::infrastructure::cleaner::FsCleaner;
use diskdoc::infrastructure::docker::DockerAnalyzerImpl;
use diskdoc::infrastructure::scanner::FsScanner;
use diskdoc::interface::tui;
use ratatui::{backend::CrosstermBackend, Terminal};
use std::{io, time::Duration};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to start scanning from
    #[arg(default_value = ".")]
    path: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Setup CLI args
    let args = Args::parse();

    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create Infrastructure Adapters
    let cleaner = Box::new(FsCleaner::new());
    let analyzer = Box::new(DockerAnalyzerImpl::new());
    let scanner = FsScanner::new();

    // Create app with dependencies
    let mut app = App::new(args.path.clone(), cleaner, analyzer);

    // Start scanner
    let (tx, rx) = std::sync::mpsc::channel();
    app.scan_receiver = Some(rx);

    let path = std::path::PathBuf::from(&args.path);
    // Use scanner trait method or direct?
    // Using trait method is better but FsScanner::scan takes &self.
    // We can just call it direct or via trait.
    scanner.scan(&path, tx);

    // Run app loop
    let res = run_app(&mut terminal, &mut app).await;

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err);
    }

    Ok(())
}

async fn run_app<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    app: &mut App,
) -> io::Result<()> {
    loop {
        terminal.draw(|f| tui::draw(f, app))?;

        if crossterm::event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = crossterm::event::read()? {
                match app.mode {
                    AppMode::DeleteConfirmation => match key.code {
                        KeyCode::Char('y') | KeyCode::Enter => app.confirm_delete(),
                        KeyCode::Char('n') | KeyCode::Char('q') | KeyCode::Esc => {
                            app.cancel_delete()
                        }
                        _ => {}
                    },
                    AppMode::DashboardCleanupConfirmation => match key.code {
                        KeyCode::Char('y') | KeyCode::Enter => app.confirm_clean_recommendation(),
                        KeyCode::Char('n') | KeyCode::Char('q') | KeyCode::Esc => {
                            app.cancel_clean()
                        }
                        _ => {}
                    },
                    AppMode::About => {
                        if key.code == KeyCode::Esc || key.code == KeyCode::Char('q') {
                            app.mode = AppMode::Browsing;
                        }
                    }
                    _ => {
                        if key.code == KeyCode::Char('q') {
                            return Ok(());
                        }
                        if key.code == KeyCode::Char('?') {
                            app.mode = AppMode::About;
                        }
                        if key.code == KeyCode::Char('1') {
                            app.scan_dashboard();
                            app.mode = AppMode::Dashboard;
                        }
                        if key.code == KeyCode::Char('2') {
                            app.mode = AppMode::Browsing;
                        }

                        if app.mode == AppMode::Dashboard {
                            if key.code == KeyCode::Esc || key.code == KeyCode::Char('q') {
                                app.mode = AppMode::Browsing;
                            }
                            match key.code {
                                KeyCode::Down | KeyCode::Char('j') => app.dashboard_next(),
                                KeyCode::Up | KeyCode::Char('k') => app.dashboard_prev(),
                                KeyCode::Char('c') | KeyCode::Enter => {
                                    app.request_clean_recommendation()
                                }
                                _ => {}
                            }
                            return Ok(());
                        }

                        if key.code == KeyCode::Char('s') {
                            app.toggle_sort();
                        }
                        match key.code {
                            KeyCode::Enter | KeyCode::Right => app.enter_dir(),
                            KeyCode::Backspace | KeyCode::Left => app.go_up(),
                            KeyCode::Down | KeyCode::Char('j') => app.date_next(),
                            KeyCode::Up | KeyCode::Char('k') => app.date_prev(),
                            KeyCode::Char('d') => app.request_delete(),
                            _ => {}
                        }
                    }
                }
            }
        }

        app.on_tick();
    }
}
