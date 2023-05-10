use std::path::PathBuf;

pub(crate) const LOCALHOST: &str = "localhost";
pub(crate) static LOCAL_DEV_PORT: Lazy<Port> = Lazy::new(|| pick_unused_port().unwrap());

use colored::Colorize;
use once_cell::sync::Lazy;
use portpicker::{pick_unused_port, Port};
use reqwest::Url;

pub async fn start_development_web_server(
    launch_browser: bool,
    output_dir: PathBuf,
) -> Result<(), std::io::Error> {
    let url = Url::parse(&format!("http://{LOCALHOST}:{}", *LOCAL_DEV_PORT)).unwrap();
    let message = format!("\nServer started at {url}\n").blue();
    println!("{message}");

    if launch_browser {
        open::that(url.as_str())?;
    }

    live_server::listen(LOCALHOST, *LOCAL_DEV_PORT, output_dir).await
}
