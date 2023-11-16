/*
 * File: ps4000.rs
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

use std::{
    collections::HashMap,
    ops::{Index, IndexMut},
};

pub use crate::check_pico_status;

use pico_common::PicoStatus;
use pico_sys_dynamic::ps4000::{
    enPS4000Channel_PS4000_CHANNEL_A, enPS4000Channel_PS4000_CHANNEL_B,
    enRatioMode_RATIO_MODE_NONE, enThresholdDirection_NONE, PS4000Loader, PS4262_MAX_VALUE,
};

use crate::{
    attenuation::Attenuation,
    block_data::BlockData,
    channel::{Channel, ChannelConfig},
    range::Range,
    trigger::Trigger,
    LIBRARY,
};

pub struct PS4262 {
    handle: i16,
    channels: [ChannelConfig; 2],
}

impl PS4262 {
    pub const MAX_VALUE: u32 = PS4262_MAX_VALUE;
    pub const MAX_CHANNELS: usize = 2;

    pub fn new() -> Result<Self, PicoStatus> {
        let handle = unsafe {
            let library = LIBRARY.get_or_init(|| PS4000Loader::new("ps4000.dll").unwrap());
            let mut handle = -1;
            check_pico_status!(library.ps4000OpenUnit(&mut handle as _));
            handle
        };

        let pico = Self {
            handle,
            channels: [
                ChannelConfig::new(enPS4000Channel_PS4000_CHANNEL_A, handle),
                ChannelConfig::new(enPS4000Channel_PS4000_CHANNEL_B, handle),
            ],
        };

        pico.channels.iter().for_each(|ch| ch.update().unwrap());

        Ok(pico)
    }

    pub fn close(&mut self) -> Result<(), PicoStatus> {
        unsafe {
            if self.handle < 0 {
                return Ok(());
            }

            let library = LIBRARY.get().unwrap();
            check_pico_status!(library.ps4000CloseUnit(self.handle));
            self.handle = -1;
        }
        Ok(())
    }

    fn disable_trigger(&self) -> Result<(), PicoStatus> {
        unsafe {
            let library = LIBRARY.get().unwrap();
            check_pico_status!(library.ps4000SetSimpleTrigger(
                self.handle,
                0,
                Channel::A.into(),
                0,
                enThresholdDirection_NONE,
                0,
                0
            ));
            check_pico_status!(library.ps4000SetSimpleTrigger(
                self.handle,
                0,
                Channel::B.into(),
                0,
                enThresholdDirection_NONE,
                0,
                0
            ));
        }
        Ok(())
    }

    fn set_trigger(&self, cond: Trigger) -> Result<(), PicoStatus> {
        unsafe {
            let library = LIBRARY.get().unwrap();

            check_pico_status!(library.ps4000SetSimpleTrigger(
                self.handle,
                1,
                cond.channel.into(),
                Self::convert_mv_to_adc(
                    cond.value_mv,
                    self[cond.channel].attenuation,
                    self[cond.channel].range
                ),
                cond.dir.into(),
                cond.delay,
                cond.auto_trigger_ms,
            ));
        }

        Ok(())
    }

    fn block_data_handler(
        &mut self,
        buffer_size: u32,
        timebase: u32,
        no_of_pre_trigger_samples: i32,
    ) -> Result<BlockData, PicoStatus> {
        let mut timebase = timebase;
        let mut buffer_size = buffer_size;
        let sample_count = buffer_size as i32;
        let mut min_pinned = HashMap::new();
        min_pinned.insert(self[Channel::A].channel, vec![0i16; buffer_size as _]);
        min_pinned.insert(self[Channel::B].channel, vec![0i16; buffer_size as _]);
        let mut max_pinned = HashMap::new();
        max_pinned.insert(self[Channel::A].channel, vec![0i16; buffer_size as _]);
        max_pinned.insert(self[Channel::B].channel, vec![0i16; buffer_size as _]);

        unsafe {
            let library = LIBRARY.get().unwrap();
            check_pico_status!(library.ps4000SetDataBuffers(
                self.handle,
                self[Channel::A].channel,
                max_pinned
                    .get_mut(&self[Channel::A].channel)
                    .unwrap()
                    .as_mut_ptr(),
                min_pinned
                    .get_mut(&self[Channel::A].channel)
                    .unwrap()
                    .as_mut_ptr(),
                sample_count,
            ));
            check_pico_status!(library.ps4000SetDataBuffers(
                self.handle,
                self[Channel::B].channel,
                max_pinned
                    .get_mut(&self[Channel::B].channel)
                    .unwrap()
                    .as_mut_ptr(),
                min_pinned
                    .get_mut(&self[Channel::B].channel)
                    .unwrap()
                    .as_mut_ptr(),
                sample_count,
            ));

            let mut time_interval_nanoseconds = 0i32;
            let mut max_samples = 0;

            while library.ps4000GetTimebase(
                self.handle,
                timebase,
                sample_count,
                &mut time_interval_nanoseconds as _,
                1,
                &mut max_samples as _,
                0,
            ) != 0
            {
                timebase += 1;
            }

            let mut time_in_disposed_ms = 0;
            check_pico_status!(library.ps4000RunBlock(
                self.handle,
                no_of_pre_trigger_samples,
                sample_count - no_of_pre_trigger_samples,
                timebase,
                1,
                &mut time_in_disposed_ms as _,
                0,
                None,
                std::ptr::null_mut(),
            ));

            loop {
                let mut ready = 0i16;
                check_pico_status!(library.ps4000IsReady(self.handle, &mut ready as _));
                if ready != 0 {
                    break;
                }
            }

            check_pico_status!(library.ps4000Stop(self.handle));

            let mut overflow = 0;
            check_pico_status!(library.ps4000GetValues(
                self.handle,
                0,
                &mut buffer_size as _,
                1,
                enRatioMode_RATIO_MODE_NONE as _,
                0,
                &mut overflow,
            ));

            Ok(BlockData::new(
                buffer_size,
                overflow,
                time_interval_nanoseconds,
                min_pinned,
                max_pinned,
                self.channels
                    .iter()
                    .map(|ch| (ch.channel, ch.range))
                    .collect(),
                self.channels
                    .iter()
                    .map(|ch| (ch.channel, ch.attenuation))
                    .collect(),
            ))
        }
    }

    pub fn collect_block_immediate(
        &mut self,
        sample_count: u32,
        sample_rate: u32,
    ) -> Result<BlockData, PicoStatus> {
        self.channels.iter_mut().try_for_each(|ch| ch.update())?;
        self.disable_trigger()?;
        let timebase = 10000000 / sample_rate - 1;
        self.block_data_handler(sample_count, timebase, 0)
    }

    pub fn collect_block_triggered(
        &mut self,
        sample_count: u32,
        sample_rate: u32,
        cond: Trigger,
    ) -> Result<BlockData, PicoStatus> {
        self.channels.iter_mut().try_for_each(|ch| ch.update())?;
        self.set_trigger(cond)?;
        let timebase = 10000000 / sample_rate - 1;
        self.block_data_handler(sample_count, timebase, cond.no_of_pre_trigger_samples)
    }

    pub(crate) fn convert_adc_to_mv(raw: i16, attenuation: Attenuation, range: Range) -> f64 {
        (raw as i32 * range.mv(attenuation).unwrap()) as f64 / Self::MAX_VALUE as f64
            * attenuation.value()
    }

    pub(crate) fn convert_mv_to_adc(raw: f64, attenuation: Attenuation, range: Range) -> i16 {
        (raw / attenuation.value() * Self::MAX_VALUE as f64 / range.mv(attenuation).unwrap() as f64)
            as i16
    }
}

impl Index<Channel> for PS4262 {
    type Output = ChannelConfig;

    fn index(&self, ch: Channel) -> &Self::Output {
        match ch {
            Channel::A => &self.channels[0],
            Channel::B => &self.channels[1],
        }
    }
}

impl IndexMut<Channel> for PS4262 {
    fn index_mut(&mut self, ch: Channel) -> &mut Self::Output {
        match ch {
            Channel::A => &mut self.channels[0],
            Channel::B => &mut self.channels[1],
        }
    }
}
