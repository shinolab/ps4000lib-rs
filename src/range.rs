/*
 * File: range.rs
 * Project: src
 * Created Date: 15/11/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 16/11/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use pico_common::PicoStatus;
use pico_sys_dynamic::ps4000::*;

use crate::attenuation::Attenuation;

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

impl Range {
    pub(crate) fn into_range(&self, attenuation: Attenuation) -> Result<enPS4000Range, PicoStatus> {
        match attenuation {
            Attenuation::X1 => match self {
                Range::Range10mv => Ok(enPS4000Range_PS4000_10MV),
                Range::Range20mv => Ok(enPS4000Range_PS4000_20MV),
                Range::Range50mv => Ok(enPS4000Range_PS4000_50MV),
                Range::Range100mv => Ok(enPS4000Range_PS4000_100MV),
                Range::Range200mv => Ok(enPS4000Range_PS4000_200MV),
                Range::Range500mv => Ok(enPS4000Range_PS4000_500MV),
                Range::Range1v => Ok(enPS4000Range_PS4000_1V),
                Range::Range2v => Ok(enPS4000Range_PS4000_2V),
                Range::Range5v => Ok(enPS4000Range_PS4000_5V),
                Range::Range10v => Ok(enPS4000Range_PS4000_10V),
                Range::Range20v => Ok(enPS4000Range_PS4000_20V),
                Range::Range50v => Ok(enPS4000Range_PS4000_50V),
                Range::Range100v => Ok(enPS4000Range_PS4000_100V),
            },
            Attenuation::X10 => match self {
                Range::Range10mv | Range::Range20mv | Range::Range50mv => {
                    Err(PicoStatus::INVALID_VOLTAGE_RANGE)
                }
                Range::Range100mv => Ok(enPS4000Range_PS4000_10MV),
                Range::Range200mv => Ok(enPS4000Range_PS4000_20MV),
                Range::Range500mv => Ok(enPS4000Range_PS4000_50MV),
                Range::Range1v => Ok(enPS4000Range_PS4000_100MV),
                Range::Range2v => Ok(enPS4000Range_PS4000_200MV),
                Range::Range5v => Ok(enPS4000Range_PS4000_500MV),
                Range::Range10v => Ok(enPS4000Range_PS4000_1V),
                Range::Range20v => Ok(enPS4000Range_PS4000_2V),
                Range::Range50v => Ok(enPS4000Range_PS4000_5V),
                Range::Range100v => Ok(enPS4000Range_PS4000_10V),
            },
        }
    }
}

impl Range {
    #[allow(non_upper_case_globals)]
    pub fn mv(&self, attenuation: Attenuation) -> Result<i32, PicoStatus> {
        match self.into_range(attenuation)? {
            enPS4000Range_PS4000_10MV => Ok(10),
            enPS4000Range_PS4000_20MV => Ok(20),
            enPS4000Range_PS4000_50MV => Ok(50),
            enPS4000Range_PS4000_100MV => Ok(100),
            enPS4000Range_PS4000_200MV => Ok(200),
            enPS4000Range_PS4000_500MV => Ok(500),
            enPS4000Range_PS4000_1V => Ok(1000),
            enPS4000Range_PS4000_2V => Ok(2000),
            enPS4000Range_PS4000_5V => Ok(5000),
            enPS4000Range_PS4000_10V => Ok(10_000),
            enPS4000Range_PS4000_20V => Ok(20_000),
            enPS4000Range_PS4000_50V => Ok(50_000),
            enPS4000Range_PS4000_100V => Ok(100_000),
            _ => unreachable!(),
        }
    }
}
