use std::{thread, time::Duration};

use crossterm::{
    self,
//    execute,
    event::{Event, KeyCode},
};

/*
enum Typeable{
    Ascii([char; 128]),
    Code([u8; 128]),
}

struct Keys{
    pub typeable: Typeable,
    pub up: bool,
    pub down: bool,
    pub right: bool,
    pub left: bool,
    pub exit: bool
}

impl Keys{
    const fn new() -> Keys{
        return Keys{
            typeable: Typeable::Code([0; 128]),
            up: false,
            down: false,
            right: false,
            left: false,
            exit: false,
        };
    }
}

static mut RELEASE : Keys = Keys::new();
static mut PRESSING: Keys = Keys::new();
static mut PRESSED : Keys = Keys::new();
*/

static mut PRESSED: KeyCode = KeyCode::Null;

fn io_loop() {
    loop{
        match  crossterm::event::read().unwrap() {
            Event::Key(ekey) => {
                unsafe{
                    if PRESSED == ekey.code {
                        continue;
                    }
                    PRESSED = ekey.code;
                }
                if ekey.code == KeyCode::Esc{
                    return;
                }
            },
            _ => {}
        } 
        thread::sleep(Duration::from_millis(100));
        unsafe {PRESSED = KeyCode::Null;}
    }
}

pub fn get_key() -> &'static KeyCode { unsafe{
        return &PRESSED;
    }
}

pub fn init_io() -> Result<(), std::io::Error> {
    thread::spawn(io_loop);
    Ok(())
}
