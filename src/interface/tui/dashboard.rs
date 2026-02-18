use crate::application::app::App;
use crate::domain::entities::RecommendationCategory;
use crate::utils::format_size;
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};

pub fn draw_dashboard(f: &mut Frame, app: &App, area: ratatui::layout::Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Length(3), // Title/Stats
                Constraint::Min(1),    // Recommendations List
            ]
            .as_ref(),
        )
        .split(area);

    // Title / High-level stats
    let total_reclaimable: u64 = app.recommendations.iter().map(|r| r.size).sum();

    let stats_text = vec![Line::from(vec![
        Span::styled(
            " Potential Reclaimable Space: ",
            Style::default().fg(Color::Yellow),
        ),
        Span::styled(
            format_size(total_reclaimable),
            Style::default().add_modifier(Modifier::BOLD).fg(Color::Red),
        ),
    ])];

    let stats_block = Paragraph::new(stats_text)
        .block(Block::default().borders(Borders::ALL).title(" Dashboard "));
    f.render_widget(stats_block, chunks[0]);

    // Recommendations List
    let items: Vec<ListItem> = app
        .recommendations
        .iter()
        .map(|r| {
            let (icon, color) = match r.category {
                RecommendationCategory::Docker => ("🐳", Color::Cyan),
                RecommendationCategory::Log => ("📝", Color::Red),
                RecommendationCategory::Cache => ("⚡", Color::Yellow),
                RecommendationCategory::Trash => ("🗑️", Color::Gray),
                RecommendationCategory::Other => ("📦", Color::White),
            };

            let content = Line::from(vec![
                Span::styled(format!("{} ", icon), Style::default()),
                Span::styled(
                    format!("{:<15}", format!("{:?}", r.category)),
                    Style::default().fg(color).add_modifier(Modifier::BOLD),
                ),
                Span::raw(" | "),
                Span::styled(
                    format!("{:<10} ", format_size(r.size)),
                    Style::default().fg(Color::Green),
                ),
                Span::raw(" | "),
                Span::raw(r.description.clone()),
            ]);

            ListItem::new(content)
        })
        .collect();

    let list = List::new(items)
        .block(
            Block::default()
                .title(" Recommendations (Press [c] to Clean) ")
                .borders(Borders::ALL),
        )
        .highlight_style(
            Style::default()
                .add_modifier(Modifier::BOLD)
                .fg(Color::Blue)
                .bg(Color::DarkGray),
        );

    let mut state = ratatui::widgets::ListState::default();
    state.select(Some(app.recommendation_selection));

    f.render_stateful_widget(list, chunks[1], &mut state);
}
