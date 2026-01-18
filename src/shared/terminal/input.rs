use windows_sys::Win32::Foundation::HANDLE;
use windows_sys::Win32::System::Console::{
    GetStdHandle, INPUT_RECORD, KEY_EVENT, ReadConsoleInputW, STD_INPUT_HANDLE,
};
use windows_sys::Win32::UI::Input::KeyboardAndMouse::GetAsyncKeyState;

#[derive(Debug)]
pub struct KeyEvent {
    pub ch: Option<char>,
}

pub fn key_is_pressed(letter: char) -> bool {
    let c = letter.to_ascii_uppercase();

    if !c.is_alphabetic() {
        return false;
    }

    let vk = c as i32;
    unsafe { ((GetAsyncKeyState(vk) as i32) & 0x8000) != 0 }
}

pub fn read_key_blocking() -> KeyEvent {
    let stdin: HANDLE = unsafe { GetStdHandle(STD_INPUT_HANDLE) };

    loop {
        if let Some(ev) = read_key(stdin) {
            return ev;
        }
    }
}

pub fn read_key(stdin: HANDLE) -> Option<KeyEvent> {
    let mut records: [INPUT_RECORD; 16] = unsafe { std::mem::zeroed() };
    let mut read = 0;

    let ok =
        unsafe { ReadConsoleInputW(stdin, records.as_mut_ptr(), records.len() as u32, &mut read) };

    if ok == 0 {
        return None;
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
