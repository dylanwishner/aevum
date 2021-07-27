use crate::cli;

pub struct Search {
    pub clock: u32,
    pub num_results: u32,
    pub timer_period: u32,
    pub time_unit: TimeMode,
}

impl Search {
    // TODO replace each arg with custom validators
    #[allow(unused)] // Silence unused variable (time_unit) warnings even though its read, possible rustc issue?
    pub fn new() -> Search {
        // Parse each parameter
        let matches = cli::matches();
        let clock = matches.value_of("CLOCK SPEED").unwrap();
        let timer_period = matches.value_of("TIME").unwrap();
        let mut time_unit: TimeMode = TimeMode::Seconds;
        let num_results = matches.value_of("RESULTS").unwrap();

        // Validate that all option values are of type u32
        let args = vec![clock, timer_period, num_results];
        for arg in args {
            match arg.parse::<u32>() {
                Ok(_) => continue,
                Err(_) => println!(
                    "Usage error: Character detected where an integer was expected. Use --help for more details."
                ),
            }
        }

        // Check if a time unit was passed
        if matches.is_present("EN_MICROSECONDS") {
            time_unit = TimeMode::Microseconds
        } else if matches.is_present("EN_MILLISECONDS") {
            time_unit = TimeMode::Milliseconds
        } else {
            // Default to seconds
            time_unit = TimeMode::Seconds
        }

        // Return the search values after being parsed and verified
        Search {
            clock: clock.parse::<u32>().unwrap(),
            num_results: num_results.parse::<u32>().unwrap(),
            timer_period: timer_period.parse::<u32>().unwrap(),
            time_unit,
        }
    }
}

pub enum TimeMode {
    Milliseconds,
    Microseconds,
    Seconds,
}
