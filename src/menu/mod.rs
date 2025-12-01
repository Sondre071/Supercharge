pub mod r#loop;

mod cursor;
mod input;

pub enum MenuId {
    Home,
    Settings,
    OpenRouter,

    Quit,
}

pub fn to_home() -> MenuId {
    MenuId::Home
}

pub fn to_settings() -> MenuId {
    MenuId::Settings
}

pub fn to_openrouter() -> MenuId {
    MenuId::OpenRouter
}

pub fn to_quit() -> MenuId {
    MenuId::Quit
}

pub struct Item {
    pub name: &'static str,
    pub next: fn() -> MenuId,
}

pub struct Menu {
    pub header: &'static str,
    pub subheaders: &'static [&'static str],
    pub options: &'static [Item],
}

static HOME: Menu = Menu {
    header: "Supercharge",
    subheaders: &[],
    options: &[
        Item {
            name: "OpenRouter",
            next: to_openrouter,
        },
        Item {
            name: "Settings",
            next: to_settings,
        },
        Item {
            name: "Quit",
            next: to_quit,
        },
    ],
};

static SETTINGS: Menu = Menu {
    header: "Settings",
    subheaders: &[],
    options: &[Item {
        name: "Back",
        next: to_home,
    }],
};

static OPENROUTER: Menu = Menu {
    header: "OpenRouter",
    subheaders: &["Her kan du chatte!", ""],
    options: &[Item {
        name: "Back",
        next: to_home,
    }],
};