use crate::utils::either::Either;
use std::borrow::Cow;
use tui::{
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, BorderType, Borders, Paragraph},
};

pub fn coloured_span<'a>(st: impl Into<Cow<'a, str>>, fg: Color) -> Span<'a> {
    Span::styled(st, Style::default().fg(fg))
}

pub fn default_block<'a>() -> Block<'a> {
    Block::default()
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::White))
        .border_type(BorderType::Plain)
}

pub fn para_from_strings<'a>(texts: Vec<Either<Cow<'a, str>, Span<'a>>>) -> Paragraph<'a> {
    let mut para_body = vec![];
    for txt in texts {
        let span = match txt {
            Either::Left(st) => Span::raw(st),
            Either::Right(sp) => sp,
        };
        para_body.push(Spans::from(span));
    }
    Paragraph::new(para_body)
}

pub fn default_style() -> Style {
    Style::default()
        .add_modifier(Modifier::BOLD)
        .bg(Color::Yellow)
        .fg(Color::Black)
}
