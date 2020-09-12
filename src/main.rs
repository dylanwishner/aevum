mod search;

use search::Search;
use std::panic;

fn main() {
    // Suppresses panic output to stdout if in release mode
    #[cfg(not(debug_assertions))]
    panic::set_hook(Box::new(|_info| {}));

    let search = Search::new();
    let target: u32 = search.clock / (1 / search.timer_period);

    for i in 1u64..2_u64.pow(32) {
        for j in 1u64..2_u64.pow(16) {
            if i * j == target as u64 {
                print(i - 1, j - 1);
            }
        }
    }
}

fn print(arr: u64, psc: u64) {
    println!("Prescaler: {}\tAuto-Reload: {}", psc, arr);
}