use windows_sys::Win32::System::Console::{CONSOLE_CURSOR_INFO, SetConsoleCursorInfo};

use windows_sys::Win32::Foundation::HANDLE;

pub fn init_console(stdout: HANDLE) {
    unsafe {
        let cursor_info = CONSOLE_CURSOR_INFO {
            dwSize: 1,
            bVisible: 0,
        };

        SetConsoleCursorInfo(stdout, &cursor_info);
    }
}
