mod colors;
pub use colors::COLORS;

mod cursor;
pub use cursor::ACTIONS;

mod console;
pub use console::{get_cursor_position, set_cursor_visibility};
