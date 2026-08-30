use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, BorderType, Paragraph, Widget},
};

use crate::app::App;

pub struct HomePage<'a> {
    pub app: &'a App,
}

impl Widget for HomePage<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let paragraph = Paragraph::new(format!("Page n°{}", self.app.current_tab + 1))
            .block(Block::bordered().border_type(BorderType::Plain))
            .style(Style::default().fg(Color::Black).bg(Color::White))
            .centered();

        paragraph.render(area, buf);
    }
}
