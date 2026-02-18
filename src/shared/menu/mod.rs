mod cursor;
mod r#loop;
pub use r#loop::run;

mod draw;
pub use draw::{clear_menu, read_line, write_headers};

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

#[derive(Clone)]
pub struct Menu {
    pub header: String,
    pub subheaders: Vec<String>,
    pub items: Vec<Item>,
}

impl Menu {
    pub fn new(
        header: impl Into<String>,
        subheaders: Vec<impl Into<String>>,
        items: Vec<impl Into<String>>,
    ) -> Self {
        Self {
            header: header.into(),
            subheaders: subheaders.into_iter().map(Into::into).collect(),
            items: items.into_iter().map(Item::new).collect(),
        }
    }

    pub fn new_with_subitems(
        header: impl Into<String>,
        subheaders: Vec<impl Into<String>>,
        items: Vec<Item>,
    ) -> Self {
        Self {
            header: header.into(),
            subheaders: subheaders.into_iter().map(Into::into).collect(),
            items,
        }
    }
}
