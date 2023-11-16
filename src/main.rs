/*
 * File: main.rs
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

use ps4000lib::*;

fn main() {
    let mut ps = PS4262::new().unwrap();

    ps[Channel::A].range = Range::Range10v;
    ps[Channel::A].attenuation = Attenuation::X1;
    ps[Channel::B].enable = false;

    let data = ps
        .collect_block_triggered(
            1000,
            10_000_000,
            Trigger::new(Channel::A, ThresholdDirection::Rising, 2000.0),
        )
        .unwrap();

    println!(
        "{:?}",
        data.get_mv(Channel::A).iter().take(50).collect::<Vec<_>>()
    );
    println!("Time interval: {:?}", data.time_interval());
    println!("Overflow: {}", data.overflow());
}
