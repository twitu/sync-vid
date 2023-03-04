use futures::{
    channel::oneshot,
    executor::LocalPool,
    select,
    task::{LocalSpawn, LocalSpawnExt},
    FutureExt,
};
use matchbox_socket::WebRtcSocket;
use rand::{distributions::Alphanumeric, Rng};
use std::{future::Future, pin::Pin, time::Duration};

use super::{NetworkInterface, SyncEvent}; // 0.8

pub struct Client {
    room_id: String,
    url: String,
    socket: WebRtcSocket,
    shutdown_receiver: oneshot::Receiver<bool>,
    local_pool: LocalPool,
}

impl NetworkInterface for Client {
    fn create_room() -> (Self, String) {
        let room_id = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(7)
            .map(char::from)
            .collect();

        (Self::join_room(&room_id), room_id)
    }
    fn join_room(room_id: &String) -> Self {
        let url = format!("ws://{}/{}", "localhost:3536", room_id);

        let (socket, loop_fut) = WebRtcSocket::new(&url);

        let (sender, shutdown_receiver) = oneshot::channel();

        let local_pool = LocalPool::new();
        local_pool
            .spawner()
            .spawn_local(async {
                loop_fut.await;
                sender.send(true).expect("Sending shutdown signal failed");
            })
            .expect("Spawn local future failed");

        Client {
            room_id: room_id.to_owned(),
            url,
            socket,
            shutdown_receiver,
            local_pool,
        }
    }
    fn get_room_id(&self) -> &String {
        &self.room_id
    }

    fn pause_video(&mut self) {
        let data = "pause".to_owned().into_boxed_str().into_boxed_bytes();
        for i in self.socket.connected_peers() {
            self.socket.send(data.clone(), i);
        }
    }
    fn play_video(&mut self) {
        let data = "play".to_owned().into_boxed_str().into_boxed_bytes();
        for i in self.socket.connected_peers() {
            self.socket.send(data.clone(), i);
        }
    }
    fn seek_video(&mut self, _to: std::time::Duration) {
        let data = "seek_video".to_owned().into_boxed_str().into_boxed_bytes();
        for i in self.socket.connected_peers() {
            self.socket.send(data.clone(), i);
        }
    }

    fn get_next_event(&mut self) -> Option<SyncEvent> {
        self.local_pool.run_until_stalled();
        if self.shutdown_receiver.try_recv() == Ok(Some(true)) {
            return None;
        }

        self.socket
            .receive()
            .iter()
            .map(|(peer, data)| {
                web_sys::console::log_1(
                    &format!("Received Data Results: {:?} from {}", &data, &peer).into(),
                );
                (peer, data)
            })
            .last()
            .map(|(peer, data)| {
                (
                    peer,
                    match std::str::from_utf8(&data).unwrap() {
                        "play" => SyncEvent::Play,
                        "pause" => SyncEvent::Pause,
                        "seek_video" => SyncEvent::Seek(Duration::ZERO),
                        _ => todo!("Unhandled event"),
                    },
                )
            })
            .map(|(_, event)| event)
    }
}
