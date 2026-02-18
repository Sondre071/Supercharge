use crate::snippets;

use snippets::utils::get_snippets;

pub fn browse_snippets() {
    let snippets = get_snippets();

    println!();

    for sn in snippets {
        println!("{}", sn);
    }

    println!("Done!");
}
