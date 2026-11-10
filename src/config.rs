// SPDX-License-Identifier: MPL-2.0

use cosmic::cosmic_config::{self, CosmicConfigEntry, cosmic_config_derive::CosmicConfigEntry};

#[derive(Debug, Clone, CosmicConfigEntry, Eq, PartialEq)]
#[version = 1]
pub struct Config {
    pub show_cn_calendar: bool,
    pub show_time_second: bool,
    pub show_cn_holiday: bool,
    pub show_24_hours: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            show_cn_calendar: true,
            show_time_second: false,
            show_cn_holiday: true,
            show_24_hours: true,
        }
    }
}
