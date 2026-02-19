use crate::shared::{
    menu::draw,
    terminal::{ACTIONS, COLORS},
};
use std::cmp;

#[derive(Clone)]
pub struct Item {
    pub value: String,
    pub items: Vec<String>,
}

impl Item {
    pub fn new(value: impl Into<String>) -> Self {
        Self {
            value: value.into(),
            items: Vec::new(),
        }
    }

    pub fn new_with_subitems(value: impl Into<String>, items: Vec<impl Into<String>>) -> Self {
        Self {
            value: value.into(),
            items: items.into_iter().map(Into::into).collect(),
        }
    }
}

pub const NONE: Option<Vec<String>> = None;

#[derive(Clone)]
pub enum Focus {
    BaseMenu,
    SubMenu,
}

#[derive(Clone)]
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
}

impl Cursor {
    pub fn new<S, SH, I>(header: S, subheaders: Option<Vec<SH>>, items: Vec<I>) -> Self
    where
        S: Into<String>,
        SH: Into<String>,
        I: Into<String>,
    {
        Self::init(
            header.into(),
            subheaders.into_iter().flatten().map(Into::into).collect(),
            items.into_iter().map(Item::new).collect(),
        )
    }

    pub fn new_with_subitems<S, SH>(
        header: S,
        subheaders: Option<Vec<SH>>,
        items: Vec<Item>,
    ) -> Self
    where
        S: Into<String>,
        SH: Into<String>,
    {
        let header = header.into();
        let subheaders = subheaders.into_iter().flatten().map(Into::into).collect();

        Self::init(header, subheaders, items)
    }

    fn init(header: String, subheaders: Vec<String>, items: Vec<Item>) -> Self {
        let visible_items = items.len().min(20);
        let submenu_x_offset = items.iter().map(|i| i.value.len()).max().unwrap() + 4;

        Self {
            header,
            subheaders,
            items,

            focus: Focus::BaseMenu,

            current: 0,
            submenu_current: 0,
            submenu_x_offset,

            offset: 0,
            visible_items,
        }
    }
}

impl Cursor {
    pub fn render_menu(&mut self) -> usize {
        let mut lines: Vec<String> = Vec::new();

        let height = self.items.len().min(20);

        let current_selected_item = self.items[self.current].clone();

        let length = cmp::max(height + self.offset, current_selected_item.items.len());

        for i in self.offset..length {
            let relative_index = i - self.offset;

            let color = match (i == self.current, &self.focus) {
                (true, Focus::BaseMenu) => COLORS.Yellow,
                (true, Focus::SubMenu) => COLORS.DimYellow,
                (false, Focus::BaseMenu) => COLORS.Gray,
                (false, Focus::SubMenu) => COLORS.DarkGray,
            };

            let mut line = {
                let prefix = if i == self.current { "> " } else { "  " };

                let value = self
                    .items
                    .get(i)
                    .map(|item| item.value.clone())
                    .unwrap_or_default();

                let content = format!("{}{}", prefix, value);
                let padded_text = format!("{:<width$}", content, width = self.submenu_x_offset);

                format!(
                    "{clear_line}{color}{}{reset}",
                    padded_text,
                    clear_line = ACTIONS.ClearLine,
                    color = color,
                    reset = COLORS.Reset
                )
            };

            if matches!(self.focus, Focus::SubMenu)
                && relative_index < current_selected_item.items.len()
            {
                let submenu_items = self.items[self.current].clone();

                line = self.add_submenu_text(i, submenu_items, line);
            }

            lines.push(line);
        }

        for line in &lines {
            println!("{}", line);
        }

        lines.len()
    }

    fn add_submenu_text(&self, i: usize, current_item: Item, base_menu_line: String) -> String {
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

    pub fn clear_menu(&self, rendered_items: usize) {
        let len = rendered_items + 1 + self.subheaders.len();
        draw::clear_menu(len);
    }
}
