use crate::cstr;

use std::ffi::CStr;

pub const CLAP_PLUGIN_FEATURE_INSTRUMENT: &CStr = cstr!("instrument");

pub const CLAP_PLUGIN_FEATURE_AUDIO_EFFECT: &CStr = cstr!("audio-effect");

pub const CLAP_PLUGIN_FEATURE_NOTE_EFFECT: &CStr = cstr!("note-effect");

pub const CLAP_PLUGIN_FEATURE_NOTE_DETECTOR: &CStr = cstr!("note-detector");

pub const CLAP_PLUGIN_FEATURE_ANALYZER: &CStr = cstr!("analyzer");

pub const CLAP_PLUGIN_FEATURE_SYNTHESIZER: &CStr = cstr!("synthesizer");
pub const CLAP_PLUGIN_FEATURE_SAMPLER: &CStr = cstr!("sampler");
pub const CLAP_PLUGIN_FEATURE_DRUM: &CStr = cstr!("drum");
pub const CLAP_PLUGIN_FEATURE_DRUM_MACHINE: &CStr = cstr!("drum-machine");

pub const CLAP_PLUGIN_FEATURE_FILTER: &CStr = cstr!("filter");
pub const CLAP_PLUGIN_FEATURE_PHASER: &CStr = cstr!("phaser");
pub const CLAP_PLUGIN_FEATURE_EQUALIZER: &CStr = cstr!("equalizer");
pub const CLAP_PLUGIN_FEATURE_DEESSER: &CStr = cstr!("de-esser");
pub const CLAP_PLUGIN_FEATURE_PHASE_VOCODER: &CStr = cstr!("phase-vocoder");
pub const CLAP_PLUGIN_FEATURE_GRANULAR: &CStr = cstr!("granular");
pub const CLAP_PLUGIN_FEATURE_FREQUENCY_SHIFTER: &CStr = cstr!("frequency-shifter");
pub const CLAP_PLUGIN_FEATURE_PITCH_SHIFTER: &CStr = cstr!("pitch-shifter");

pub const CLAP_PLUGIN_FEATURE_DISTORTION: &CStr = cstr!("distortion");
pub const CLAP_PLUGIN_FEATURE_TRANSIENT_SHAPER: &CStr = cstr!("transient-shaper");
pub const CLAP_PLUGIN_FEATURE_COMPRESSOR: &CStr = cstr!("compressor");
pub const CLAP_PLUGIN_FEATURE_EXPANDER: &CStr = cstr!("expander");
pub const CLAP_PLUGIN_FEATURE_GATE: &CStr = cstr!("gate");
pub const CLAP_PLUGIN_FEATURE_LIMITER: &CStr = cstr!("limiter");

pub const CLAP_PLUGIN_FEATURE_FLANGER: &CStr = cstr!("flanger");
pub const CLAP_PLUGIN_FEATURE_CHORUS: &CStr = cstr!("chorus");
pub const CLAP_PLUGIN_FEATURE_DELAY: &CStr = cstr!("delay");
pub const CLAP_PLUGIN_FEATURE_REVERB: &CStr = cstr!("reverb");

pub const CLAP_PLUGIN_FEATURE_TREMOLO: &CStr = cstr!("tremolo");
pub const CLAP_PLUGIN_FEATURE_GLITCH: &CStr = cstr!("glitch");

pub const CLAP_PLUGIN_FEATURE_UTILITY: &CStr = cstr!("utility");
pub const CLAP_PLUGIN_FEATURE_PITCH_CORRECTION: &CStr = cstr!("pitch-correction");
pub const CLAP_PLUGIN_FEATURE_RESTORATION: &CStr = cstr!("restoration");

pub const CLAP_PLUGIN_FEATURE_MULTI_EFFECTS: &CStr = cstr!("multi-effects");

pub const CLAP_PLUGIN_FEATURE_MIXING: &CStr = cstr!("mixing");
pub const CLAP_PLUGIN_FEATURE_MASTERING: &CStr = cstr!("mastering");

pub const CLAP_PLUGIN_FEATURE_MONO: &CStr = cstr!("mono");
pub const CLAP_PLUGIN_FEATURE_STEREO: &CStr = cstr!("stereo");
pub const CLAP_PLUGIN_FEATURE_SURROUND: &CStr = cstr!("surround");
pub const CLAP_PLUGIN_FEATURE_AMBISONIC: &CStr = cstr!("ambisonic");
