mod lily;

use std::{io, time::{
    Duration,
    Instant,
}};
use tui::{
    backend::CrosstermBackend,
    widgets::Paragraph,
    // widgets::{Widget, Block, Borders, Paragraph, canvas},
    // layout:: {Layout, Constraint, Direction, Rect},
    layout::Rect,
    Terminal,
};

use crossterm::{
    self,
    execute,
    event::{DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

use crate::lily::{
    drawers::{
        relation_drawer::{self, RelationDrawer},
    },
    generator::bubble,
};

fn main() -> Result<(), io::Error>{
    relation_drawer::setup();
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut look_ahead = ['\0'; 5];
    let mut drawer = RelationDrawer{
        x_off:  0.0,
        y_off:  0.0,
        x_zm :  2.0,
        y_zm :  1.0,

        dist : 0.9,
        function: bubble::bubble_gen_2d, 
    };
    let mut input: char = ' ';
    let mut debug_escape: bool = false;
    let mut x_off = 0.001;
    let mut y_off = 0.001;
    let mut instant = Instant::now();
    loop{

        //rendering part
        terminal.draw(|f| {
            let border = f.size();
            f.render_widget(&drawer, border);
            let time = instant.elapsed();

            let mut string: String = "".to_string();
            string.push_str(look_ahead[0].to_string().as_str());
            string.push_str(look_ahead[1].to_string().as_str());
            string.push_str(look_ahead[2].to_string().as_str());
            string.push_str(look_ahead[3].to_string().as_str());
            string.push_str(look_ahead[4].to_string().as_str());
            //let par_look = Paragraph::new(string);
            //f.render_widget(par_look, Rect{x: 0, y: 0, width: border.width, height: border.height});
            let par_time = Paragraph::new(time.as_millis().to_string());
            instant = Instant::now();
            f.render_widget(par_time, Rect{x: 0, y: 0, width: border.width, height: border.height});
        })?;

        //read inputs
        if crossterm::event::poll(Duration::from_millis(1))?{
            match crossterm::event::read()?{
               Event::FocusGained    => {},
               Event::FocusLost      => {},
               Event::Key(event_key) => {
                   match event_key.code{
                       KeyCode::Esc => debug_escape = true,
                       KeyCode::Char(chr) => input = chr,
                       KeyCode::Up    => {  drawer.y_off += y_off;},
                       KeyCode::Down  => {  drawer.y_off -= y_off;},
                       KeyCode::Left  => {  drawer.x_off -= x_off;},
                       KeyCode::Right => {  drawer.x_off += x_off;},
                       _ => {},
                   }
               },
               Event::Mouse(_)       => {},
               Event::Paste(_)       => {},
               Event::Resize(_, _)   => {},
            }
            match input{
                '+' => {
                    drawer.x_zm /= 2.0;
                    drawer.y_zm /= 2.0;
                    x_off /= 2.0;
                    y_off /= 2.0;
                }
                '-' => {
                    drawer.x_zm *= 2.0;
                    drawer.y_zm *= 2.0;
                    x_off *= 2.0;
                    y_off *= 2.0;
                }
                 _ => {}
            }
            look_ahead[0] = look_ahead[1];
            look_ahead[1] = look_ahead[2];
            look_ahead[2] = look_ahead[3];
            look_ahead[3] = look_ahead[4];
            look_ahead[4] = input;
        }
        else{
            input = 0 as char;
        }
        
        if debug_escape {
            break;
        }

    }
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    println!("\n\n\neverything okay\n");
    Ok(())
}
