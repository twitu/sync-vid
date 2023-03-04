use super::PlayerInterface;

use wasm_bindgen::JsValue;
use web_sys::{console::*, HtmlMediaElement};

#[derive(Debug, Default)]
pub struct YouTube {
    video: Option<HtmlMediaElement>,
}

impl YouTube {
    pub fn new() -> Self {
        log_1(&"Loading Youtube Player".into());
        YouTube::default()
    }
}

impl PlayerInterface for YouTube {
    fn get_video(&self) -> Option<&web_sys::HtmlMediaElement> {
        self.video.as_ref()
    }

    fn initialize(&mut self) -> Result<(), String> {
        web_sys::window()
            .ok_or("Global window does not exist".to_owned())
            .and_then(|w| {
                w.document()
                    .ok_or("Document not found for the window".to_owned())
            })
            .and_then(|doc| {
                doc.query_selector("video.html5-main-video")
                    .map_err(|er| {
                        er.as_string()
                            .unwrap_or("Can't search for videos".to_owned())
                    })
                    .and_then(|res| res.ok_or("Couldn't find any videos".to_owned()))
            })
            .map(|elem| {
                self.video = Some(HtmlMediaElement::from(JsValue::from(elem)));
                ()
            })
    }
}
