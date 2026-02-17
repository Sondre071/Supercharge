use crate::shared::menu;
use crate::snippets::{self, run};

use snippets::utils::get_snippets;

pub fn main() {
    let choice = menu::run("Snippets", None, vec!["Browse snippets", "Add snippet"]).unwrap();

    if choice == "Browse snippets" {
        let snippets = get_snippets();

        println!();

        for sn in snippets {
            println!("{}", sn);
        }

        println!("Done!");
    } else {
        run::add_snippet();
    }
}
