use futures::executor::block_on;
use sync_vid_network::{matchbox_webrtc::Client, NetworkInterface};

fn main() {
    let room_id = String::from("231");
    let mut client = Client::join_room(&room_id);
    block_on(client.socket.wait_for_peers(1));

    let event = client.get_next_event();
    dbg!(&event);
    let event = client.get_next_event();
    dbg!(&event);
    client.play_video();
}
