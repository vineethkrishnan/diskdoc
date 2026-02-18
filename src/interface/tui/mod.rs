use crate::application::app::{App, AppMode};
use crate::domain::entities::FileType;
use crate::utils::format_size;
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Gauge, List, ListItem, Paragraph, Wrap},
    Frame,
};

pub mod dashboard;
pub mod theme;
pub mod widgets;

pub fn draw(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(
            [
                Constraint::Length(3), // Header
                Constraint::Min(0),    // Content
                Constraint::Length(3), // Footer/Status
            ]
            .as_ref(),
        )
        .split(f.size());

    // 1. Header
    let title = format!(" DiskDoctor - {} ", app.current_path.display());
    let header = Block::default()
        .title(title)
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::Cyan));
    f.render_widget(header, chunks[0]);

    // 2. Content
    match app.mode {
        AppMode::Scanning => draw_scanning(f, app, chunks[1]),
        AppMode::Browsing => draw_browsing(f, app, chunks[1]),
        AppMode::Dashboard => dashboard::draw_dashboard(f, app, chunks[1]),
        AppMode::About => draw_about(f, chunks[1]),
        AppMode::DeleteConfirmation => {
            draw_browsing(f, app, chunks[1]); // Draw background
            draw_delete_popup(f, app, f.size()); // Draw popup over full screen
        }
        AppMode::DashboardCleanupConfirmation => {
            dashboard::draw_dashboard(f, app, chunks[1]);
            draw_dashboard_cleanup_popup(f, app, f.size());
        }
    }

    // 3. Footer
    let status_text = format!(
        " [1] Dashboard | [2] Files | Total: {} | Files: {} | [s] Sort | [d] Delete | [q] Quit ",
        format_size(app.total_size),
        app.scanned_count
    );
    let footer = Paragraph::new(status_text).block(Block::default().borders(Borders::ALL));
    f.render_widget(footer, chunks[2]);
}

fn draw_scanning(f: &mut Frame, app: &App, area: ratatui::layout::Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(1)].as_ref())
        .split(area);

    let gauge = Gauge::default()
        .block(Block::default().title("Scanning...").borders(Borders::ALL))
        .gauge_style(Style::default().fg(Color::Yellow))
        .percent(0); // Indeterminate or based on expected count?

    f.render_widget(gauge, chunks[0]);

    let info = Paragraph::new(format!("Found {} files...", app.scanned_count))
        .block(Block::default().borders(Borders::NONE));
    f.render_widget(info, chunks[1]);
}

fn draw_browsing(f: &mut Frame, app: &App, area: ratatui::layout::Rect) {
    // For now, just list the top largest files/dirs?
    // Or a simple list

    // List only files in current directory
    let current_files = app.get_current_files();

    let items: Vec<ListItem> = current_files
        .iter()
        .take(100)
        .map(|f| {
            let name = f.path.file_name().unwrap_or_default().to_string_lossy();

            let style = if f.is_dir {
                Style::default()
                    .add_modifier(Modifier::BOLD)
                    .fg(Color::Blue)
            } else {
                match f.file_type {
                    FileType::Log => Style::default().fg(Color::Red),
                    FileType::Cache => Style::default().fg(Color::Yellow),
                    FileType::NpmCache | FileType::ComposerCache | FileType::AptCache => {
                        Style::default().fg(Color::Yellow)
                    }
                    FileType::Docker => Style::default().fg(Color::Cyan),
                    FileType::Normal => Style::default(),
                }
            };

            let name_styled = if f.is_dir {
                Span::styled(format!("{}/", name), style)
            } else {
                Span::styled(name.to_string(), style)
            };

            let content = Line::from(vec![
                Span::styled(
                    format!("{:<10} ", format_size(f.size)),
                    Style::default().fg(Color::Yellow),
                ),
                name_styled,
            ]);
            ListItem::new(content)
        })
        .collect();

    let list = List::new(items)
        .block(Block::default().title("Files").borders(Borders::ALL))
        .highlight_style(
            Style::default()
                .add_modifier(Modifier::BOLD)
                .fg(Color::Green),
        ); // We need to render with state

    let mut state = ratatui::widgets::ListState::default();
    state.select(Some(app.selection));

    f.render_stateful_widget(list, area, &mut state);
}

