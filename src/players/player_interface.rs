use web_sys::HtmlVideoElement;

pub trait PlayerInterface {

    // Get the video element being controlled by the player
    // Assumes there will be only one video on the page for now
    // the initialization method can be extended with more parameters for user controls
    fn get_video(&self) -> Option<&HtmlVideoElement>;

    fn initialize(&mut self) -> Result<(), String>;
}