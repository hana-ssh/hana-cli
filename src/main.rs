use color_eyre::Result;
use crossterm::event::{self, Event};
use ratatui::layout::Alignment;
use ratatui::{DefaultTerminal, Frame};
use ratatui::widgets::Block;

fn main() -> Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let result = run(terminal);
    ratatui::restore();
    result
}

fn run(mut terminal: DefaultTerminal) -> Result<()> {
    loop {
        terminal.draw(render)?;
        if matches!(event::read()?, Event::Key(_)) {
            break Ok(());
        }
    }
}

fn render(frame: &mut Frame) {
    let block = Block::default().title("Hana — Connected to: root@192.168.0.167").borders(ratatui::widgets::Borders::ALL).title_alignment(Alignment::Center).border_style(ratatui::style::Style::default().fg(ratatui::style::Color::DarkGray));
    frame.render_widget(block, frame.area());
}