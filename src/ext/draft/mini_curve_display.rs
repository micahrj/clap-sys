use crate::{cstr, host::clap_host, plugin::clap_plugin};

use core::ffi::{c_char, CStr};

pub const CLAP_EXT_MINI_CURVE_DISPLAY: &CStr = cstr!("clap.mini-curve-display/1");

pub const CLAP_MINI_CURVE_DISPLAY_CURVE_CHANGED: clap_mini_curve_display_change_flags = 1 << 0;
pub const CLAP_MINI_CURVE_DISPLAY_AXIS_NAME_CHANGED: clap_mini_curve_display_change_flags = 1 << 1;

pub type clap_mini_curve_display_change_flags = u32;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct clap_plugin_mini_curve_display {
    pub render: Option<
        unsafe extern "C" fn(plugin: *const clap_plugin, data: *mut u16, data_size: u32) -> bool,
    >,
    pub set_observed: Option<unsafe extern "C" fn(plugin: *const clap_plugin, is_observed: bool)>,
    pub get_axis_name: Option<
        unsafe extern "C" fn(
            plugin: *const clap_plugin,
            x_name: *mut c_char,
            y_name: *mut c_char,
            name_capacity: u32,
        ) -> bool,
    >,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct clap_host_mini_curve_display {
    pub set_dynamic: Option<unsafe extern "C" fn(host: *const clap_host, is_dynamic: bool)>,
    pub curve_changed: Option<unsafe extern "C" fn(host: *const clap_host)>,
    pub changed: Option<
        unsafe extern "C" fn(host: *const clap_host, flags: clap_mini_curve_display_change_flags),
    >,
}
