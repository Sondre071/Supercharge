use crate::shared::{
    menu::{self, Menu},
    terminal,
};

use menu::cursor::{Focus};

pub fn run(menu: &mut Menu) -> Option<(String, Option<String>)> {
    terminal::set_cursor_visibility(false);

    menu::write_headers(menu.cursor.header.clone(), menu.cursor.subheaders.clone());

    let mut start_y = terminal::get_cursor_pos().Y;

    loop {
        terminal::set_cursor_pos(0, start_y as usize);

        menu.cursor.render_menu();

        let key = terminal::read_key_blocking();

        if let Some(ch) = key.ch {
            match ch {
                'q' | 'h' => match menu.cursor.focus {
                    Focus::BaseMenu => {
                        menu::clear_menu(menu.cursor.total_height);
                        terminal::set_cursor_visibility(true);
                        return None;
                    }
                    Focus::SubMenu => {
                        menu.cursor.submenu_current = 0;
                        menu.cursor.focus = Focus::BaseMenu
                    }
                },

                'j' => match menu.cursor.focus {
                    Focus::BaseMenu => {
                        let offset = menu.cursor.offset;

                        let pos_from_bottom = menu.cursor.visible_items - (menu.cursor.current - offset + 1);

                        if menu.cursor.current < menu.cursor.items.len() - 1 {
                            menu.cursor.current += 1;
                        }

                        if pos_from_bottom == 1 && menu.cursor.current < menu.cursor.items.len() - 1 {
                            menu.cursor.offset += 1
                        }
                    }
                    Focus::SubMenu => {
                        let current_item = &menu.cursor.items[menu.cursor.current];

                        if menu.cursor.submenu_current < current_item.items.len() - 1 {
                            menu.cursor.submenu_current += 1;
                        }
                    }
                },
                'k' => match menu.cursor.focus {
                    Focus::BaseMenu => {
                        let pos_from_top = menu.cursor.current - menu.cursor.offset;

                        if menu.cursor.current > 0 {
                            menu.cursor.current -= 1;
                        }

                        if menu.cursor.offset > 0 && pos_from_top <= 1 {
                            menu.cursor.offset -= 1
                        }
                    }
                    Focus::SubMenu => {
                        if menu.cursor.submenu_current > 0 {
                            menu.cursor.submenu_current -= 1;
                        }
                    }
                },

                'l' => {
                    let current_item = &menu.cursor.items[menu.cursor.current];

                    match (&menu.cursor.focus, current_item.items.is_empty()) {
                        (Focus::BaseMenu, false) => {
                            menu.cursor.focus = Focus::SubMenu;
                        }
                        (Focus::BaseMenu, true) => {
                            menu::clear_menu(menu.cursor.total_height);
                            terminal::set_cursor_visibility(true);

                            return Some((current_item.value.clone(), None));
                        }
                        _ => {
                            menu::clear_menu(menu.cursor.total_height);
                            terminal::set_cursor_visibility(true);

                            return Some((
                                current_item.value.clone(),
                                Some(current_item.items[menu.cursor.submenu_current].clone()),
                            ));
                        }
                    }
                }

                _ => {
                    continue;
                }
            }
        }

        start_y = terminal::get_cursor_pos().Y - menu.cursor.visible_items as i16;
    }
}
