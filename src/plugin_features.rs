use std::ffi::CStr;

pub const CLAP_PLUGIN_FEATURE_INSTRUMENT: &CStr = c"instrument";

pub const CLAP_PLUGIN_FEATURE_AUDIO_EFFECT: &CStr = c"audio-effect";

pub const CLAP_PLUGIN_FEATURE_NOTE_EFFECT: &CStr = c"note-effect";

pub const CLAP_PLUGIN_FEATURE_NOTE_DETECTOR: &CStr = c"note-detector";

pub const CLAP_PLUGIN_FEATURE_ANALYZER: &CStr = c"analyzer";

pub const CLAP_PLUGIN_FEATURE_SYNTHESIZER: &CStr = c"synthesizer";
pub const CLAP_PLUGIN_FEATURE_SAMPLER: &CStr = c"sampler";
pub const CLAP_PLUGIN_FEATURE_DRUM: &CStr = c"drum";
pub const CLAP_PLUGIN_FEATURE_DRUM_MACHINE: &CStr = c"drum-machine";

pub const CLAP_PLUGIN_FEATURE_FILTER: &CStr = c"filter";
pub const CLAP_PLUGIN_FEATURE_PHASER: &CStr = c"phaser";
pub const CLAP_PLUGIN_FEATURE_EQUALIZER: &CStr = c"equalizer";
pub const CLAP_PLUGIN_FEATURE_DEESSER: &CStr = c"de-esser";
pub const CLAP_PLUGIN_FEATURE_PHASE_VOCODER: &CStr = c"phase-vocoder";
pub const CLAP_PLUGIN_FEATURE_GRANULAR: &CStr = c"granular";
pub const CLAP_PLUGIN_FEATURE_FREQUENCY_SHIFTER: &CStr = c"frequency-shifter";
pub const CLAP_PLUGIN_FEATURE_PITCH_SHIFTER: &CStr = c"pitch-shifter";

pub const CLAP_PLUGIN_FEATURE_DISTORTION: &CStr = c"distortion";
pub const CLAP_PLUGIN_FEATURE_TRANSIENT_SHAPER: &CStr = c"transient-shaper";
pub const CLAP_PLUGIN_FEATURE_COMPRESSOR: &CStr = c"compressor";
pub const CLAP_PLUGIN_FEATURE_EXPANDER: &CStr = c"expander";
pub const CLAP_PLUGIN_FEATURE_GATE: &CStr = c"gate";
pub const CLAP_PLUGIN_FEATURE_LIMITER: &CStr = c"limiter";

pub const CLAP_PLUGIN_FEATURE_FLANGER: &CStr = c"flanger";
pub const CLAP_PLUGIN_FEATURE_CHORUS: &CStr = c"chorus";
pub const CLAP_PLUGIN_FEATURE_DELAY: &CStr = c"delay";
pub const CLAP_PLUGIN_FEATURE_REVERB: &CStr = c"reverb";

pub const CLAP_PLUGIN_FEATURE_TREMOLO: &CStr = c"tremolo";
pub const CLAP_PLUGIN_FEATURE_GLITCH: &CStr = c"glitch";

pub const CLAP_PLUGIN_FEATURE_UTILITY: &CStr = c"utility";
pub const CLAP_PLUGIN_FEATURE_PITCH_CORRECTION: &CStr = c"pitch-correction";
pub const CLAP_PLUGIN_FEATURE_RESTORATION: &CStr = c"restoration";

pub const CLAP_PLUGIN_FEATURE_MULTI_EFFECTS: &CStr = c"multi-effects";

pub const CLAP_PLUGIN_FEATURE_MIXING: &CStr = c"mixing";
pub const CLAP_PLUGIN_FEATURE_MASTERING: &CStr = c"mastering";

pub const CLAP_PLUGIN_FEATURE_MONO: &CStr = c"mono";
pub const CLAP_PLUGIN_FEATURE_STEREO: &CStr = c"stereo";
pub const CLAP_PLUGIN_FEATURE_SURROUND: &CStr = c"surround";
pub const CLAP_PLUGIN_FEATURE_AMBISONIC: &CStr = c"ambisonic";
