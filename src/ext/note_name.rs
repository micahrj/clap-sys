use crate::{cstr, host::*, plugin::*, string_sizes::*};

use std::ffi::{c_char, CStr};

pub const CLAP_EXT_NOTE_NAME: &CStr = cstr!("clap.note-name");

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct clap_note_name {
    pub name: [c_char; CLAP_NAME_SIZE],
    pub port: i16,
    pub key: i16,
    pub channel: i16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct clap_plugin_note_name {
    pub count: Option<unsafe extern "C" fn(plugin: *const clap_plugin) -> u32>,
    pub get: Option<
        unsafe extern "C" fn(
            plugin: *const clap_plugin,
            index: u32,
            note_name: *mut clap_note_name,
        ) -> bool,
    >,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct clap_host_note_name {
    pub changed: Option<unsafe extern "C" fn(host: *const clap_host)>,
}
