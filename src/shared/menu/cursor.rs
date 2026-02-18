use crate::shared::menu::{Item, Menu};
use crate::shared::terminal::{ACTIONS, COLORS};

pub enum Focus {
    BaseMenu,
    SubMenu,
}

pub struct Cursor {
    pub header: String,
    pub subheaders: Vec<String>,
    pub items: Vec<Item>,

    pub focus: Focus,

    pub current: usize,
    pub submenu_current: usize,
    pub submenu_x_offset: usize,

    pub offset: usize,
    pub visible_items: usize,
    pub total_height: usize,
}

impl Cursor {
    pub fn new(menu: Menu) -> Self {
        let visible_items = menu.items.len().min(20);
        let total_height = 1 + menu.subheaders.len() + visible_items;

        Self {
            header: menu.header,
            subheaders: menu.subheaders,
            items: menu.items.clone(),

            focus: Focus::BaseMenu,

            current: 0,
            submenu_current: 0,
            submenu_x_offset: menu.items.iter().map(|i| i.value.len()).max().unwrap() + 4,

            offset: 0,
            visible_items,
            total_height,
        }
    }

    pub fn render_menu(&self) {
        let mut lines: Vec<String> = Vec::new();

        let height = self.items.len().min(20);

        for i in self.offset..(height + self.offset) {
            let current_item = self.items[i].clone();

            let mut line = {
                let prefix = if i == self.current { "> " } else { "  " };

                let color = match (i == self.current, &self.focus) {
                    (true, Focus::BaseMenu) => COLORS.Yellow,
                    (true, Focus::SubMenu) => COLORS.DimYellow,
                    (false, Focus::BaseMenu) => COLORS.Gray,
                    (false, Focus::SubMenu) => COLORS.DarkGray,
                };

                let content = format!("{}{}", prefix, current_item.value.clone());

                let text = format!("{:<width$}", content, width = self.submenu_x_offset);

                format!(
                    "{clear_line}{color}{}{reset}",
                    text,
                    clear_line = ACTIONS.ClearLine,
                    color = color,
                    reset = COLORS.Reset
                )
            };

            if matches!(self.focus, Focus::SubMenu) && i < self.items[self.current].items.len() {
                let submenu_items = self.items[self.current].clone();
                
                line = self.add_submenu_text(i, submenu_items, line);
            }

            lines.push(line);
        }

        for line in lines {
            println!("{}", line);
        }
    }

    fn add_submenu_text(
        &self,
        i: usize,
        current_item: Item,
        base_menu_line: String,
    ) -> String {
        let text = current_item.items[i].clone();

        let (prefix, color) = if i == self.submenu_current {
            ("> ", COLORS.Yellow)
        } else {
            ("  ", COLORS.Gray)
        };

        format!(
            "{}{gray}│ {color}{prefix}{}{reset}",
            base_menu_line,
            text,
            gray = COLORS.DarkGray,
            reset = COLORS.Reset
        )
    }
}
