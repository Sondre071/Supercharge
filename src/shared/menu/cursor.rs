use crate::shared::terminal::{ACTIONS, COLORS};

pub enum Focus {
    BaseMenu,
    SubMenu,
}

pub struct Cursor<'a> {
    pub header: &'a str,
    pub subheaders: Vec<&'a str>,

    pub items: Vec<&'a str>,
    pub current: usize,
    pub visible_items: usize,
    pub total_height: usize,
    pub offset: usize,

    pub focus: Focus,
    pub submenu: Submenu<'a>,
}

pub struct Submenu<'a> {
    pub current: usize,
    pub items: Vec<&'a str>,
    x_offset: usize,
}

impl<'a> Cursor<'a> {
    pub fn new(
        header: &'a str,
        subheaders: Option<Vec<&'a str>>,
        items: Vec<&'a str>,
        submenu_items: Option<Vec<&'a str>>,
    ) -> Self {
        let subheaders = subheaders.unwrap_or(vec![]);
        let visible_items = items.len().min(20);
        let total_height = 1 + subheaders.len() + visible_items;

        let submenu = Submenu {
            current: 0,
            items: submenu_items.unwrap_or_default(),
            x_offset: items.iter().map(|i| i.len()).max().unwrap() + 4,
        };

        Self {
            header,
            subheaders,
            items,
            current: 0,
            offset: 0,
            visible_items,
            total_height,
            focus: Focus::BaseMenu,
            submenu,
        }
    }

    pub fn render_menu(&self) {
        let mut lines: Vec<String> = Vec::new();

        let pad_left = if !self.submenu.items.is_empty() {
            self.submenu.x_offset
        } else {
            0
        };

        let height = self.items.len().min(20);
        let offset = self.offset;

        for i in offset..(height + offset) {
            let mut line = {
                let prefix = if i == self.current { "> " } else { "  " };

                let color = match (i == self.current, &self.focus) {
                    (true, Focus::BaseMenu) => COLORS.Yellow,
                    (true, Focus::SubMenu) => COLORS.DimYellow,
                    (false, Focus::BaseMenu) => COLORS.Gray,
                    (false, Focus::SubMenu) => COLORS.DarkGray,
                };

                let content = format!("{}{}", prefix, self.items[i]);

                let text = format!("{:<width$}", content, width = pad_left);

                format!(
                    "{clear_line}{color}{}{reset}",
                    text,
                    clear_line = ACTIONS.ClearLine,
                    color = color,
                    reset = COLORS.Reset
                )
            };

            if matches!(self.focus, Focus::SubMenu) {
                line = self.add_submenu_text(i, line);
            }

            lines.push(line);
        }

        for line in lines {
            println!("{}", line);
        }
    }

    fn add_submenu_text(&self, index: usize, base_menu_line: String) -> String {
        if index < self.submenu.items.len() {
            let text = self.submenu.items[index];

            let (prefix, color) = if index == self.submenu.current {
                ("> ", COLORS.Yellow)
            } else {
                ("  ", COLORS.Gray)
            };

            format!(
                "{}{white}| {color}{prefix}{}{reset}",
                base_menu_line,
                text,
                white = COLORS.White,
                reset = COLORS.Reset
            )
        } else {
            base_menu_line
        }
    }
}
