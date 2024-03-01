mod tui;

use crossterm::event::{self, poll, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
  prelude::*,
  style::Style,
  symbols::border,
  widgets::{block::Title, Bar, BarChart, BarGroup, Block, Borders, Padding, Paragraph},
};
use std::{
  io::{self, Result},
  time::Duration,
};

#[derive(Debug, Default)]
pub struct App {
  rssi: u8,
  exit: bool,
}

impl App {
  /// runs the application's main loop until the user quits
  pub fn run(&mut self, terminal: &mut tui::Tui) -> io::Result<()> {
    while !self.exit {
      terminal.draw(|frame| self.render_frame(frame))?;
      if poll(Duration::from_millis(100))? {
        self.handle_events()?;
      }
    }
    Ok(())
  }

  fn render_frame(&self, frame: &mut Frame) {
    frame.render_widget(self, frame.size());
  }

  fn handle_events(&mut self) -> io::Result<()> {
    match event::read()? {
      // it's important to check that the event is a key press event as
      // crossterm also emits key release and repeat events on Windows.
      Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
        self.handle_key_event(key_event)
      }
      _ => {}
    };
    Ok(())
  }

  fn handle_key_event(&mut self, key_event: KeyEvent) {
    match key_event.code {
      KeyCode::Char('q') => self.exit(),
      _ => {}
    }
  }

  fn exit(&mut self) {
    self.exit = true;
  }
}

impl Widget for &App {
  fn render(self, area: Rect, buf: &mut Buffer) {
    let title = Title::from(" SA818RSSI ".bold());

    let block = Block::default()
      .title(title.alignment(Alignment::Center))
      .borders(Borders::ALL)
      .border_set(border::THICK);
    let inner = block.clone().padding(Padding::top(1)).inner(area);

    let rssi_text = Text::from(vec![Line::from(vec![
      "Value: ".into(),
      self.rssi.to_string().yellow(),
    ])]);

    Paragraph::new(rssi_text)
      .centered()
      .block(block)
      .render(area, buf);
    BarChart::default()
      .bar_width(3)
      .bar_style(Style::new().yellow().on_red())
      .value_style(Style::new().red().bold())
      .data(BarGroup::default().bars(&[Bar::default().value(self.rssi.into())]))
      .max(255)
      .render(centered_bar(inner, 3, inner.height), buf);
  }
}

fn centered_bar(r: Rect, bar_width: u16, bar_height: u16) -> Rect {
  Rect {
    x: (r.width / 2 - bar_width / 2),
    y: r.y,
    width: bar_width,
    height: bar_height,
  }
}

fn main() -> Result<()> {
  initialize_panic_handler();
  let mut terminal = tui::init()?;
  let app_result = App::default().run(&mut terminal);
  tui::restore()?;
  app_result
}

pub fn initialize_panic_handler() {
  let original_hook = std::panic::take_hook();
  std::panic::set_hook(Box::new(move |panic_info| {
    crossterm::execute!(std::io::stderr(), crossterm::terminal::LeaveAlternateScreen).unwrap();
    crossterm::terminal::disable_raw_mode().unwrap();
    original_hook(panic_info);
  }));
}
