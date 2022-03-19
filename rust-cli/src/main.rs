pub mod widgets;

use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io;
use tui::{
    backend::{Backend, CrosstermBackend},
    Frame, Terminal,
};
use widgets::{ChessboardWidget, Context};

// only to set up and shut down
fn main() -> io::Result<()> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let res = run_app(&mut terminal);

    // restore terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen,)?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

// this is actually the main loop.
fn run_app<B: Backend>(terminal: &mut Terminal<B>) -> io::Result<()> {
    let mut ctx = Context::new();
    loop {
        terminal.draw(|f| ui(f, &mut ctx))?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => return Ok(()),
                KeyCode::Left => ctx.hover_left(),
                KeyCode::Right => ctx.hover_right(),
                KeyCode::Up => ctx.hover_up(),
                KeyCode::Down => ctx.hover_down(),
                KeyCode::Char(' ') | KeyCode::Enter => {
                    if let Some(m) = ctx.click() {
                        ctx.inner_mut().execute_and_cycle(m).unwrap();
                    }
                }
                _ => (),
            }
        }
    }
}

// draw all the things
fn ui<B: Backend>(f: &mut Frame<B>, ctx: &mut Context) {
    let cb = ChessboardWidget;
    f.render_stateful_widget(cb, f.size(), ctx);
}
