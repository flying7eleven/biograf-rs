use lazy_static::lazy_static;
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct GoogleClientSecretData {
    pub client_id: String,
    pub project_id: String,
    pub auth_uri: String,
    pub token_uri: String,
    pub auth_provider_x509_cert_url: String,
    pub client_secret: String,
    pub redirect_uris: Vec<String>,
}

#[derive(Serialize, Deserialize)]
struct GoogleClientSecretsRoot {
    pub installed: GoogleClientSecretData,
}

const GOOGLE_CLIENT_INFORMATION_STR: &str = include_str!("../client_secret.json");
lazy_static! {
    static ref GOOGLE_CLIENT_INFORMATION_JSON: GoogleClientSecretsRoot =
        serde_json::from_str(&GOOGLE_CLIENT_INFORMATION_STR).unwrap();
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
    setup_logging(3);
}
