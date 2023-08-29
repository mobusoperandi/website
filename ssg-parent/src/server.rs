pub(crate) const LOCALHOST: &str = "localhost";
pub(crate) static LOCAL_DEV_PORT: Lazy<Port> = Lazy::new(|| pick_unused_port().unwrap());

use std::path::PathBuf;

use camino::Utf8PathBuf;
use colored::Colorize;
use once_cell::sync::Lazy;
use portpicker::{pick_unused_port, Port};
use reqwest::Url;

#[allow(clippy::module_name_repetitions)]
pub async fn start_development_web_server(
    launch_browser: bool,
    output_dir: Utf8PathBuf,
) -> std::io::Error {
    let url = Url::parse(&format!("http://{LOCALHOST}:{}", *LOCAL_DEV_PORT)).unwrap();
    let message = format!("\nServer started at {url}\n").blue();
    eprintln!("{message}");

    if launch_browser {
        if let Err(error) = open::that(url.as_str()) {
            return error;
        }
    }

    let Err(error) =
        live_server::listen(LOCALHOST, *LOCAL_DEV_PORT, PathBuf::from(output_dir)).await
    else {
        panic!("success not expected")
    };

    error
}
