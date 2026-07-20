use crate::{
    color::clap_color,
    cstr,
    plugin::*,
    string_sizes::{CLAP_NAME_SIZE, CLAP_PATH_SIZE},
};

use core::ffi::{c_char, CStr};

pub const CLAP_EXT_PROJECT_LOCATION: &CStr = cstr!("clap.project-location/2");

pub const CLAP_PROJECT_LOCATION_PROJECT: clap_project_location_kind = 1;
pub const CLAP_PROJECT_LOCATION_TRACK_GROUP: clap_project_location_kind = 2;
pub const CLAP_PROJECT_LOCATION_TRACK: clap_project_location_kind = 3;
pub const CLAP_PROJECT_LOCATION_DEVICE: clap_project_location_kind = 4;
pub const CLAP_PROJECT_LOCATION_NESTED_DEVICE_CHAIN: clap_project_location_kind = 5;

pub type clap_project_location_kind = u32;

pub const CLAP_PROJECT_LOCATION_INSTRUMENT_TRACK: clap_project_location_track_kind = 1;
pub const CLAP_PROJECT_LOCATION_AUDIO_TRACK: clap_project_location_track_kind = 2;
pub const CLAP_PROJECT_LOCATION_HYBRID_TRACK: clap_project_location_track_kind = 3;
pub const CLAP_PROJECT_LOCATION_RETURN_TRACK: clap_project_location_track_kind = 4;
pub const CLAP_PROJECT_LOCATION_MASTER_TRACK: clap_project_location_track_kind = 5;

pub type clap_project_location_track_kind = u32;

pub const CLAP_PROJECT_LOCATION_HAS_INDEX: clap_project_location_flags = 1 << 0;
pub const CLAP_PROJECT_LOCATION_HAS_COLOR: clap_project_location_flags = 1 << 1;

pub type clap_project_location_flags = u64;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct clap_project_location_element {
    pub flags: clap_project_location_flags,
    pub kind: clap_project_location_kind,
    pub track_kind: clap_project_location_track_kind,
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
