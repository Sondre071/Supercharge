mod colors;
pub use colors::COLORS;

mod cursor;
pub use cursor::ACTIONS;

mod console;
pub use console::{get_cursor_pos, set_cursor_pos, set_cursor_visibility};

mod input;
pub use input::read_key_blocking;
