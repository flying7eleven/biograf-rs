use lazy_static::lazy_static;
use google_youtube3::{YouTube, oauth2, hyper, hyper_rustls, chrono, Result, Error};
use log::{debug, error, info};

const GOOGLE_CLIENT_INFORMATION_STR: &str = include_str!("../client_secret.json");
lazy_static! {
    static ref GOOGLE_CLIENT_INFORMATION: google_youtube3::oauth2::ConsoleApplicationSecret =
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

#[tokio::main]
async fn main() -> Result<()> {
    setup_logging(2);

    let auth = oauth2::InstalledFlowAuthenticator::builder(
        GOOGLE_CLIENT_INFORMATION.installed.clone().unwrap(),
        oauth2::InstalledFlowReturnMethod::HTTPRedirect,
    ).build().await.unwrap();
    let hub = YouTube::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);

    let result = hub.playlists().list(&vec![]).mine(true).add_part( "snippet").doit().await;

    match result {
        Err(e) => match e {
            Error::HttpError(_)
            | Error::Io(_)
            | Error::MissingAPIKey
            | Error::MissingToken(_)
            | Error::Cancelled
            | Error::UploadSizeLimitExceeded(_, _)
            | Error::Failure(_)
            | Error::BadRequest(_)
            | Error::FieldClash(_)
            | Error::JsonDecodeError(_, _) => error!("{}", e),
        },
        Ok((_, res)) =>{

            for current_list in res.items.unwrap() {

                info!("{}", current_list.snippet.unwrap().title.unwrap());

            }
        }
    }

    Ok(())
}
