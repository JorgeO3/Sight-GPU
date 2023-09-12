#![allow(unused)]

use ratatui::{prelude::*, widgets::*};
use std::{error::Error, io};

#[derive(Debug, Default)]
pub struct ProgressBar<'a> {
    block: Option<Block<'a>>,
    percentaje: f64,
    label: Option<Span<'a>>,
    style: Style,
    use_unicode: bool,
}

impl<'a> ProgressBar<'a> {
    pub fn block(mut self, block: Block<'a>) -> Self {
        self.block = Some(block);
        self
    }

    pub fn percent(mut self, percent: u16) -> Self {
        self.percentaje = percent as f64 / 100.0;
        self
    }

    pub fn label<T>(mut self, label: T) -> Self
    where
        T: Into<Span<'a>>,
    {
        self.label = Some(label.into());
        self
    }

    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    pub fn use_unicode(mut self, unicode: bool) -> Self {
        self.use_unicode = unicode;
        self
    }
}

impl<'a> Widget for ProgressBar<'a> {
    fn render(mut self, area: Rect, buf: &mut Buffer) {
        buf.set_style(area, self.style);
        let pb_area = match self.block.take() {
            Some(b) => {
                let inner_area = b.inner(area);
                b.render(area, buf);
                inner_area
            }
            None => area,
        };

        let label = {
            let pct = f64::round(self.percentaje * 100.0);
            self.label.unwrap_or_else(|| Span::from(format!("{pct}%")))
        };

        if pb_area.height < 1 {};
        let zero_y = pb_area.top();
        let first_line = pb_area.top() + 1;

        let clamped_label_width = pb_area.width.min(label.width() as u16);
        let label_col = pb_area.left() + (pb_area.width - clamped_label_width) / 2;
        let label_row = pb_area.top() + pb_area.height / 2;

        for x in pb_area.left()..pb_area.right() {
            let cell = buf.get_mut(x, first_line);
            cell.set_symbol("|")
                .set_fg(Style::default().fg.unwrap_or(Color::Reset))
                .set_bg(Style::default().bg.unwrap_or(Color::Reset));
        }

        buf.set_span(label_col, label_row, &label, clamped_label_width);
    }
}

fn get_unicode_block<'a>(frac: f64) -> &'a str {
    match (frac * 8.0).round() as u16 {
        1 => symbols::block::ONE_EIGHTH,
        2 => symbols::block::ONE_QUARTER,
        3 => symbols::block::THREE_EIGHTHS,
        4 => symbols::block::HALF,
        5 => symbols::block::FIVE_EIGHTHS,
        6 => symbols::block::THREE_QUARTERS,
        7 => symbols::block::SEVEN_EIGHTHS,
        8 => symbols::block::FULL,
        _ => " ",
    }
}
