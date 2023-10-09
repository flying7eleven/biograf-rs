use clap::{crate_authors, crate_description, crate_name, crate_version, Parser};

#[derive(Parser)]
#[clap(name = crate_name!(), version = crate_version!(), author = crate_authors!(), about = crate_description!())]
struct Biograf {
    /// Do not really modify the data, just simulate based on the input data
    #[clap(short, long)]
    dry_run: bool,
    /// A level of verbosity, and can be used multiple times
    #[arg(short, long, action = clap::ArgAction::Count)]
    verbose: u8,
}

fn setup_logging(verbosity_level: u8) {
    use chrono::Utc;

    // create an instance for the Dispatcher to create a new logging configuration
    let mut base_config = fern::Dispatch::new();

    // determine the logging level based on the verbosity the user chose
    base_config = match verbosity_level {
        0 => base_config.level(log::LevelFilter::Warn),
        1 => base_config.level(log::LevelFilter::Info),
        2 => base_config.level(log::LevelFilter::Debug),
        _3_or_more => base_config.level(log::LevelFilter::Trace),
    };

    // define how a logging line in the logfile should look like
    let file_config = fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}] {}",
                Utc::now().format("[%Y-%m-%d][%H:%M:%S]"),
                record.target(),
                record.level(),
                message
            ))
        })
        .chain(fern::log_file("biograf.log").unwrap());

    // define how a logging line on the console should look like
    let stdout_config = fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}] {}",
                Utc::now().format("[%Y-%m-%d][%H:%M:%S]"),
                record.target(),
                record.level(),
                message
            ))
        })
        .chain(std::io::stdout());

    // now chain everything together and get ready for actually logging stuff
    base_config
        .chain(file_config)
        .chain(stdout_config)
        .apply()
        .unwrap();
}

fn main() {
    use log::{debug, error, info, trace, warn};

    // get the command line parameters from the user
    let cmd_parameters: Biograf = Biograf::parse();

    // determine the verbosity level based on the occurrences of the flag
    setup_logging(cmd_parameters.verbose);

    trace!("Trace");
    debug!("Debug");
    info!("Info");
    warn!("Warning");
    error!("Error");
}
