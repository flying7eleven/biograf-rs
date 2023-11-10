use dialoguer::theme::ColorfulTheme;
use dialoguer::Select;
use google_youtube3::{chrono, hyper, hyper_rustls, oauth2, Result, YouTube};
use lazy_static::lazy_static;

const GOOGLE_CLIENT_INFORMATION_STR: &str = include_str!("../client_secret.json");
lazy_static! {
    static ref GOOGLE_CLIENT_INFORMATION: google_youtube3::oauth2::ConsoleApplicationSecret =
        serde_json::from_str(&GOOGLE_CLIENT_INFORMATION_STR).unwrap();
}

fn setup_logging() {
    use chrono::Utc;

    // create an instance for the Dispatcher to create a new logging configuration
    let mut base_config = fern::Dispatch::new();
    base_config = base_config.level(log::LevelFilter::Debug);

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

    // now chain everything together and get ready for actually logging stuff
    base_config.chain(file_config).apply().unwrap();
}

#[tokio::main]
async fn main() -> Result<()> {
    // setup the basic logging we need
    setup_logging();

    // get an authenticated API client for the YouTube API to deal with the playlists
    let auth = oauth2::InstalledFlowAuthenticator::builder(
        GOOGLE_CLIENT_INFORMATION.installed.clone().unwrap(),
        oauth2::InstalledFlowReturnMethod::Interactive,
    )
    .persist_tokens_to_disk("token.json")
    .build()
    .await
    .unwrap();
    let hub = YouTube::new(
        hyper::Client::builder().build(
            hyper_rustls::HttpsConnectorBuilder::new()
                .with_native_roots()
                .https_or_http()
                .enable_http1()
                .build(),
        ),
        auth,
    );

    // get the playlists the user has configured to ask which playlist should be processed
    let playlists = match hub
        .playlists()
        .list(&vec![])
        .mine(true)
        .add_part("snippet")
        .doit()
        .await
    {
        Err(e) => panic!("{}", e),
        Ok((_, res)) => res.items.unwrap(),
    };
    let playlist_titles = playlists
        .iter()
        .map(|item| {
            item.snippet
                .as_ref()
                .unwrap()
                .title
                .as_ref()
                .unwrap()
                .clone()
        })
        .collect::<Vec<String>>();

    // ask the user which playlist should be processed
    let selected_playlist = playlists
        .get(
            Select::with_theme(&ColorfulTheme::default())
                .with_prompt("Pick the playlist you want to clean up:")
                .default(0)
                .items(&playlist_titles[..])
                .interact()
                .unwrap(),
        )
        .unwrap();

    // get the items from the playlist
    let items_in_playlist = match hub
        .playlist_items()
        .list(&vec![])
        .add_part("snippet")
        .add_part("status")
        .playlist_id(selected_playlist.id.as_ref().unwrap().as_str())
        .doit()
        .await
    {
        Err(e) => panic!("{}", e),
        Ok((_, res)) => res.items.unwrap(),
    };

    // loop through all videos with the corresponding video id
    for current_item in items_in_playlist {
        if current_item
            .status
            .unwrap()
            .privacy_status
            .unwrap()
            .eq("public")
        {
            println!(
                "{} -> {:?}",
                current_item
                    .snippet
                    .as_ref()
                    .unwrap()
                    .title
                    .as_ref()
                    .unwrap(),
                current_item
                    .snippet
                    .as_ref()
                    .unwrap()
                    .resource_id
                    .as_ref()
                    .unwrap()
                    .video_id
                    .as_ref()
                    .unwrap()
            );
        }
    }

    Ok(())
}
