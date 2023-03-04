pub mod matchbox_webrtc;

pub trait NetworkInterface {
    fn create_room() -> (Self, String)
    where
        Self: Sized;
    fn join_room(room_id: &String) -> Self;
    fn get_room_id(&self) -> &String;

    fn pause_video(&mut self);
    fn play_video(&mut self);
    fn seek_video(&mut self, to: std::time::Duration);

    fn get_next_event(&mut self) -> Option<SyncEvent>;
}

pub enum SyncEvent {
    Play,
    Pause,
    Seek(std::time::Duration),
}
