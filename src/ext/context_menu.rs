use crate::{host::*, id::*, plugin::*};

use std::ffi::c_char;
use std::ffi::{c_void, CStr};

pub const CLAP_EXT_CONTEXT_MENU: &CStr = c"clap.context-menu/1";
pub const CLAP_EXT_CONTEXT_MENU_COMPAT: &CStr = c"clap.context-menu.draft/0";

pub const CLAP_CONTEXT_MENU_TARGET_KIND_GLOBAL: u32 = 0;
pub const CLAP_CONTEXT_MENU_TARGET_KIND_PARAM: u32 = 1;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct clap_context_menu_target {
    pub kind: u32,
    pub id: clap_id,
}

pub const CLAP_CONTEXT_MENU_ITEM_ENTRY: clap_context_menu_item_kind = 0;
pub const CLAP_CONTEXT_MENU_ITEM_CHECK_ENTRY: clap_context_menu_item_kind = 1;
pub const CLAP_CONTEXT_MENU_ITEM_SEPARATOR: clap_context_menu_item_kind = 2;
pub const CLAP_CONTEXT_MENU_ITEM_BEGIN_SUBMENU: clap_context_menu_item_kind = 3;
pub const CLAP_CONTEXT_MENU_ITEM_END_SUBMENU: clap_context_menu_item_kind = 4;
pub const CLAP_CONTEXT_MENU_ITEM_TITLE: clap_context_menu_item_kind = 5;
pub type clap_context_menu_item_kind = u32;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct clap_context_menu_entry {
    pub label: *const c_char,
    pub is_enabled: bool,
    pub action_id: clap_id,
}

unsafe impl Send for clap_context_menu_entry {}
unsafe impl Sync for clap_context_menu_entry {}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct clap_context_menu_check_entry {
    pub label: *const c_char,
    pub is_enabled: bool,
    pub is_checked: bool,
    pub action_id: clap_id,
}

unsafe impl Send for clap_context_menu_check_entry {}
unsafe impl Sync for clap_context_menu_check_entry {}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct clap_context_menu_item_title {
    pub title: *const c_char,
    pub is_enabled: bool,
}

unsafe impl Send for clap_context_menu_item_title {}
unsafe impl Sync for clap_context_menu_item_title {}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct clap_context_menu_submenu {
    pub label: *const c_char,
    pub is_enabled: bool,
}

unsafe impl Send for clap_context_menu_submenu {}
unsafe impl Sync for clap_context_menu_submenu {}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct clap_context_menu_builder {
    pub ctx: *mut c_void,
    pub add_item: Option<
        unsafe extern "C" fn(
            builder: *const clap_context_menu_builder,
            item_kind: clap_context_menu_item_kind,
            item_data: *const c_void,
        ) -> bool,
    >,
    pub supports: Option<
        unsafe extern "C" fn(
            builder: *const clap_context_menu_builder,
            item_kind: clap_context_menu_item_kind,
        ) -> bool,
    >,
}

unsafe impl Send for clap_context_menu_builder {}
unsafe impl Sync for clap_context_menu_builder {}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct clap_plugin_context_menu {
    pub populate: Option<
        unsafe extern "C" fn(
            plugin: *const clap_plugin,
            target: *const clap_context_menu_target,
            builder: *const clap_context_menu_builder,
        ) -> bool,
    >,
    pub perform: Option<
        unsafe extern "C" fn(
            plugin: *const clap_plugin,
            target: *const clap_context_menu_target,
            action_id: clap_id,
        ) -> bool,
    >,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct clap_host_context_menu {
    pub populate: Option<
        unsafe extern "C" fn(
            host: *const clap_host,
            target: *const clap_context_menu_target,
            builder: *const clap_context_menu_builder,
        ) -> bool,
    >,
    pub perform: Option<
        unsafe extern "C" fn(
            host: *const clap_host,
            target: *const clap_context_menu_target,
            action_id: clap_id,
        ) -> bool,
    >,
    pub can_popup: Option<unsafe extern "C" fn(host: *const clap_host) -> bool>,
    pub popup: Option<
        unsafe extern "C" fn(
            host: *const clap_host,
            target: *const clap_context_menu_target,
            screen_index: i32,
            x: i32,
            y: i32,
        ) -> bool,
    >,
}
