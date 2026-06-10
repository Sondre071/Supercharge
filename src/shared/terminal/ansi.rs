#[allow(non_snake_case, dead_code)]
pub struct Colors {
    pub Black: &'static str,
    pub Gray: &'static str,
    pub DarkGray: &'static str,
    pub White: &'static str,

    pub Blue: &'static str,
    pub Cyan: &'static str,
    pub Green: &'static str,
    pub Purple: &'static str,
    pub Red: &'static str,
    pub Yellow: &'static str,
    pub DimOrange: &'static str,
    pub Orange: &'static str,

    pub DimYellow: &'static str,

    pub Reset: &'static str,
    
    
    pub RegalNavy: &'static str,
    pub RoyalGold: &'static str,
    pub SandyBrown: &'static str,
    pub LemonChiffron: &'static str,
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
    pub ClearScreen: &'static str,
    pub ClearToEnd: &'static str,
    pub ResetLocation: &'static str,
}

#[allow(non_snake_case, dead_code)]
pub const COLORS: Colors = Colors {
    Black: "\x1b[0;30m",
    Gray: "\x1b[0;37m",
    DarkGray: "\x1b[1;30m",
    White: "\x1b[1;37m",

    Blue: "\x1b[1;34m",
    Cyan: "\x1b[1;36m",
    Green: "\x1b[1;32m",
    Purple: "\x1b[1;35m",
    Red: "\x1b[1;31m",
    Yellow: "\x1b[1;33m",
    
    DimOrange: "\x1b[38;2;242;174;58m",
    Orange: "\x1b[38;2;252;184;68m",

    DimYellow: "\x1b[0;33m",
    
    RegalNavy: "\x1b[38;2;13;59;102m",
    RoyalGold: "\x1b[38;2;244;211;94m",
    SandyBrown: "\x1b[38;2;238;150;75m",
    LemonChiffron: "\x1b[38;2;250;240;202m",

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
    ClearScreen: "\x1b[2J",
    ClearToEnd: "\x1b[J",
    ResetLocation: "\x1b[H",
};
