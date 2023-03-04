use wasm_bindgen::{prelude::*, JsCast};
use web_sys::*;

macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

#[wasm_bindgen(start)]
pub async fn main() -> Result<(), JsValue> {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));

    log!("Hello World!");
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");

    // Manufacture the element we're gonna append
    let val = document.create_element("p")?;
    val.set_text_content(Some("Hello from Rust!"));
    
    let video = document.create_element("video")?;
    video.set_attribute("controls", "")?;

    let a = Closure::<dyn FnMut(JsValue)>::new(move |_event: JsValue| {
        log!("playing vid here");
    });
    let b = Closure::<dyn FnMut(JsValue)>::new(move |_event: JsValue| {
        log!("pause vid here");
    });
    video.add_event_listener_with_callback("play", a.as_ref().unchecked_ref())?;
    video.add_event_listener_with_callback("pause", b.as_ref().unchecked_ref())?;
    body.append_child(&val)?;
    body.append_child(&video)?;

    Ok(())
}
