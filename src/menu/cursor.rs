use crate::utils;

use utils::terminal;
use terminal::COLORS;

pub struct Cursor<'a> {
    pub header: &'a str,
    pub subheaders: Vec<&'a str>,

    pub items: Vec<&'a str>,
    pub current: usize,
    offset: usize,
    pub visible_items: usize,
    pub total_height: usize,
}

impl<'a> Cursor<'a> {
    pub fn new(header: &'a str, subheaders: Option<Vec<&'a str>>, items: Vec<&'a str>) -> Self {
        let subheaders = subheaders.unwrap_or(vec![]);
        let visible_items = items.len().min(20);
        let total_height = 1 + subheaders.len() + visible_items;

        Self {
            header,
            subheaders,
            items,
            current: 0,
            offset: 0,
            visible_items,
            total_height,
        }
    }

    pub fn render_menu(&self) {
        let height = self.items.len().min(20);
        let offset = self.offset;

        for i in offset..(height + offset) {
            if i == self.current {
                println!(
                    "{yellow}> {}{reset}",
                    self.items[i],
                    yellow = COLORS.Yellow,
                    reset = COLORS.Reset
                );
            } else {
                println!("  {}", self.items[i]);
            }
        }
    }

    pub fn increment(&mut self) {
        let offset = &self.offset;

        let pos_from_bottom = self.visible_items - (self.current - offset + 1);

        if self.current < self.items.len() - 1 {
            self.current += 1;
        }

        if pos_from_bottom == 1 && self.current < self.items.len() - 1 {
            self.offset += 1
        }
    }

    pub fn decrement(&mut self) {
        let pos_from_top = self.current - self.offset;

        if self.current > 0 {
            self.current -= 1;
        }

        if self.offset > 0 && pos_from_top <= 1 {
            self.offset -= 1
        }
    }
}
