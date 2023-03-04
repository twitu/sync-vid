pub(crate) mod youtube;
use web_sys::HtmlMediaElement;

pub fn get_player() -> Result<impl PlayerInterface, String> {
    let window_url = web_sys::window()
        .ok_or("Global window does not exist".to_owned())
        .map(|w| w.location())
        .and_then(|l| {
            l.host().map_err(|e| {
                e.as_string()
                    .unwrap_or("Can't get host url from location".to_owned())
            })
        })?;

    match window_url.as_ref() {
        "www.youtube.com" => Ok(youtube::YouTube::new()),
        _ => Err(String::from("Unsupported Player")),
    }
}

pub trait PlayerInterface {
    // Get the video element being controlled by the player
    // Assumes there will be only one video on the page for now
    // the initialization method can be extended with more parameters for user controls
    fn get_video(&self) -> Option<&HtmlMediaElement>;

    fn initialize(&mut self) -> Result<(), String>;
}
