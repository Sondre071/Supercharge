use crate::shared::{menu, terminal};

use menu::cursor::{self, Focus};

pub fn run<'a>(
    header: &'a str,
    subheaders: Option<Vec<&'a str>>,
    items: Vec<&'a str>,
    submenu_items: Option<Vec<&'a str>>,
) -> Option<&'a str> {
    let mut cursor = cursor::Cursor::new(header, subheaders, items, submenu_items);
    terminal::set_cursor_visibility(false);

    menu::write_headers(cursor.header, Some(&cursor.subheaders));

    let mut start_y = terminal::get_cursor_pos().Y;

    loop {
        terminal::set_cursor_pos(0, start_y as usize);

        cursor.render_menu();

        let key = terminal::read_key_blocking();

        if let Some(ch) = key.ch {
            match ch {
                'q' | 'h' => match &cursor.focus {
                    Focus::BaseMenu => {
                        menu::clear_menu(cursor.total_height);
                        terminal::set_cursor_visibility(true);
                        return None;
                    }
                    Focus::SubMenu => {
                        cursor.submenu.current = 0;
                        cursor.focus = Focus::BaseMenu
                    }
                },

                'j' => match cursor.focus {
                    Focus::BaseMenu => {
                        let offset = cursor.offset;

                        let pos_from_bottom = cursor.visible_items - (cursor.current - offset + 1);

                        if cursor.current < cursor.items.len() - 1 {
                            cursor.current += 1;
                        }

                        if pos_from_bottom == 1 && cursor.current < cursor.items.len() - 1 {
                            cursor.offset += 1
                        }
                    }
                    Focus::SubMenu => {
                        if cursor.submenu.current < cursor.submenu.items.len() - 1 {
                            cursor.submenu.current += 1;
                        }
                    }
                },
                'k' => match cursor.focus {
                    Focus::BaseMenu => {
                        let pos_from_top = cursor.current - cursor.offset;

                        if cursor.current > 0 {
                            cursor.current -= 1;
                        }

                        if cursor.offset > 0 && pos_from_top <= 1 {
                            cursor.offset -= 1
                        }
                    }
                    Focus::SubMenu => {
                        if cursor.submenu.current > 0 {
                            cursor.submenu.current -= 1;
                        }
                    }
                },

                'l' => match (&cursor.focus, &cursor.submenu.items.is_empty()) {
                    (Focus::BaseMenu, false) => {
                        cursor.focus = Focus::SubMenu;
                    }
                    (Focus::BaseMenu, true) => {
                        menu::clear_menu(cursor.total_height);
                        terminal::set_cursor_visibility(true);
                        return Some(cursor.items[cursor.current]);
                    }
                    _ => {
                        menu::clear_menu(cursor.total_height);
                        terminal::set_cursor_visibility(true);

                        let return_item = &cursor.submenu.items[cursor.submenu.current];

                        return Some(return_item);
                    }
                },

                _ => {
                    continue;
                }
            }
        }

        start_y = terminal::get_cursor_pos().Y - cursor.visible_items as i16;
    }
}
