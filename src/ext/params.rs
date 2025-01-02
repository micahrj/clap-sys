use crate::{cstr, events::*, host::*, id::*, plugin::*, string_sizes::*};

use std::ffi::{c_char, c_void, CStr};

pub const CLAP_EXT_PARAMS: &CStr = cstr!("clap.params");

pub const CLAP_PARAM_IS_STEPPED: clap_param_info_flags = 1 << 0;
pub const CLAP_PARAM_IS_PERIODIC: clap_param_info_flags = 1 << 1;
pub const CLAP_PARAM_IS_HIDDEN: clap_param_info_flags = 1 << 2;
pub const CLAP_PARAM_IS_READONLY: clap_param_info_flags = 1 << 3;
pub const CLAP_PARAM_IS_BYPASS: clap_param_info_flags = 1 << 4;
pub const CLAP_PARAM_IS_AUTOMATABLE: clap_param_info_flags = 1 << 5;
pub const CLAP_PARAM_IS_AUTOMATABLE_PER_NOTE_ID: clap_param_info_flags = 1 << 6;
pub const CLAP_PARAM_IS_AUTOMATABLE_PER_KEY: clap_param_info_flags = 1 << 7;
pub const CLAP_PARAM_IS_AUTOMATABLE_PER_CHANNEL: clap_param_info_flags = 1 << 8;
pub const CLAP_PARAM_IS_AUTOMATABLE_PER_PORT: clap_param_info_flags = 1 << 9;
pub const CLAP_PARAM_IS_MODULATABLE: clap_param_info_flags = 1 << 10;
pub const CLAP_PARAM_IS_MODULATABLE_PER_NOTE_ID: clap_param_info_flags = 1 << 11;
pub const CLAP_PARAM_IS_MODULATABLE_PER_KEY: clap_param_info_flags = 1 << 12;
pub const CLAP_PARAM_IS_MODULATABLE_PER_CHANNEL: clap_param_info_flags = 1 << 13;
pub const CLAP_PARAM_IS_MODULATABLE_PER_PORT: clap_param_info_flags = 1 << 14;
pub const CLAP_PARAM_REQUIRES_PROCESS: clap_param_info_flags = 1 << 15;
pub const CLAP_PARAM_IS_ENUM: clap_param_info_flags = 1 << 16;

pub type clap_param_info_flags = u32;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct clap_param_info {
    pub id: clap_id,
    pub flags: clap_param_info_flags,
    pub cookie: *mut c_void,
    pub name: [c_char; CLAP_NAME_SIZE],
    pub module: [c_char; CLAP_PATH_SIZE],
    pub min_value: f64,
    pub max_value: f64,
    pub default_value: f64,
}

unsafe impl Send for clap_param_info {}
unsafe impl Sync for clap_param_info {}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct clap_plugin_params {
    pub count: Option<unsafe extern "C" fn(plugin: *const clap_plugin) -> u32>,
    pub get_info: Option<
        unsafe extern "C" fn(
            plugin: *const clap_plugin,
            param_index: u32,
            param_info: *mut clap_param_info,
        ) -> bool,
    >,
    pub get_value: Option<
        unsafe extern "C" fn(
            plugin: *const clap_plugin,
            param_id: clap_id,
            out_value: *mut f64,
        ) -> bool,
    >,
    pub value_to_text: Option<
        unsafe extern "C" fn(
            plugin: *const clap_plugin,
            param_id: clap_id,
            value: f64,
            out_buffer: *mut c_char,
            out_buffer_capacity: u32,
        ) -> bool,
    >,
    pub text_to_value: Option<
        unsafe extern "C" fn(
            plugin: *const clap_plugin,
            param_id: clap_id,
            param_value_text: *const c_char,
            out_value: *mut f64,
        ) -> bool,
    >,
    pub flush: Option<
        unsafe extern "C" fn(
            plugin: *const clap_plugin,
            in_: *const clap_input_events,
            out: *const clap_output_events,
        ),
    >,
}

pub const CLAP_PARAM_RESCAN_VALUES: clap_param_rescan_flags = 1 << 0;
pub const CLAP_PARAM_RESCAN_TEXT: clap_param_rescan_flags = 1 << 1;
pub const CLAP_PARAM_RESCAN_INFO: clap_param_rescan_flags = 1 << 2;
pub const CLAP_PARAM_RESCAN_ALL: clap_param_rescan_flags = 1 << 3;

pub type clap_param_rescan_flags = u32;

pub const CLAP_PARAM_CLEAR_ALL: clap_param_clear_flags = 1 << 0;
pub const CLAP_PARAM_CLEAR_AUTOMATIONS: clap_param_clear_flags = 1 << 1;
pub const CLAP_PARAM_CLEAR_MODULATIONS: clap_param_clear_flags = 1 << 2;

pub type clap_param_clear_flags = u32;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct clap_host_params {
    pub rescan:
        Option<unsafe extern "C" fn(host: *const clap_host, flags: clap_param_rescan_flags)>,
    pub clear: Option<
        unsafe extern "C" fn(
            host: *const clap_host,
            param_id: clap_id,
            flags: clap_param_clear_flags,
        ),
    >,
    pub request_flush: Option<unsafe extern "C" fn(host: *const clap_host)>,
}
