use super::player_interface::PlayerInterface;

use web_sys::console::*;

pub struct YouTube {

}

impl YouTube {
    pub fn new() -> Self {
        log_1(&"Loading Youtube Player".into());
        YouTube {}
    }
}

impl PlayerInterface for YouTube {}