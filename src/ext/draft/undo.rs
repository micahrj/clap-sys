use crate::{cstr, host::*, id::*, plugin::*};

use std::ffi::{c_char, c_void, CStr};

pub const CLAP_EXT_UNDO: &CStr = cstr!("clap.undo/4");
pub const CLAP_EXT_UNDO_CONTEXT: &CStr = cstr!("clap.undo_context/4");
pub const CLAP_EXT_UNDO_DELTA: &CStr = cstr!("clap.undo_delta/4");

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct clap_undo_delta_properties {
    pub has_delta: bool,
    pub are_deltas_persistent: bool,
    pub format_version: clap_id,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct clap_plugin_undo_delta {
    pub get_delta_properties: Option<
        unsafe extern "C" fn(
            plugin: *const clap_plugin,
            properties: *mut clap_undo_delta_properties,
        ),
    >,
    pub can_use_delta_format_version:
        Option<unsafe extern "C" fn(plugin: *const clap_plugin, format_version: clap_id) -> bool>,
    pub undo: Option<
        unsafe extern "C" fn(
            plugin: *const clap_plugin,
            format_version: clap_id,
            delta: *const c_void,
            delta_size: usize,
        ) -> bool,
    >,
    pub redo: Option<
        unsafe extern "C" fn(
            plugin: *const clap_plugin,
            format_version: clap_id,
            delta: *const c_void,
            delta_size: usize,
        ) -> bool,
    >,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct clap_plugin_undo_context {
    pub set_can_undo: Option<unsafe extern "C" fn(plugin: *const clap_plugin, can_undo: bool)>,
    pub set_can_redo: Option<unsafe extern "C" fn(plugin: *const clap_plugin, can_redo: bool)>,
    pub set_undo_name:
        Option<unsafe extern "C" fn(plugin: *const clap_plugin, name: *const c_char)>,
    pub set_redo_name:
        Option<unsafe extern "C" fn(plugin: *const clap_plugin, name: *const c_char)>,
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
            delta: *const c_void,
            delta_size: usize,
            delta_can_undo: bool,
        ),
    >,
    pub request_undo: Option<unsafe extern "C" fn(host: *const clap_host)>,
    pub request_redo: Option<unsafe extern "C" fn(host: *const clap_host)>,
    pub set_wants_context_updates:
        Option<unsafe extern "C" fn(host: *const clap_host, is_subscribed: bool)>,
}
