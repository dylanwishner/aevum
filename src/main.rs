mod search;

use search::Search;
use std::panic;

fn main() {
    // Suppresses error output to stdout
    panic::set_hook(Box::new(|_info| {}));

    let search = Search::new();
}
