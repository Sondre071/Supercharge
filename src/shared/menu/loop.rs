use crate::shared::{
    menu::{self, Menu},
    terminal,
};

use menu::cursor::{self, Focus};

pub fn run(menu: Menu) -> Option<(String, Option<String>)> {
    let mut cursor = cursor::Cursor::new(menu);
    terminal::set_cursor_visibility(false);

    menu::write_headers(cursor.header.clone(), cursor.subheaders.clone());

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
                        cursor.submenu_current = 0;
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
                        let current_item = &cursor.items[cursor.current];

                        if cursor.submenu_current < current_item.items.len() - 1 {
                            cursor.submenu_current += 1;
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
                        if cursor.submenu_current > 0 {
                            cursor.submenu_current -= 1;
                        }
                    }
                },

                'l' => {
                    let current_item = &cursor.items[cursor.current];

                    match (&cursor.focus, current_item.items.is_empty()) {
                        (Focus::BaseMenu, false) => {
                            cursor.focus = Focus::SubMenu;
                        }
                        (Focus::BaseMenu, true) => {
                            menu::clear_menu(cursor.total_height);
                            terminal::set_cursor_visibility(true);

                            return Some((current_item.value.clone(), None));
                        }
                        _ => {
                            menu::clear_menu(cursor.total_height);
                            terminal::set_cursor_visibility(true);

                            return Some((
                                current_item.value.clone(),
                                Some(current_item.items[cursor.submenu_current].clone()),
                            ));
                        }
                    }
                }

                _ => {
                    continue;
                }
            }
        }

        start_y = terminal::get_cursor_pos().Y - cursor.visible_items as i16;
    }
}
