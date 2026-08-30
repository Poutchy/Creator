use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    symbols,
    widgets::{Block, BorderType, Tabs, Widget},
};

use crate::app::App;

pub struct Header<'a> {
    pub app: &'a App,
}

impl Widget for Header<'_> {
    /// Renders the user interface widgets.
    ///
    // This is where you add new widgets.
    // See the following resources:
    // - https://docs.rs/ratatui/latest/ratatui/widgets/index.html
    // - https://github.com/ratatui/ratatui/tree/master/examples
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = Block::bordered()
            .border_type(BorderType::Plain);

        let tabs = Tabs::new(vec!["Tab1", "Tab2", "Tab3"])
            .block(block)
            .style(Style::default().black().bg(Color::White))
            .highlight_style(Style::default().blue().bg(Color::White))
            .select(self.app.current_tab)
            .divider(symbols::DOT)
            .padding("->", "<-");

        tabs.render(area, buf);
    }
}
