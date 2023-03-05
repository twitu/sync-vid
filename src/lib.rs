use std::sync::{Arc, Mutex};

use once_cell::sync::OnceCell;
use wasm_bindgen::{prelude::*, JsCast};
use web_sys::*;

use crate::{players::PlayerInterface, network::{NetworkInterface, SyncEvent}};
pub mod network;
pub mod players;

macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

/// A Video has a sequence number of events it has seen and it's current
/// time. A video op is applied to a video based on CRDT rules so that
/// events so that concurrent updates and can be merged without conflict.
///
/// It also has the dom element for the video player
#[derive(Debug)]
struct VideoPlayer {
    id: u32,
    seq: u32,
    time: u32,
    playing: bool,
}

impl Default for VideoPlayer {
    fn default() -> Self {
        Self {
            id: 0,
            seq: 0,
            time: 0,
            playing: false,
        }
    }
}

impl VideoPlayer {
    fn play(&mut self) -> Result<(), JsValue> {
        let mut player = players::get_player()?;
        player.initialize()?;
        let video_element = player.get_video().expect("Valid video player");
        let current_time = video_element.current_time();
        video_element.play()?;
        Ok(())
    }

    fn pause(&mut self) -> Result<(), JsValue> {
        let mut player = players::get_player()?;
        player.initialize()?;
        let video_element = player.get_video().expect("Valid video player");
        let current_time = video_element.current_time();
        video_element.pause()?;
        Ok(())
    }

    fn seek(&mut self, to_time: f64) -> Result<(), JsValue> {
        let mut player = players::get_player()?;
        player.initialize()?;
        let video_element = player.get_video().expect("Valid video player");
        video_element.fast_seek(to_time)?;
        Ok(())
    }
}

static mut VIDEO_ELEMENT: OnceCell<VideoPlayer> = OnceCell::new();

/// HtmlMediaElement internally uses raw pointers.
/// However wasm applications are currently run on a single thread
/// so we can skip this problem and implement Sync and Send for it.
unsafe impl Sync for VideoPlayer {}
unsafe impl Send for VideoPlayer {}

#[wasm_bindgen(start)]
pub async fn main() -> Result<(), JsValue> {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));

    log!("Hello World!");
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let _body = document.body().expect("document should have a body");

    let player = Arc::new(Mutex::new(players::get_player()?));

    let room_id = String::from("231");

    let network = network::matchbox_webrtc::Client::join_room(&room_id);
    let rtc_client = Arc::new(Mutex::new(network));

    log!("Created a room with id {}", room_id);

    rtc_client.lock().map(|mut i| {
        log!("Polling RTC {:?}", i.get_next_event());
    }).map_err(|er| {
        log!("Failed to lock mutex {:?}", er);
    }).ok().unwrap();

    // window.set_interval_with_callback_and_timeout_and_arguments_0(handler, 100);

    let rtc_loop = rtc_client.clone();
    let player_loop = player.clone();
    let loop_handler = Closure::<dyn Fn()>::new(move || {
        if let (Ok(mut client), Ok(player)) = (rtc_loop.lock(), player_loop.lock()) {
            if let Some(event) = client.get_next_event() {
                match (event, player.get_video()) {
                    (SyncEvent::Play, Some(i)) => i.pause(),
                    (SyncEvent::Pause, Some(i)) => i.play().map(|i| ()),
                    (SyncEvent::Seek(d), Some(i)) => Ok(i.set_current_time(d.as_secs_f64())),
                    _ => {Ok(())}
                }.map_err(|er| {
                    log!("Error handling event {:?}", er)
                }).unwrap();
            }
        }
    });

    player.lock().unwrap().initialize()?;
    let rtc_pause = rtc_client.clone();
    let onpause = Closure::<dyn FnMut(Event)>::new(move |event: Event| {
        let video = HtmlVideoElement::from(JsValue::from(event.target().unwrap()));
        if let Ok(mut client) = rtc_pause.clone().lock() {
            client.pause_video();
        } else {
            log!("Failed to acquire lock for pausing video");
        }
        log!("We're paused at {}", video.current_time());
    });
    let rtc_play = rtc_client.clone();
    let onplay = Closure::<dyn FnMut(Event)>::new(move |event: Event| {
        let video = HtmlVideoElement::from(JsValue::from(event.target().unwrap()));
        if let Ok(mut client) = rtc_play.clone().lock() {
            client.play_video();
        } else {
            log!("Failed to acquire lock for playing video");
        }
        log!("We're playing at {}", video.current_time());
    });
    console::log_2(&"Player video".into(), player.lock().unwrap().get_video().unwrap().as_ref());

    player
        .lock()
        .unwrap()
        .get_video()
        .unwrap()
        .set_onpause(Some(onpause.as_ref().unchecked_ref()));

    player
        .lock()
        .unwrap()
        .get_video()
        .unwrap()
        .set_onplay(Some(onplay.as_ref().unchecked_ref()));

    let interval_id = window.set_interval_with_callback_and_timeout_and_arguments_0(loop_handler.as_ref().unchecked_ref(), 100).unwrap();

    log!("Interval ID {}", interval_id);

    onpause.forget();
    onplay.forget();
    loop_handler.forget();
    unsafe { VIDEO_ELEMENT.set(VideoPlayer::default()).unwrap() };
    Ok(())
}
