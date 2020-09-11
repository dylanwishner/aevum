use clap::{App, Arg};

pub struct Search {
    pub clock: u32,
}

impl Search {
    pub fn new() -> Search {
        let app = App::new("STM32 Timer Calculator")
            .version("0.1.0")
            .author("Dylan Wishner")
            .about("Determine the prescaler and auto-reload register values for a specified period of time for STM32 MCU timer peripherals")
            .arg(Arg::with_name("CLOCK")
                .short("c")
                .long("clock")
                .help("The clock speed of the timer peripheral")
                .default_value("8000000")
                .required(false));

        // Parse each parameter
        let matches = app.get_matches();
        let clock = matches.value_of("CLOCK").unwrap();

        // Validate that all arg values are of type u32
        let args = vec![clock];
        for arg in args {
            match arg.parse::<u32>() {
                Ok(_) => continue,
                Err(_) => println!(
                    "Usage error: parameter must be of type int. Use --help for more details."
                ),
            }
        }

        // Return the search values after being parsed and verified
        Search {
            clock: clock.parse::<u32>().unwrap(),
        }
    }
}
