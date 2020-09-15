use clap::{App, Arg};

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
        let app = App::new("STM32 Timer Calculator")
            .version("0.1.0")
            .author("Dylan Wishner")
            .about("Determine the prescaler and auto-reload register values for a specified period of time for STM32 MCU timer peripherals")
            .arg(Arg::with_name("CLOCK SPEED")
                .short("c")
                .long("clock")
                .help("The clock speed of the timer peripheral")
                .default_value("8000000")
                .required(false)
                .takes_value(true))
            .arg(Arg::with_name("TIME")
                .help("The length of the timer (default is in seconds")
                .default_value("10")
                .required(true)
                .takes_value(true))
            .arg(Arg::with_name("EN_MICROSECONDS")
                .short("u")
                .long("microseconds")
                .help("Set the specified timer period to be in microseconds")
                .conflicts_with_all(&["EN_MILLISECONDS", "EN_SECONDS"])
                .overrides_with_all(&["EN_MILLISECONDS", "EN_SECONDS"])
                .takes_value(false))
            .arg(Arg::with_name("EN_MILLISECONDS")
                .short("m")
                .long("milliseconds")
                .help("Set the specified timer period to be in milliseconds")
                .takes_value(false))
            .arg(Arg::with_name("EN_SECONDS")
                .short("s")
                .long("seconds")
                .help("Set the specified timer period to be in seconds")
                .takes_value(false))
            .arg(Arg::with_name("RESULTS")
                .short("r")
                .long("results")
                .help("The number of results to return")
                .default_value("5")
                .takes_value(true));

        // Parse each parameter
        let matches = app.get_matches();
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
