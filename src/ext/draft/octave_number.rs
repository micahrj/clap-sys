use crate::{cstr, plugin::clap_plugin};
use core::ffi::CStr;

pub const CLAP_EXT_OCTAVE_NUMBER: &CStr = cstr!("clap.octave-number/1");

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct clap_plugin_octave_number {
    pub set_note60_octave:
        Option<unsafe extern "C" fn(plugin: *const clap_plugin, octave_number: i8)>,
}
