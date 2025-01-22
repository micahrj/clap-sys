use crate::{
    color::clap_color,
    plugin::*,
    string_sizes::{CLAP_NAME_SIZE, CLAP_PATH_SIZE},
};

use std::ffi::c_char;
use std::ffi::CStr;

pub const CLAP_EXT_LOCATION: &CStr = c"clap.location/1";

pub const CLAP_PLUGIN_LOCATION_PROJECT: u32 = 1;
pub const CLAP_PLUGIN_LOCATION_TRACK_GROUP: u32 = 2;
pub const CLAP_PLUGIN_LOCATION_TRACK: u32 = 3;
pub const CLAP_PLUGIN_LOCATION_DEVICE: u32 = 4;
pub const CLAP_PLUGIN_LOCATION_NESTED_DEVICE_CHAIN: u32 = 5;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct clap_plugin_location_element {
    pub kind: u32,
    pub index: u32,
    pub id: [c_char; CLAP_PATH_SIZE],
    pub name: [c_char; CLAP_NAME_SIZE],
    pub color: clap_color,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct clap_plugin_location {
    pub set_location: Option<
        unsafe extern "C" fn(
            plugin: *const clap_plugin,
            path: *const clap_plugin_location,
            num_elements: u32,
        ),
    >,
}
