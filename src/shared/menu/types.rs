use crate::shared::menu::cursor::*;

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
    pub cursor: Cursor,
}

impl Menu {
    pub fn new(
        header: impl Into<String>,
        subheaders: Vec<impl Into<String>>,
        items: Vec<impl Into<String>>,
    ) -> Self {
        let header = header.into();
        let subheaders = subheaders.into_iter().map(Into::into).collect();
        let items = items.into_iter().map(Item::new).collect();

        Self {
            cursor: Cursor::new(header, subheaders, items),
        }
    }

    pub fn new_with_subitems(
        header: impl Into<String>,
        subheaders: Vec<impl Into<String>>,
        items: Vec<Item>,
    ) -> Self {
        let header = header.into();
        let subheaders = subheaders.into_iter().map(Into::into).collect();

        Self {
            cursor: Cursor::new(header, subheaders, items),
        }
    }
}
