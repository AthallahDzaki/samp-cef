use crate::browser::Browser;
use std::sync::Arc;

pub trait AudioHandler {
    fn on_audio_stream_packet(
        self: &Arc<Self>, browser: Browser, stream_id: i32, data: *mut *const f32, frames: i32,
        pts: i64,
    ) {
    }

    fn on_audio_stream_started(
        self: &Arc<Self>, browser: Browser, stream_id: i32, channels: i32, channel_layout: i32,
        sample_rate: i32, frames_per_buffer: i32,
    ) {
    }

    fn on_audio_stream_stopped(self: &Arc<Self>, browser: Browser, stream_id: i32) {}
}
