mod actor;
mod area;
mod contexted;
#[cfg(target_os = "linux")]
mod paintable;
#[cfg(target_os = "linux")]
mod proxy;

pub use actor::*;
pub use area::*;
pub use contexted::*;
#[cfg(target_os = "linux")]
pub use paintable::*;
#[cfg(target_os = "linux")]
pub use proxy::*;

// The wl-proxy embedder only exists on Linux; elsewhere the render-API
// GL area path is used and arming the proxy is a no-op.
#[cfg(not(target_os = "linux"))]
pub fn arm_mpv_proxy() {}

use flume::{Receiver, Sender, unbounded};
use once_cell::sync::Lazy;

type TimeMillis = f64;

pub enum ListenEvent {
    Seek(TimeMillis),
    PlaybackRestart(TimeMillis),
    Eof(u32),
    StartFile,
    FileLoaded,
    Duration(f64),
    Pause(bool),
    CacheSpeed(i64),
    Error(String),
    TrackList(MpvTracks),
    Volume(i64),
    Speed(f64),
    Shutdown,
    DemuxerCacheTime(i64),
    TimePos(i64),
    PausedForCache(bool, TimeMillis),
    ChapterList(ChapterList),
    Playlist(Playlist),
}

pub struct MPVEventChannel {
    pub tx: Sender<ListenEvent>,
    pub rx: Receiver<ListenEvent>,
}

pub static MPV_EVENT_CHANNEL: Lazy<MPVEventChannel> = Lazy::new(|| {
    let (tx, rx) = unbounded::<ListenEvent>();

    MPVEventChannel { tx, rx }
});

pub struct RenderUpdate {
    pub tx: Sender<bool>,
    pub rx: Receiver<bool>,
}

// Give render update a unique channel
pub static RENDER_UPDATE: Lazy<RenderUpdate> = Lazy::new(|| {
    let (tx, rx) = unbounded::<bool>();

    RenderUpdate { tx, rx }
});
