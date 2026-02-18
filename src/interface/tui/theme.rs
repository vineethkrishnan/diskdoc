use ratatui::style::Color;

#[allow(dead_code)]
pub struct Theme {
    pub primary: Color,
    pub secondary: Color,
    pub danger: Color,
}

impl Default for Theme {
    fn default() -> Self {
        Theme {
            primary: Color::Cyan,
            secondary: Color::Yellow,
            danger: Color::Red,
        }
    }
}
