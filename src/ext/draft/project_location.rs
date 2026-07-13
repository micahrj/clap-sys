use crate::{
    color::clap_color,
    cstr,
    plugin::*,
    string_sizes::{CLAP_NAME_SIZE, CLAP_PATH_SIZE},
};

use core::ffi::c_char;
use core::ffi::CStr;

pub const CLAP_EXT_PROJECT_LOCATION: &CStr = cstr!("clap.project-location/2");

pub const CLAP_PROJECT_LOCATION_PROJECT: u32 = 1;
pub const CLAP_PROJECT_LOCATION_TRACK_GROUP: u32 = 2;
pub const CLAP_PROJECT_LOCATION_TRACK: u32 = 3;
pub const CLAP_PROJECT_LOCATION_DEVICE: u32 = 4;
pub const CLAP_PROJECT_LOCATION_NESTED_DEVICE_CHAIN: u32 = 5;

pub const CLAP_PROJECT_LOCATION_INSTRUMENT_TRACK: u32 = 1;
pub const CLAP_PROJECT_LOCATION_AUDIO_TRACK: u32 = 2;
pub const CLAP_PROJECT_LOCATION_HYBRID_TRACK: u32 = 3;
pub const CLAP_PROJECT_LOCATION_RETURN_TRACK: u32 = 4;
pub const CLAP_PROJECT_LOCATION_MASTER_TRACK: u32 = 5;

pub const CLAP_PROJECT_LOCATION_HAS_INDEX: u32 = 1 << 0;
pub const CLAP_PROJECT_LOCATION_HAS_COLOR: u32 = 1 << 1;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct clap_project_location_element {
    pub flags: u64,
    pub kind: u32,
    pub track_kind: u32,
    pub index: u32,
    pub id: [c_char; CLAP_PATH_SIZE],
    pub name: [c_char; CLAP_NAME_SIZE],
    pub color: clap_color,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct clap_plugin_project_location {
    pub set: Option<
        unsafe extern "C" fn(
            plugin: *const clap_plugin,
            path: *const clap_project_location_element,
            num_elements: u32,
        ),
    >,
}
