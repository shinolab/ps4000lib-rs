/*
 * File: trigger.rs
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

use crate::{Channel, Range, PS4262};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ThresholdDirection {
    // Values for level threshold mode
    Above,
    Below,
    Rising,
    Falling,
    RisingOrFalling,

    // Values for window threshold mode
    Inside,
    Outside,
    Enter,
    Exit,
    EnterOrExit,

    None,
}

impl From<ThresholdDirection> for THRESHOLD_DIRECTION {
    fn from(value: ThresholdDirection) -> Self {
        match value {
            ThresholdDirection::Above => enThresholdDirection_ABOVE,
            ThresholdDirection::Below => enThresholdDirection_BELOW,
            ThresholdDirection::Rising => enThresholdDirection_RISING,
            ThresholdDirection::Falling => enThresholdDirection_FALLING,
            ThresholdDirection::RisingOrFalling => enThresholdDirection_RISING_OR_FALLING,
            ThresholdDirection::Inside => enThresholdDirection_INSIDE,
            ThresholdDirection::Outside => enThresholdDirection_OUTSIDE,
            ThresholdDirection::Enter => enThresholdDirection_ENTER,
            ThresholdDirection::Exit => enThresholdDirection_EXIT,
            ThresholdDirection::EnterOrExit => enThresholdDirection_ENTER_OR_EXIT,
            ThresholdDirection::None => enThresholdDirection_NONE,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ThresholdMode {
    Level,
    Window,
}

impl From<ThresholdMode> for THRESHOLD_MODE {
    fn from(value: ThresholdMode) -> Self {
        match value {
            ThresholdMode::Level => enThresholdMode_LEVEL,
            ThresholdMode::Window => enThresholdMode_WINDOW,
        }
    }
}

pub enum TriggerState {
    DoNotCare,
    True,
    False,
}

impl From<TriggerState> for TRIGGER_STATE {
    fn from(value: TriggerState) -> Self {
        match value {
            TriggerState::DoNotCare => enTriggerState_CONDITION_DONT_CARE,
            TriggerState::True => enTriggerState_CONDITION_TRUE,
            TriggerState::False => enTriggerState_CONDITION_FALSE,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Trigger {
    pub(crate) channel: Channel,
    value_mv: f64,
    dir: ThresholdDirection,
    pub(crate) no_of_pre_trigger_samples: i32,
    pub(crate) delay: u32,
    pub(crate) auto_trigger_ms: i32,
}

impl Trigger {
    pub fn new(channel: Channel, dir: ThresholdDirection, value_mv: f64) -> Self {
        Self {
            channel,
            dir,
            value_mv,
            delay: 0,
            no_of_pre_trigger_samples: 0,
            auto_trigger_ms: 0,
        }
    }

    pub fn with_no_of_pre_trigger_samples(mut self, no_of_pre_trigger_samples: i32) -> Self {
        self.no_of_pre_trigger_samples = no_of_pre_trigger_samples;
        self
    }

    pub fn with_delay(mut self, delay: u32) -> Self {
        self.delay = delay;
        self
    }

    pub fn with_auto_trigger_ms(mut self, auto_trigger_ms: i32) -> Self {
        self.auto_trigger_ms = auto_trigger_ms;
        self
    }

    pub(crate) fn get_properties(
        &self,
        attenuation: i32,
        range: Range,
    ) -> Vec<tTriggerChannelProperties> {
        let adc = PS4262::convert_mv_to_adc(self.value_mv / attenuation as f64, range);
        let hysterisis = (256 * 10 / attenuation) as _;
        vec![
            tTriggerChannelProperties {
                thresholdUpper: adc,
                thresholdUpperHysteresis: hysterisis,
                thresholdLower: adc,
                thresholdLowerHysteresis: hysterisis,
                channel: Channel::A.into(),
                thresholdMode: ThresholdMode::Level.into(),
            },
            tTriggerChannelProperties {
                thresholdUpper: adc,
                thresholdUpperHysteresis: hysterisis,
                thresholdLower: adc,
                thresholdLowerHysteresis: hysterisis,
                channel: Channel::B.into(),
                thresholdMode: ThresholdMode::Level.into(),
            },
        ]
    }

    pub(crate) fn get_conditions(&self) -> Vec<tTriggerConditions> {
        vec![tTriggerConditions {
            channelA: if self.channel == Channel::A {
                TriggerState::True.into()
            } else {
                TriggerState::DoNotCare.into()
            },
            channelB: if self.channel == Channel::B {
                TriggerState::True.into()
            } else {
                TriggerState::DoNotCare.into()
            },
            channelC: TriggerState::DoNotCare.into(),
            channelD: TriggerState::DoNotCare.into(),
            external: TriggerState::DoNotCare.into(),
            aux: TriggerState::DoNotCare.into(),
            pulseWidthQualifier: TriggerState::DoNotCare.into(),
        }]
    }

    pub(crate) fn get_directions(&self) -> Vec<THRESHOLD_DIRECTION> {
        let ch_a = if self.channel == Channel::A {
            self.dir.into()
        } else {
            ThresholdDirection::None.into()
        };
        let ch_b = if self.channel == Channel::B {
            self.dir.into()
        } else {
            ThresholdDirection::None.into()
        };
        vec![ch_a, ch_b]
    }
}
