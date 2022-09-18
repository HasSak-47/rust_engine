mod lily;

use std::{io, time::Duration};
use tui::{
    backend::CrosstermBackend,
    // widgets::{Widget, Block, Borders, Paragraph, canvas},
    // layout:: {Layout, Constraint, Direction, Rect},
    Terminal,
};

use crossterm::{
    self,
    execute,
    event::{DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

use crate::lily::{
    function_drawer::RelationDrawer,
    generator::bubble
};

fn main() -> Result<(), io::Error>{
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut look_ahead = ['\0', '\0', '\0', '\0', '\0'];
    let mut x_offset = 0.0;
    let mut y_offset = 0.0;
    let mut drawer = RelationDrawer{
        beg_x: -4.0,
        end_x:  4.0,
        beg_y: -2.0,
        end_y:  2.0,

        dist : 0.9,
        function: bubble::bubble_gen_2d, 
    };
    loop{
        let mut input: char = ' ';
        let mut debug_escape: bool = false;
        let mut x_off = 0.0;
        let mut y_off = 0.0;

        //rendering part
        terminal.draw(|f| {
            let border = f.size();
            x_off = 1.0 / border.width as f64;
            y_off = 1.0 / border.height as f64;

            f.render_widget(&drawer, border);
        })?;

        //read inputs
        crossterm::event::poll(Duration::from_nanos(1))?;
        match crossterm::event::read()?{
            Event::FocusGained    => {},
            Event::FocusLost      => {},
            Event::Key(event_key) => {
                match event_key.code{
                    KeyCode::Esc => debug_escape = true,
                    KeyCode::Char(chr) => input = chr,
                    KeyCode::Up    => {  drawer.end_y += y_off; drawer.beg_y += y_off;},
                    KeyCode::Down  => {  drawer.end_y -= y_off; drawer.beg_y -= y_off;},
                    KeyCode::Left  => {  drawer.end_x -= x_off; drawer.beg_x -= x_off;},
                    KeyCode::Right => {  drawer.end_x += x_off; drawer.beg_x += x_off;},
                    _ => {},
                }
            },
            Event::Mouse(_)       => {},
            Event::Paste(_)       => {},
            Event::Resize(_, _)   => {},
        }
        
        if debug_escape {
            break;
        }

        look_ahead[0] = look_ahead[1];
        look_ahead[1] = look_ahead[2];
        look_ahead[2] = look_ahead[3];
        look_ahead[3] = look_ahead[4];
        look_ahead[4] = input;
    }
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    println!("{} {} {} {} {}\n", 
        look_ahead[0],
        look_ahead[1],
        look_ahead[2],
        look_ahead[3],
        look_ahead[4],
    );
    println!("\n\n\neverything okay\n");
    Ok(())
}
