use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use windows_sys::Win32::System::Console::{CTRL_C_EVENT, SetConsoleCtrlHandler};

pub fn enable_sigint() {
    unsafe { SetConsoleCtrlHandler(Some(ctrl_c_handler), 1) };
}

extern "system" fn ctrl_c_handler(ctrl_type: u32) -> i32 {
    if ctrl_type == CTRL_C_EVENT {
        std::process::exit(130);
    }
    0
}

pub fn key_is_pressed(letter: char) -> bool {
    if event::poll(std::time::Duration::from_millis(0)).unwrap_or(false)
        && let Ok(Event::Key(key_event)) = event::read()
        && key_event.code == KeyCode::Char(letter)
    {
        true
    } else {
        false
    }
}

pub fn read_key_blocking() -> KeyCode {
    loop {
        if let Some(ev) = read_key() {
            return ev;
        }
    }
}

pub fn read_key() -> Option<KeyCode> {
    match event::read() {
        Ok(Event::Key(ev)) => {
            if matches!(ev.kind, KeyEventKind::Press | KeyEventKind::Repeat) {
                Some(ev.code)
            } else {
                None
            }
        }
        _ => None,
    }
}
