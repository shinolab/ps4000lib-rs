/*
 * File: block_data.rs
 * Project: src
 * Created Date: 14/11/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 15/11/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use std::collections::HashMap;

use pico_sys_dynamic::ps4000::PS4000_CHANNEL;

use crate::{ps4000::PS4262, range::Range, Channel};

pub struct BlockData {
    sample_count: usize,
    time_interval: i32,
    _min_pinned: HashMap<PS4000_CHANNEL, Vec<i16>>,
    max_pinned: HashMap<PS4000_CHANNEL, Vec<i16>>,
    range: HashMap<PS4000_CHANNEL, Range>,
    attenuation: HashMap<PS4000_CHANNEL, i32>,
}

impl BlockData {
    pub fn new(
        sample_count: u32,
        time_interval: i32,
        min_pinned: HashMap<PS4000_CHANNEL, Vec<i16>>,
        max_pinned: HashMap<PS4000_CHANNEL, Vec<i16>>,
        range: HashMap<PS4000_CHANNEL, Range>,
        attenuation: HashMap<PS4000_CHANNEL, i32>,
    ) -> Self {
        Self {
            sample_count: sample_count as usize,
            time_interval,
            _min_pinned: min_pinned,
            max_pinned,
            range,
            attenuation,
        }
    }

    pub fn time_interval(&self) -> std::time::Duration {
        std::time::Duration::from_nanos(self.time_interval as _)
    }

    pub fn get_adc(&self, channel: Channel) -> Vec<i16> {
        self.max_pinned[&channel.into()]
            .iter()
            .take(self.sample_count)
            .copied()
            .collect()
    }

    pub fn get_mv(&self, channel: Channel) -> Vec<f64> {
        let channel = channel.into();
        let range = self.range[&channel];
        let atten = self.attenuation[&channel];
        self.max_pinned[&channel]
            .iter()
            .take(self.sample_count)
            .map(|&x| PS4262::convert_adc_to_mv(x, range) * atten as f64)
            .collect()
    }
}
