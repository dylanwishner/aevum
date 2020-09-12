mod search;

use search::{Search, TimeMode};
#[allow(unused_imports)]
use std::panic;

fn main() {
    // Suppresses panic output to stdout if in release mode
    #[cfg(not(debug_assertions))]
    panic::set_hook(Box::new(|_info| {}));

    let search = Search::new();
    let multiplier = match search.time_unit {
        TimeMode::Microseconds => 1000000,
        TimeMode::Milliseconds => 1000,
        TimeMode::Seconds => 1,
    };

    let target: u32 = search.clock * search.timer_period / multiplier;

    find_matches(target);
}

fn find_matches(target: u32) {
    for i in 1u64..2_u64.pow(16) {
        for j in 1u64..2_u64.pow(16) {
            if i * j == target as u64 {
                print_match(i - 1, j - 1);
            }
        }
    }
}

fn print_match(arr: u64, psc: u64) {
    println!("Prescaler: {}\tAuto-Reload: {}", psc, arr);
}
