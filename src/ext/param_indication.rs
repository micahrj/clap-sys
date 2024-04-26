use crate::{color::*, id::*, plugin::*};

use std::ffi::c_char;
use std::ffi::CStr;

pub const CLAP_EXT_PARAM_INDICATION: &CStr = c"clap.param-indication/4";
pub const CLAP_EXT_PARAM_INDICATION_COMPAT: &CStr = c"clap.param-indication.draft/4";

pub const CLAP_PARAM_INDICATION_AUTOMATION_NONE: u32 = 0;
pub const CLAP_PARAM_INDICATION_AUTOMATION_PRESENT: u32 = 1;
pub const CLAP_PARAM_INDICATION_AUTOMATION_PLAYING: u32 = 2;
pub const CLAP_PARAM_INDICATION_AUTOMATION_RECORDING: u32 = 3;
pub const CLAP_PARAM_INDICATION_AUTOMATION_OVERRIDING: u32 = 4;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct clap_plugin_param_indication {
    pub set_mapping: Option<
        unsafe extern "C" fn(
            plugin: *const clap_plugin,
            param_id: clap_id,
            has_mapping: bool,
            color: *const clap_color,
            label: *const c_char,
            description: *const c_char,
        ),
    >,
    pub set_automation: Option<
        unsafe extern "C" fn(
            plugin: *const clap_plugin,
            param_id: clap_id,
            automation_state: u32,
            color: *const clap_color,
        ),
    >,
}
