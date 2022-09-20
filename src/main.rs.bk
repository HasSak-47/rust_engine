mod lily;
use crate::lily::function_drawer;

use std::{io, thread, time::Duration};
use tui::{
    backend::CrosstermBackend,
    widgets::{Widget, Block, Borders, Paragraph, canvas},
    layout:: {Layout, Constraint, Direction, Rect},
    Terminal
}; 

use crossterm::{
    self,
    execute,
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEvent, read},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

fn drawer(x : &f32) -> f32{
    return x.clone();
}

fn fake_main() -> Result<(), io::Error>{

    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    crossterm::cursor::MoveTo(0, 0);
    // ended terminal setup

    loop{
        crossterm::cursor::MoveTo(0, 0);
        let mut input : char = 0 as char;
        let mut debug_escape : bool = false;

        // start rendering part 
        terminal.draw(|f| {
            let widget_border = f.size();
            let drawer = function_drawer::FunctionDrawer{
                beg_x : -1.0,
                beg_y : -0.5,
                end_x :  1.0,
                end_y :  0.5,

                function: drawer
            };
            //let graph_border = Block::default().title("graph").borders(Borders::ALL);
            //f.render_widget(graph_border, widget_border);
            f.render_widget(drawer, widget_border);
        })?;
        match crossterm::event::poll(Duration::from_millis(1)) {
            Ok(polled) => if !polled {continue;},
            Err(_) => break,
        }

        // read and setup of the app
        match crossterm::event::read()? {
            Event::FocusGained    => {},
            Event::FocusLost      => {},
            Event::Key(event_key) => {
                match event_key.code {
                    KeyCode::Esc =>
                        debug_escape = true,
                    KeyCode::Char(chr) => {
                        input = chr;
                    },
                    _ => {}
                }
            },
            Event::Mouse(_) => {},
            Event::Paste(_)     => {},
            Event::Resize(_, _) => {}
        }
        
        if debug_escape {
            break;
        }

    }

    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;

    Ok(())
}

fn main() {
    match fake_main() {
        Ok(_) => print!("\n\n\neverything ok!\n"),
        Err(err) => print!("{}", err)
    }

}
