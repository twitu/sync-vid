use wasm_bindgen::{prelude::*, JsCast};
use web_sys::*;

use crate::players::player_interface::PlayerInterface;
pub mod players;

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
    let _body = document.body().expect("document should have a body");

    let mut player = players::get_player()?;
 
    player.initialize()?;
    let onpause = Closure::<dyn FnMut(Event)>::new(move |event: Event| {
        let video = HtmlVideoElement::from(JsValue::from(event.target().unwrap()));
        log!("We're paused at {}", video.current_time());
 
    });
    let onplay = Closure::<dyn FnMut(Event)>::new(move |event: Event| {
        let video = HtmlVideoElement::from(JsValue::from(event.target().unwrap()));
        log!("We're playing at {}", video.current_time());
 
    });
    console::log_2(&"Player video".into(), player.get_video().unwrap().as_ref());
 
    player.get_video().unwrap().set_onpause(Some(onpause.as_ref().unchecked_ref()));
    
    player.get_video().unwrap().set_onplay(Some(onplay.as_ref().unchecked_ref()));
 
    onpause.forget();
    onplay.forget();
    
    Ok(())
}
