use crate::{cstr, plugin::*};

use core::ffi::CStr;

pub const CLAP_EXT_GAIN_ADJUSTMENT_METERING: &CStr = cstr!("clap.gain-adjustment-metering/0");

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct clap_plugin_gain_adjustment_metering {
    pub get: Option<unsafe extern "C" fn(plugin: *const clap_plugin) -> f64>,
}
