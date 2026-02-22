use std::mem::MaybeUninit;
use windows_sys::Win32::System::Console::{
    CONSOLE_SCREEN_BUFFER_INFO, GetConsoleScreenBufferInfo, GetStdHandle, STD_OUTPUT_HANDLE,
};

pub fn get_screen_width() -> usize {
    let stdout = unsafe { GetStdHandle(STD_OUTPUT_HANDLE) };

    let mut info = MaybeUninit::<CONSOLE_SCREEN_BUFFER_INFO>::uninit();

    let csbi = unsafe {
        if GetConsoleScreenBufferInfo(stdout, info.as_mut_ptr()) == 0 {
            panic!("Failed to get console screen buffer info.")
        }

        info.assume_init()
    };

    (csbi.srWindow.Right - csbi.srWindow.Left + 1) as usize
}
