/*
 * File: range.rs
 * Project: src
 * Created Date: 15/11/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 15/11/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use pico_sys_dynamic::ps4000::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Range {
    Range10mv,
    Range20mv,
    Range50mv,
    Range100mv,
    Range200mv,
    Range500mv,
    Range1v,
    Range2v,
    Range5v,
    Range10v,
    Range20v,
    Range50v,
    Range100v,
}

impl From<Range> for PS4000_RANGE {
    fn from(value: Range) -> Self {
        match value {
            Range::Range10mv => enPS4000Range_PS4000_10MV,
            Range::Range20mv => enPS4000Range_PS4000_20MV,
            Range::Range50mv => enPS4000Range_PS4000_50MV,
            Range::Range100mv => enPS4000Range_PS4000_100MV,
            Range::Range200mv => enPS4000Range_PS4000_200MV,
            Range::Range500mv => enPS4000Range_PS4000_500MV,
            Range::Range1v => enPS4000Range_PS4000_1V,
            Range::Range2v => enPS4000Range_PS4000_2V,
            Range::Range5v => enPS4000Range_PS4000_5V,
            Range::Range10v => enPS4000Range_PS4000_10V,
            Range::Range20v => enPS4000Range_PS4000_20V,
            Range::Range50v => enPS4000Range_PS4000_50V,
            Range::Range100v => enPS4000Range_PS4000_100V,
        }
    }
}

impl Range {
    pub fn mv(&self) -> i32 {
        match self {
            Range::Range10mv => 10,
            Range::Range20mv => 20,
            Range::Range50mv => 50,
            Range::Range100mv => 100,
            Range::Range200mv => 200,
            Range::Range500mv => 500,
            Range::Range1v => 1000,
            Range::Range2v => 2000,
            Range::Range5v => 5000,
            Range::Range10v => 10000,
            Range::Range20v => 20000,
            Range::Range50v => 50000,
            Range::Range100v => 100000,
        }
    }
}
