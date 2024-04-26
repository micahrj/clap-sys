use crate::{host::*, id::*, plugin::*, string_sizes::*};

use std::ffi::c_char;
use std::ffi::CStr;

pub const CLAP_EXT_NOTE_PORTS: &CStr = c"clap.note-ports";

pub const CLAP_NOTE_DIALECT_CLAP: clap_note_dialect = 1 << 0;
pub const CLAP_NOTE_DIALECT_MIDI: clap_note_dialect = 1 << 1;
pub const CLAP_NOTE_DIALECT_MIDI_MPE: clap_note_dialect = 1 << 2;
pub const CLAP_NOTE_DIALECT_MIDI2: clap_note_dialect = 1 << 3;

pub type clap_note_dialect = u32;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct clap_note_port_info {
    pub id: clap_id,
    pub supported_dialects: clap_note_dialect,
    pub preferred_dialect: clap_note_dialect,
    pub name: [c_char; CLAP_NAME_SIZE],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct clap_plugin_note_ports {
    pub count: Option<unsafe extern "C" fn(plugin: *const clap_plugin, is_input: bool) -> u32>,
    pub get: Option<
        unsafe extern "C" fn(
            plugin: *const clap_plugin,
            index: u32,
            is_input: bool,
            info: *mut clap_note_port_info,
        ) -> bool,
    >,
}

pub const CLAP_NOTE_PORTS_RESCAN_ALL: u32 = 1 << 0;
pub const CLAP_NOTE_PORTS_RESCAN_NAMES: u32 = 1 << 1;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct clap_host_note_ports {
    pub supported_dialects:
        Option<unsafe extern "C" fn(host: *const clap_host) -> clap_note_dialect>,
    pub rescan: Option<unsafe extern "C" fn(host: *const clap_host, flags: u32)>,
}
