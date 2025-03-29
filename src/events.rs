use crate::{fixedpoint::*, id::*};

use core::ffi::c_void;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct clap_event_header {
    pub size: u32,
    pub time: u32,
    pub space_id: u16,
    pub type_: clap_event_type,
    pub flags: u32,
}

pub const CLAP_CORE_EVENT_SPACE_ID: u16 = 0;

pub const CLAP_EVENT_IS_LIVE: clap_event_flags = 1 << 0;
pub const CLAP_EVENT_DONT_RECORD: clap_event_flags = 1 << 1;

pub type clap_event_flags = u32;

pub const CLAP_EVENT_NOTE_ON: clap_event_type = 0;
pub const CLAP_EVENT_NOTE_OFF: clap_event_type = 1;
pub const CLAP_EVENT_NOTE_CHOKE: clap_event_type = 2;
pub const CLAP_EVENT_NOTE_END: clap_event_type = 3;
pub const CLAP_EVENT_NOTE_EXPRESSION: clap_event_type = 4;
pub const CLAP_EVENT_PARAM_VALUE: clap_event_type = 5;
pub const CLAP_EVENT_PARAM_MOD: clap_event_type = 6;
pub const CLAP_EVENT_PARAM_GESTURE_BEGIN: clap_event_type = 7;
pub const CLAP_EVENT_PARAM_GESTURE_END: clap_event_type = 8;
pub const CLAP_EVENT_TRANSPORT: clap_event_type = 9;
pub const CLAP_EVENT_MIDI: clap_event_type = 10;
pub const CLAP_EVENT_MIDI_SYSEX: clap_event_type = 11;
pub const CLAP_EVENT_MIDI2: clap_event_type = 12;

pub type clap_event_type = u16;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct clap_event_note {
    pub header: clap_event_header,
    pub note_id: i32,
    pub port_index: i16,
    pub channel: i16,
    pub key: i16,
    pub velocity: f64,
}

pub const CLAP_NOTE_EXPRESSION_VOLUME: clap_note_expression = 0;
pub const CLAP_NOTE_EXPRESSION_PAN: clap_note_expression = 1;
pub const CLAP_NOTE_EXPRESSION_TUNING: clap_note_expression = 2;
pub const CLAP_NOTE_EXPRESSION_VIBRATO: clap_note_expression = 3;
pub const CLAP_NOTE_EXPRESSION_EXPRESSION: clap_note_expression = 4;
pub const CLAP_NOTE_EXPRESSION_BRIGHTNESS: clap_note_expression = 5;
pub const CLAP_NOTE_EXPRESSION_PRESSURE: clap_note_expression = 6;

pub type clap_note_expression = i32;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct clap_event_note_expression {
    pub header: clap_event_header,
    pub expression_id: clap_note_expression,
    pub note_id: i32,
    pub port_index: i16,
    pub channel: i16,
    pub key: i16,
    pub value: f64,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct clap_event_param_value {
    pub header: clap_event_header,
    pub param_id: clap_id,
    pub cookie: *mut c_void,
    pub note_id: i32,
    pub port_index: i16,
    pub channel: i16,
    pub key: i16,
    pub value: f64,
}

unsafe impl Send for clap_event_param_value {}
unsafe impl Sync for clap_event_param_value {}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct clap_event_param_mod {
    pub header: clap_event_header,
    pub param_id: clap_id,
    pub cookie: *mut c_void,
    pub note_id: i32,
    pub port_index: i16,
    pub channel: i16,
    pub key: i16,
    pub amount: f64,
}

unsafe impl Send for clap_event_param_mod {}
unsafe impl Sync for clap_event_param_mod {}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct clap_event_param_gesture {
    pub header: clap_event_header,
    pub param_id: clap_id,
}

pub const CLAP_TRANSPORT_HAS_TEMPO: clap_transport_flags = 1 << 0;
pub const CLAP_TRANSPORT_HAS_BEATS_TIMELINE: clap_transport_flags = 1 << 1;
pub const CLAP_TRANSPORT_HAS_SECONDS_TIMELINE: clap_transport_flags = 1 << 2;
pub const CLAP_TRANSPORT_HAS_TIME_SIGNATURE: clap_transport_flags = 1 << 3;
pub const CLAP_TRANSPORT_IS_PLAYING: clap_transport_flags = 1 << 4;
pub const CLAP_TRANSPORT_IS_RECORDING: clap_transport_flags = 1 << 5;
pub const CLAP_TRANSPORT_IS_LOOP_ACTIVE: clap_transport_flags = 1 << 6;
pub const CLAP_TRANSPORT_IS_WITHIN_PRE_ROLL: clap_transport_flags = 1 << 7;

pub type clap_transport_flags = u32;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct clap_event_transport {
    pub header: clap_event_header,
    pub flags: clap_transport_flags,
    pub song_pos_beats: clap_beattime,
    pub song_pos_seconds: clap_sectime,
    pub tempo: f64,
    pub tempo_inc: f64,
    pub loop_start_beats: clap_beattime,
    pub loop_end_beats: clap_beattime,
    pub loop_start_seconds: clap_sectime,
    pub loop_end_seconds: clap_sectime,
    pub bar_start: clap_beattime,
    pub bar_number: i32,
    pub tsig_num: u16,
    pub tsig_denom: u16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct clap_event_midi {
    pub header: clap_event_header,
    pub port_index: u16,
    pub data: [u8; 3],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct clap_event_midi_sysex {
    pub header: clap_event_header,
    pub port_index: u16,
    pub buffer: *const u8,
    pub size: u32,
}

unsafe impl Send for clap_event_midi_sysex {}
unsafe impl Sync for clap_event_midi_sysex {}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct clap_event_midi2 {
    pub header: clap_event_header,
    pub port_index: u16,
    pub data: [u32; 4],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct clap_input_events {
    pub ctx: *mut c_void,
    pub size: Option<unsafe extern "C" fn(list: *const clap_input_events) -> u32>,
    pub get: Option<
        unsafe extern "C" fn(
            list: *const clap_input_events,
            index: u32,
        ) -> *const clap_event_header,
    >,
}

unsafe impl Send for clap_input_events {}
unsafe impl Sync for clap_input_events {}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct clap_output_events {
    pub ctx: *mut c_void,
    pub try_push: Option<
        unsafe extern "C" fn(
            list: *const clap_output_events,
            event: *const clap_event_header,
        ) -> bool,
    >,
}

unsafe impl Send for clap_output_events {}
unsafe impl Sync for clap_output_events {}
