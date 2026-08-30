use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    widgets::Widget,
};

use crate::app::App;

pub mod header;
pub mod pages {
    pub mod home;
}

use header::Header;
use pages::home::HomePage;

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let [header, content] =
            Layout::vertical([Constraint::Length(3), Constraint::Min(0)]).areas(area);

        let current_tab = self.current_tab;

        Header { app: self }.render(header, buf);

        match current_tab {
            1 => HomePage { app: self }.render(content, buf),
            2 => HomePage { app: self }.render(content, buf),
            3 => HomePage { app: self }.render(content, buf),
            _ => HomePage { app: self }.render(content, buf),
        }
    }
}
