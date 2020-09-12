use clap::{App, Arg};

pub struct Search {
    pub clock: u32,
    pub timer_period: u32,
}

impl Search {
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
                .required(false))
            .arg(Arg::with_name("TIME")
                .help("The length of the timer in seconds")
                .default_value("0")
                .required(true));

        // Parse each parameter
        let matches = app.get_matches();
        let clock = matches.value_of("CLOCK SPEED").unwrap();
        let timer_period = matches.value_of("TIME").unwrap();

        // Validate that all arg values are of type u32
        let args = vec![clock, timer_period];
        for arg in args {
            match arg.parse::<u32>() {
                Ok(_) => continue,
                Err(_) => println!(
                    "Usage error: Character detected where an integer was expected. Use --help for more details."
                ),
            }
        }

        // Return the search values after being parsed and verified
        Search {
            clock: clock.parse::<u32>().unwrap(),
            timer_period: timer_period.parse::<u32>().unwrap(),
        }
    }
}
