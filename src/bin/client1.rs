use sync_vid_network::{matchbox_webrtc::Client, NetworkInterface};
use futures::executor::block_on;

fn main() {
    let room_id = String::from("231");
    let mut client = Client::join_room(&room_id);
    block_on(client.socket.wait_for_peers(1));

    client.pause_video();
    client.play_video();
    let event = client.get_next_event();
    dbg!(&event);
}
