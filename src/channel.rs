/*
 * File: channel.rs
 * Project: src
 * Created Date: 14/11/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 16/11/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use pico_common::{PicoCoupling, PicoStatus};
use pico_sys_dynamic::ps4000::{
    enPS4000Channel_PS4000_CHANNEL_A, enPS4000Channel_PS4000_CHANNEL_B, PS4000_CHANNEL,
};

use crate::{attenuation::Attenuation, check_pico_status, range::Range, LIBRARY};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Channel {
    A,
    B,
}

impl From<Channel> for PS4000_CHANNEL {
    fn from(value: Channel) -> Self {
        match value {
            Channel::A => enPS4000Channel_PS4000_CHANNEL_A,
            Channel::B => enPS4000Channel_PS4000_CHANNEL_B,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ChannelConfig {
    handle: i16,
    pub enable: bool,
    pub(crate) channel: PS4000_CHANNEL,
    pub coupling: PicoCoupling,
    pub range: Range,
    pub attenuation: Attenuation,
}

impl ChannelConfig {
    pub(crate) fn update(&self) -> Result<(), PicoStatus> {
        unsafe {
            let loader = LIBRARY.get().unwrap();
            check_pico_status!(loader.ps4000SetChannel(
                self.handle,
                self.channel,
                if self.enable { 1 } else { 0 },
                self.coupling.into(),
                self.range.into_range(self.attenuation)?,
            ));
            Ok(())
        }
    }

    pub(crate) fn new(channel: PS4000_CHANNEL, handle: i16) -> Self {
        Self {
            handle,
            enable: true,
            channel,
            coupling: PicoCoupling::DC,
            range: Range::Range5v,
            attenuation: Attenuation::X1,
        }
    }
}
