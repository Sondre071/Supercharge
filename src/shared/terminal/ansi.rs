#[allow(dead_code)]
pub mod codes {
    pub const BORDER_COLOR: &str = "\x1b[38;2;238;150;75m";
    pub const INFO_COLOR: &str = "\x1b[38;2;128;193;254m";
    pub const CONTENT_COLOR: &str = "\x1b[38;2;169;213;254m";

    pub const TEXT_HIGHLIGHTED_COLOR: &str = "\x1b[38;2;254;225;120m";
    pub const TEXT_HIGHLIGHTED_FADED_COLOR: &str = "\x1b[38;2;180;161;94m";
    pub const TEXT_COLOR: &str = "\x1b[0;37m";
    pub const TEXT_FADED_COLOR: &str = "\x1b[1;30m";

    pub const SUCCESS_COLOR: &str = "\x1b[1;32m";
    pub const WARNING_COLOR: &str = "\x1b[38;2;254;225;120m";
    pub const DANGER_COLOR: &str = "\x1b[1;31m";

    pub const RESET_COLOR: &str = "\x1b[0m";

    pub const CLEAR_LINE: &str = "\x1b[2K";
    pub const CLEAR_SCREEN: &str = "\x1b[2J";
    pub const CLEAR_TO_END: &str = "\x1b[J";
    pub const RESET_CURSOR_LOCATION: &str = "\x1b[H";
}