fn draw_about(f: &mut Frame, area: ratatui::layout::Rect) {
    let text = vec![
        Line::from(Span::styled(
            "DiskDoctor (dru)",
            Style::default()
                .add_modifier(Modifier::BOLD)
                .fg(Color::Cyan),
        )),
        Line::from(""),
        Line::from(vec![
            Span::raw("Built by "),
            Span::styled("@vineethkrishnan", Style::default().fg(Color::Green)),
        ]),
        Line::from("https://github.com/vineethkrishnan"),
        Line::from(""),
        Line::from(Span::styled(
            "And YOU! (Future Contributor)",
            Style::default()
                .fg(Color::Magenta)
                .add_modifier(Modifier::ITALIC),
        )),
        Line::from(""),
        Line::from("Press [Esc] or [q] to return"),
    ];

    let paragraph = Paragraph::new(text)
        .block(Block::default().title("About").borders(Borders::ALL))
        .alignment(ratatui::layout::Alignment::Center)
        .wrap(Wrap { trim: true });

    // Center the box vertically and horizontally roughly
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage(20),
                Constraint::Percentage(60),
                Constraint::Percentage(20),
            ]
            .as_ref(),
        )
        .split(area);

    let h_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage(20),
                Constraint::Percentage(60),
                Constraint::Percentage(20),
            ]
            .as_ref(),
        )
        .split(layout[1]);

    f.render_widget(paragraph, h_layout[1]);
}

fn draw_delete_popup(f: &mut Frame, app: &App, area: ratatui::layout::Rect) {
    if let Some(path) = &app.item_to_delete {
        let block = Block::default()
            .title(" Confirm Deletion ")
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::Red).add_modifier(Modifier::BOLD));

        let area = centered_rect(60, 25, area);

        let text = vec![
            Line::from("Are you sure you want to delete this item?"),
            Line::from(""),
            Line::from(Span::styled(
                path.to_string_lossy(),
                Style::default().fg(Color::Yellow),
            )),
            Line::from(""),
            Line::from(vec![
                Span::raw("Press "),
                Span::styled("[y]", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" or "),
                Span::styled("[Enter]", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to Confirm"),
            ]),
            Line::from(vec![
                Span::raw("Press "),
                Span::styled("[n]", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" or "),
                Span::styled("[Esc]", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to Cancel"),
            ]),
        ];

        let paragraph = Paragraph::new(text)
            .block(block)
            .alignment(ratatui::layout::Alignment::Center)
            .wrap(Wrap { trim: true });

        f.render_widget(Clear, area); // Clear background
        f.render_widget(paragraph, area);
    }
}

fn draw_dashboard_cleanup_popup(f: &mut Frame, app: &App, area: ratatui::layout::Rect) {
    if let Some(rec) = app.recommendations.get(app.recommendation_selection) {
        let block = Block::default()
            .title(" Confirm Cleanup ")
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::Red).add_modifier(Modifier::BOLD));

        let area = centered_rect(60, 25, area);

        let text = vec![
            Line::from("Are you sure you want to clean this category?"),
            Line::from(""),
            Line::from(Span::styled(
                format!("{:?}", rec.category),
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            )),
            Line::from(Span::raw(format!("Size: {}", format_size(rec.size)))),
            Line::from(""),
            Line::from(vec![
                Span::raw("This will "),
                Span::styled(
                    "PERMANENTLY DELETE",
                    Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
                ),
                Span::raw(" all items in this category."),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::raw("Press "),
                Span::styled("[y]", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" or "),
                Span::styled("[Enter]", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to Confirm"),
            ]),
            Line::from(vec![
                Span::raw("Press "),
                Span::styled("[n]", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" or "),
                Span::styled("[Esc]", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to Cancel"),
            ]),
        ];

        let paragraph = Paragraph::new(text)
            .block(block)
            .alignment(ratatui::layout::Alignment::Center)
            .wrap(Wrap { trim: true });

        f.render_widget(Clear, area); // Clear background
        f.render_widget(paragraph, area);
    }
}

fn centered_rect(
    percent_x: u16,
    percent_y: u16,
    r: ratatui::layout::Rect,
) -> ratatui::layout::Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage((100 - percent_y) / 2),
                Constraint::Percentage(percent_y),
                Constraint::Percentage((100 - percent_y) / 2),
            ]
            .as_ref(),
        )
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage((100 - percent_x) / 2),
                Constraint::Percentage(percent_x),
                Constraint::Percentage((100 - percent_x) / 2),
            ]
            .as_ref(),
        )
        .split(popup_layout[1])[1]
}
