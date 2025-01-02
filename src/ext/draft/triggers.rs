use crate::{cstr, events::*, host::*, id::*, plugin::*, string_sizes::*};

use std::ffi::c_void;
use std::ffi::CStr;
use std::os::raw::c_char;

pub const CLAP_EXT_TRIGGERS: &CStr = cstr!("clap.triggers/1");

pub const CLAP_TRIGGER_IS_AUTOMATABLE_PER_NOTE_ID: clap_trigger_info_flags = 1 << 0;
pub const CLAP_TRIGGER_IS_AUTOMATABLE_PER_KEY: clap_trigger_info_flags = 1 << 1;
pub const CLAP_TRIGGER_IS_AUTOMATABLE_PER_CHANNEL: clap_trigger_info_flags = 1 << 2;
pub const CLAP_TRIGGER_IS_AUTOMATABLE_PER_PORT: clap_trigger_info_flags = 1 << 3;

pub type clap_trigger_info_flags = u32;

pub const CLAP_EVENT_TRIGGER: clap_event_type = 0;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct clap_event_trigger {
    pub header: clap_event_header,
    pub trigger_id: clap_id,
    pub cookie: *mut c_void,
    pub note_id: i32,
    pub port_index: i16,
    pub channel: i16,
    pub key: i16,
}

unsafe impl Send for clap_event_trigger {}
unsafe impl Sync for clap_event_trigger {}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct clap_trigger_info {
    pub id: clap_id,
    pub flags: clap_trigger_info_flags,
    pub cookie: *mut c_void,
    pub name: [c_char; CLAP_NAME_SIZE],
    pub module: [c_char; CLAP_PATH_SIZE],
}

unsafe impl Send for clap_trigger_info {}
unsafe impl Sync for clap_trigger_info {}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct clap_plugin_triggers {
    pub count: Option<unsafe extern "C" fn(plugin: *const clap_plugin) -> u32>,
    pub get_info: Option<
        unsafe extern "C" fn(
            plugin: *const clap_plugin,
            trigger_index: u32,
            trigger_info: *mut clap_trigger_info,
        ) -> bool,
    >,
}

pub const CLAP_TRIGGER_RESCAN_INFO: clap_trigger_rescan_flags = 1 << 0;
pub const CLAP_TRIGGER_RESCAN_ALL: clap_trigger_rescan_flags = 1 << 1;

pub type clap_trigger_rescan_flags = u32;

pub const CLAP_TRIGGER_CLEAR_ALL: clap_trigger_clear_flags = 1 << 0;
pub const CLAP_TRIGGER_CLEAR_AUTOMATIONS: clap_trigger_clear_flags = 1 << 1;

pub type clap_trigger_clear_flags = u32;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct clap_host_triggers {
    pub rescan:
        Option<unsafe extern "C" fn(host: *const clap_host, flags: clap_trigger_rescan_flags)>,
    pub clear: Option<
        unsafe extern "C" fn(
            host: *const clap_host,
            trigger_id: clap_id,
            flags: clap_trigger_clear_flags,
        ),
    >,
}
