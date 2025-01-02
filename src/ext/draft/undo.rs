use crate::{cstr, host::*, id::*, plugin::*};

use std::ffi::{c_char, c_void, CStr};

pub const CLAP_EXT_UNDO: &CStr = cstr!("clap.undo/2");

pub const CLAP_UNDO_IS_WITHIN_CHANGE: clap_undo_context_flags = 1 << 0;

pub type clap_undo_context_flags = u64;

pub const CLAP_UNDO_DELTA_PROPERTIES_HAS_DELTA: clap_undo_delta_properties_flags = 1 << 0;
pub const CLAP_UNDO_DELTA_PROPERTIES_IS_PERSISTENT: clap_undo_delta_properties_flags = 1 << 1;

pub type clap_undo_delta_properties_flags = u64;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct clap_undo_delta_properties {
    pub flags: clap_undo_delta_properties_flags,
    pub format_version: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct clap_plugin_undo {
    pub get_delta_properties: Option<
        unsafe extern "C" fn(
            plugin: *const clap_plugin,
            properties: *mut clap_undo_delta_properties,
        ),
    >,
    pub can_use_delta_format_version:
        Option<unsafe extern "C" fn(plugin: *const clap_plugin, format_version: clap_id) -> bool>,
    pub apply_delta: Option<
        unsafe extern "C" fn(
            plugin: *const clap_plugin,
            format_version: clap_id,
            delta: *const c_void,
            delta_size: usize,
        ) -> bool,
    >,
    pub set_context_info: Option<
        unsafe extern "C" fn(
            plugin: *const clap_plugin,
            flags: clap_undo_context_flags,
            undo_name: *const c_char,
            redo_name: *const c_char,
        ),
    >,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct clap_host_undo {
    pub begin_change: Option<unsafe extern "C" fn(host: *const clap_host)>,
    pub cancel_change: Option<unsafe extern "C" fn(host: *const clap_host)>,
    pub change_made: Option<
        unsafe extern "C" fn(
            host: *const clap_host,
            name: *const c_char,
            redo_delta: *const c_void,
            redo_delta_size: usize,
            undo_delta: *const c_void,
            undo_delta_size: usize,
        ),
    >,
    pub undo: Option<unsafe extern "C" fn(host: *const clap_host)>,
    pub redo: Option<unsafe extern "C" fn(host: *const clap_host)>,
    pub set_context_info_subscription:
        Option<unsafe extern "C" fn(host: *const clap_host, is_subscribed: bool)>,
}
