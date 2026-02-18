#[allow(non_snake_case, dead_code)]
pub struct Colors {
    pub Gray: &'static str,
    pub DarkGray: &'static str,
    pub White: &'static str,

    pub Blue: &'static str,
    pub Cyan: &'static str,
    pub Green: &'static str,
    pub Purple: &'static str,
    pub Red: &'static str,
    pub Yellow: &'static str,

    pub DimYellow: &'static str,

    pub Reset: &'static str,
}

#[allow(non_snake_case, dead_code)]
pub struct Backgrounds {
    pub Black: &'static str,
    pub Red: &'static str,
    pub Green: &'static str,
    pub Yellow: &'static str,
    pub Blue: &'static str,
    pub Purple: &'static str,
    pub Cyan: &'static str,
    pub White: &'static str,
}

#[allow(non_snake_case, dead_code)]
pub struct Actions {
    pub ClearLine: &'static str,
}

#[allow(non_snake_case, dead_code)]
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

    DimYellow: "\x1b[0;33m",

    Reset: "\x1b[0m",
};

#[allow(non_snake_case, dead_code)]
pub const BACKGROUNDS: Backgrounds = Backgrounds {
    Black: "\x1b[40m",
    Red: "\x1b[41m",
    Green: "\x1b[42m",
    Yellow: "\x1b[43m",
    Blue: "\x1b[44m",
    Purple: "\x1b[45m",
    Cyan: "\x1b[46m",
    White: "\x1b[47m",
};

pub const ACTIONS: Actions = Actions {
    ClearLine: "\x1b[2K",
};
