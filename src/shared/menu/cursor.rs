use crate::shared::{
    menu::draw,
    terminal::{ACTIONS, COLORS},
};
use std::{
    cmp,
    io::{Write, stdout},
    iter,
};

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
        let height = self.items.len().min(20);

        let length = cmp::max(height + self.offset, self.items[self.current].items.len());

        let left_border_color = match self.focus {
            Focus::BaseMenu => COLORS.Gray,
            Focus::SubMenu => COLORS.DarkGray,
        };
        
        let mut lines = self.write_headers(left_border_color);

        for index in self.offset..length {
            let relative_index = index - self.offset;

            let line = self.format_line(index, relative_index, left_border_color);

            lines.push(line);
        }

        #[allow(clippy::print_with_newline)]
        for line in &lines {
            print!("{}\n", line);
        }

        stdout().flush().unwrap();

        lines.len()
    }

    fn format_line(
        &self,
        current_index: usize,
        relative_index: usize,
        border_color: &str,
    ) -> String {
        let prefix = if current_index == self.current {
            "► "
        } else {
            "  "
        };

        let value = self
            .items
            .get(current_index)
            .map(|item| item.value.clone())
            .unwrap_or_default();

        let content = format!("{}{}", prefix, value);
        let padded_text = format!("{:<width$}", content, width = self.submenu_x_offset);

        let color = match (current_index == self.current, &self.focus) {
            (true, Focus::BaseMenu) => COLORS.Yellow,
            (true, Focus::SubMenu) => COLORS.DimYellow,
            (false, Focus::BaseMenu) => COLORS.Gray,
            (false, Focus::SubMenu) => COLORS.DarkGray,
        };

        let mut text = format!(
            "{clear_line}{border_color}│{color}{}{reset}",
            padded_text,
            clear_line = ACTIONS.ClearLine,
            border_color = border_color,
            color = color,
            reset = COLORS.Reset
        );

        let current_item = &self.items[self.current];

        if matches!(self.focus, Focus::SubMenu) && relative_index < current_item.items.len() {
            text = self.add_submenu_text(current_index, text);
        }

        text
    }

    fn add_submenu_text(&self, i: usize, base_menu_line: String) -> String {
        let text = &self.items[self.current].items[i];

        let (prefix, color) = if i == self.submenu_current {
            ("► ", COLORS.Yellow)
        } else {
            ("  ", COLORS.Gray)
        };

        format!(
            "{}{gray}│{color}{prefix}{}{reset}",
            base_menu_line,
            text,
            gray = COLORS.Gray,
            reset = COLORS.Reset
        )
    }

    fn write_headers(&self, border_color: &str) -> Vec<String> {
        let mut lines = Vec::<String>::new();

        let header_text = {
            let width: usize = 30;

            // Truncate
            let mut header = self.header.to_owned();

            if header.chars().count() > width {
                let truncated: String = header.chars().take(width - 2).collect();
                header = format!("{truncated}..");
            }

            // Format
            let pad_left_len = (width.saturating_sub(header.chars().count()) - 2) / 2;
            let pad_right_len = width - pad_left_len - header.chars().count();

            let pad_left: String = iter::repeat_n("─", pad_left_len).collect();
            let pad_right: String = iter::repeat_n("─", pad_right_len).collect();
            format!("{} {} {}", pad_left, header, pad_right)
        };

        lines.push(format!(
            "{border_color}┌{yellow}{}{reset}",
            header_text,
            border_color = border_color,
            yellow = COLORS.Yellow,
            reset = COLORS.Reset
        ));

        for subheader in self.subheaders.iter() {
            lines.push(format!(
                "{border_color}│ {yellow}{}{reset}",
                subheader,
                border_color = border_color,
                yellow = COLORS.Yellow,
                reset = COLORS.Reset
            ));
        }

        lines
    }

    pub fn clear_menu(&self, rendered_items: usize) {
        let len = rendered_items + 1 + self.subheaders.len();
        draw::clear_menu(len);
    }
}
