use crossterm::{
  event::{self, KeyCode, KeyEventKind},
  terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
  ExecutableCommand,
};
use ratatui::{
  prelude::*,
  style::Style,
  widgets::{block, Bar, BarChart, BarGroup, Block, Borders, Padding, Paragraph},
};
use std::io::{stdout, Result};
fn centered_rect(r: Rect, percent_x: u16, percent_y: u16) -> Rect {
  let popup_layout = Layout::default()
    .direction(Direction::Vertical)
    .constraints([
      Constraint::Percentage((100 - percent_y) / 2),
      Constraint::Percentage(percent_y),
      Constraint::Percentage((100 - percent_y) / 2),
    ])
    .split(r);

  Layout::default()
    .direction(Direction::Horizontal)
    .constraints([
      Constraint::Percentage((100 - percent_x) / 2),
      Constraint::Percentage(percent_x),
      Constraint::Percentage((100 - percent_x) / 2),
    ])
    .split(popup_layout[1])[1]
}
fn main() -> Result<()> {
  stdout().execute(EnterAlternateScreen)?;
  enable_raw_mode()?;
  let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
  terminal.clear()?;

  loop {
    terminal.draw(|frame| {
      let bar = BarChart::default()
        .bar_width(6)
        .bar_gap(1)
        .group_gap(6)
        .bar_style(Style::new().green().on_blue())
        .value_style(Style::new().white())
        .label_style(Style::new().white())
        .data(&[("RSSI", 127)])
        .max(255);
      let area = frame.size();
      let b = Block::default()
        .title(block::Title::from("SA818 RSSI").alignment(Alignment::Center))
        .borders(Borders::ALL)
        .padding(Padding::left((area.width - 6) / 2));
      let inner = b.inner(area);
      frame.render_widget(b, area);
      frame.render_widget(bar, inner);
    })?;
    if event::poll(std::time::Duration::from_millis(16))? {
      if let event::Event::Key(key) = event::read()? {
        if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q')
          || key.code == KeyCode::Char('Q')
        {
          break;
        }
      }
    }
  }
  stdout().execute(LeaveAlternateScreen)?;
  disable_raw_mode()?;
  Ok(())
}
