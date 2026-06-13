use crate::shared::terminal::codes::*;
use std::{
    io::{Write, stdout},
    iter,
};

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

pub enum Focus {
    BaseMenu,
    SubMenu,
}

pub type DisplayFunc = Option<Box<dyn Fn(&Cursor) -> Vec<String>>>;

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

    pub display_func: DisplayFunc,
}

impl Cursor {
    pub fn new<S, SH, I>(
        header: S,
        subheaders: Option<Vec<SH>>,
        items: Vec<I>,
        display_func: DisplayFunc,
    ) -> Self
    where
        S: Into<String>,
        SH: Into<String>,
        I: Into<String>,
    {
        Self::init(
            header.into(),
            subheaders.into_iter().flatten().map(Into::into).collect(),
            items.into_iter().map(Item::new).collect(),
            display_func,
        )
    }

    pub fn new_with_subitems<S, SH>(
        header: S,
        subheaders: Option<Vec<SH>>,
        items: Vec<Item>,
        display_func: DisplayFunc,
    ) -> Self
    where
        S: Into<String>,
        SH: Into<String>,
    {
        let header = header.into();
        let subheaders = subheaders.into_iter().flatten().map(Into::into).collect();

        Self::init(header, subheaders, items, display_func)
    }

    fn init(
        header: String,
        subheaders: Vec<String>,
        items: Vec<Item>,
        display_func: DisplayFunc,
    ) -> Self {
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

            display_func,
        }
    }
}

impl Cursor {
    pub fn render_menu(&mut self) -> usize {
        let height = self.items.len().min(20);

        let length = match self.focus {
            Focus::BaseMenu => height + self.offset,
            Focus::SubMenu => (height + self.offset).max(self.items[self.current].items.len()),
        };

        let mut lines = self.write_headers();

        for index in self.offset..length {
            let relative_index = index - self.offset;

            let line = self.format_line(index, relative_index);

            lines.push(line);
        }

        if let Some(func) = &self.display_func {
            lines.append(&mut func(self));
        }

        lines.push(format!(
            "{CLEAR_LINE}{BORDER_COLOR}└{RESET_COLOR}{CLEAR_TO_END}"
        ));

        #[allow(clippy::print_with_newline)]
        for line in &lines {
            print!("{}\n", line);
        }

        stdout().flush().unwrap();

        lines.len()
    }

    fn format_line(&self, current_index: usize, relative_index: usize) -> String {
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

        let text_color = match (current_index == self.current, &self.focus) {
            (true, Focus::BaseMenu) => TEXT_HIGHLIGHTED_COLOR,
            (true, Focus::SubMenu) => TEXT_HIGHLIGHTED_FADED_COLOR,
            (false, Focus::BaseMenu) => TEXT_COLOR,
            (false, Focus::SubMenu) => TEXT_FADED_COLOR,
        };

        let mut text = format!(
            "{CLEAR_LINE}{BORDER_COLOR}│{text_color}{}{RESET_COLOR}",
            padded_text,
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
            ("► ", TEXT_HIGHLIGHTED_COLOR)
        } else {
            ("  ", TEXT_COLOR)
        };

        format!("{base_menu_line}{BORDER_COLOR}│{color}{prefix}{text}{RESET_COLOR}",)
    }

    fn write_headers(&self) -> Vec<String> {
        let mut lines = Vec::<String>::new();

        let (header, left_line, right_line) = {
            let width: usize = 30;

            // Truncate
            let mut header = self.header.to_owned();

            if header.chars().count() > width - 6 {
                let truncated: String = header.chars().take(width - 6).collect();
                header = format!("{truncated}..");
            }

            // Format
            let pad_left_len = (width
                .saturating_sub(header.chars().count())
                .saturating_sub(2))
                / 2;
            let pad_right_len = width
                .saturating_sub(pad_left_len)
                .saturating_sub(header.chars().count());

            let pad_left: String = iter::repeat_n("─", pad_left_len).collect();
            let pad_right: String = iter::repeat_n("─", pad_right_len).collect();
            (header, pad_left, pad_right)
        };

        lines.push(format!(
            "{BORDER_COLOR}┌{left_line} {header} {right_line}{RESET_COLOR}",
        ));

        for subheader in self.subheaders.iter() {
            lines.push(format!(
                "{BORDER_COLOR}│ {INFO_COLOR}{subheader}{RESET_COLOR}",
            ));
        }

        lines
    }
}
