#[allow(non_snake_case, dead_code)]
pub struct Colors {
    pub Gray: &'static str,
    pub DarkGray : &'static str,
    pub White: &'static str,

    pub Blue: &'static str,
    pub Cyan: &'static str,
    pub Green: &'static str,
    pub Purple: &'static str,
    pub Red: &'static str,
    pub Yellow: &'static str,
}

pub const COLORS: Colors = Colors {
    Gray: "\x1b[0;37m",
    DarkGray: "\x1b[1;30m",
    White: "\x1b[1;37m",

    Blue: "\x1b[1;34m",
    Cyan: "\x1b[1;36m",
    Green: "\x1b[1;32m",
    Purple: "\x1b[1;35m",
    Red: "\x1b[1;31m",
    Yellow: "\x1b[1;33m",
};
