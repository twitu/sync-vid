use sync_vid_extension::network::{matchbox_webrtc::Client, NetworkInterface};

fn main() {
    let room_id = String::from("231");
    let mut client = Client::join_room(&room_id);
    
    loop {
        if !client.socket.connected_peers().is_empty() {
            break;
        }
    }
    
    client.pause_video();
    client.play_video();
    let event = client.get_next_event();
    dbg!(&event);
}
