// cool stuff up top
pub use tui::{
    backend::CrosstermBackend, 
    widgets::{Widget, Block, Borders},
    layout::{Layout, Constraint, Direction},
    Terminal,
};
pub use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    execute
};
// lame stuff down here lol
pub use console::Term;// only using Term because I want to be able to do strikethroughs lol
pub use std::{io, thread, time::Duration};

fn main() {
    enable_raw_mode().ok();
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture).ok();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend).unwrap();

    terminal.draw(|w| {
        let s = w.size();
        let block = Block::default()
              .title("Woah")
              .borders(Borders::ALL);
        w.render_widget(block, s);
    }).ok();
    disable_raw_mode().ok();

    // exit
    execute!(
        &mut terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    ).expect("Error! {}");
}
