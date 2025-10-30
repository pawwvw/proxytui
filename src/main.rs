use crossterm::event::{self, Event, KeyCode};
use ratatui::{
    Frame,
    crossterm::style::style,
    style::Style,
    text::Text,
    widgets::{Block, Borders, Paragraph},
};
use std::error::Error;
mod app;
mod profile;
use app::App;

fn main() -> Result<(), Box<dyn Error>> {
    let mut terminal = ratatui::init();
    let mut app = App::new();
    loop {
        terminal.draw(draw)?;
        if let Event::Key(key) = event::read()? {
            app.handle_key(key.code);
        }

        if app.should_quit {
            break;
        }
    }
    ratatui::restore();
    Ok(())
}

fn draw(frame: &mut Frame) {
    let block = Block::default()
        .borders(Borders::ALL)
        .title("proxytui")
        .title_alignment(ratatui::layout::Alignment::Center)
        .style(Style::default().fg(ratatui::style::Color::Green));
    let text = Paragraph::new("Hello proxytui")
        .block(block)
        .alignment(ratatui::layout::Alignment::Center);
    frame.render_widget(text, frame.area());
}
