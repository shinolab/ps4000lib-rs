use std::sync::OnceLock;

mod block_data;
mod channel;
mod ps4000;
mod range;
mod trigger;

static LIBRARY: OnceLock<pico_sys_dynamic::ps4000::PS4000Loader> = OnceLock::new();

pub use channel::Channel;
pub use ps4000::PS4262;
pub use range::Range;
pub use trigger::{ThresholdDirection, Trigger};

#[macro_export]
macro_rules! check_pico_status {
    ($x:expr) => {{
        let status = $x.into();
        if status != PicoStatus::OK {
            return Err(status);
        }
    }};
}
