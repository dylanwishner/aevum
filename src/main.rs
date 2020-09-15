mod search;

use search::{Search, TimeMode};
#[allow(unused_imports)]
use std::panic;
use std::process::exit;

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

    let target: u32 = (search.clock as f32
        / (1_f32 / (search.timer_period as f32 / multiplier as f32) as f32))
        as u32;

    find_matches(target, &search);
}

fn find_matches(target: u32, search: &Search) {
    let mut num_matches = 0;

    // Print target value if in debug mode
    #[cfg(debug_assertions)]
    println!("Searching for target: {}", target);

    for i in 1u64..2_u64.pow(16) {
        for j in 1u64..2_u64.pow(16) {
            if i * j == target as u64 {
                num_matches += 1;
                print_match(i - 1, j - 1);

                if num_matches == search.num_results {
                    exit(0);
                }
            }
        }
    }
}

fn print_match(arr: u64, psc: u64) {
    println!("Prescaler: {}\tAuto-Reload: {}", psc, arr);
}
