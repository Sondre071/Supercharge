use windows_sys::Win32::System::Console::{INPUT_RECORD, KEY_EVENT, ReadConsoleInputW};
use windows_sys::Win32::Foundation::HANDLE;

#[derive(Debug)]
pub struct KeyEvent {
    pub ch: Option<char>,
}

pub fn read_key(stdin: HANDLE) -> Option<KeyEvent> {
    let mut records: [INPUT_RECORD; 16] = unsafe { std::mem::zeroed() };
    let mut read = 0;

    let ok =
        unsafe { ReadConsoleInputW(stdin, records.as_mut_ptr(), records.len() as u32, &mut read) };

    if ok == 0 {
        return None; // OS call failed.
    }

    for rec in &records[..read as usize] {
        if rec.EventType as u32 == KEY_EVENT {
            let ev = unsafe { rec.Event.KeyEvent };

            if ev.bKeyDown != 0 {
                let u = unsafe { ev.uChar.UnicodeChar };
                let ch = if u == 0 {
                    None
                } else {
                    char::from_u32(u as u32)
                };
                return Some(KeyEvent { ch });
            }
        }
    }

    None
}

pub fn read_key_blocking(stdin: HANDLE) -> KeyEvent {
        loop {
            if let Some(ev) = read_key(stdin) {
                return ev;
            }
        }
}
