// TODO: move this to rek-tui library
use std::io::{self, BufRead, stdout};
use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{prelude::*, widgets::*};

fn ui(frame: &mut Frame) {
    let main_layout = Layout::new(
        Direction::Vertical,
        [
            Constraint::Length(1),
            Constraint::Min(0),
            Constraint::Length(1),
        ],
    )
    .split(frame.size());
    frame.render_widget(
        Block::new().borders(Borders::TOP).title("rekord"),
        main_layout[0],
    );
    frame.render_widget(
        Block::new().borders(Borders::TOP).title("Status Bar"),
        main_layout[2],
    );

    let inner_layout = Layout::new(
        Direction::Horizontal,
        [Constraint::Percentage(33), Constraint::Percentage(33), Constraint::Percentage(34)]
    )
    .split(main_layout[1]);
    frame.render_widget(Block::bordered().title("One-Shots"), inner_layout[0]);
    frame.render_widget(Block::bordered().title("Loops"), inner_layout[1]);
    frame.render_widget(Block::bordered().title("Stems"), inner_layout[2]);
}

fn main() -> io::Result<()> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

    let mut should_quit = false;
    while !should_quit {
        terminal.draw(ui)?;
        should_quit = handle_q()?;
    }

    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}

fn handle_q() -> io::Result<bool> {
    if event::poll(std::time::Duration::from_millis(50))? {
        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Press && key.code == KeyCode::Char('q') {
                return Ok(true);
            }
        }
    }
    Ok(false)

    // fn ui(frame: &mut Frame) {
    //     frame.render_widget(
    //         Paragraph::new("Hello World!").block(Block::bordered().title("Greeting")),
    //         frame.size(),
    //     )
}
