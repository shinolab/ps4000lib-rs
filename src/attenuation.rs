/*
 * File: attenuation.rs
 * Project: src
 * Created Date: 16/11/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 16/11/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Attenuation {
    X1,
    X10,
}

impl Attenuation {
    pub(crate) fn value(&self) -> f64 {
        match self {
            Attenuation::X1 => 1.0,
            Attenuation::X10 => 10.0,
        }
    }
}
