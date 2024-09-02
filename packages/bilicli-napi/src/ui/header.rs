use ratatui::{
    prelude::*,
    widgets::{Block, Padding, Paragraph},
};

use crate::TuiState;

#[derive(Default, Debug)]
pub struct Header;

impl StatefulWidget for &Header {
    type State = TuiState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let block = Block::bordered()
            .border_type(ratatui::widgets::BorderType::Rounded)
            .padding(Padding::horizontal(1));

        Paragraph::new(
            Line::from(vec![
                Span::from(format!("{}", state.room_id)).fg(Color::Green),
                Span::raw(" "),
                Span::from(format!("({}·{})", state.area_name, state.parent_area_name))
                    .fg(Color::LightGreen),
                Span::raw(" "),
                Span::from(state.title.clone()),
            ])
            .bold(),
        )
        .block(block)
        .render(area, buf);
    }
}
