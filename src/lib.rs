pub mod channel;
pub mod filter_config;
pub mod freq_conf;
pub mod group_call;
pub mod volume_config;
use crate::channel::Channel;
use crate::filter_config::FilterConfig;
use crate::volume_config::VolumeConfig;
enum TailTone {
    Open,
    Close,
}

struct Sa818Config {
    channel_conf: Option<Channel>,
    filter_conf: Option<FilterConfig>,
    tail_conf: Option<TailTone>,
    volume_conf: Option<VolumeConfig>,
}
