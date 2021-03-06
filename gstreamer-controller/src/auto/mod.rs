// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git)
// DO NOT EDIT

mod argb_control_binding;
pub use self::argb_control_binding::{ARGBControlBinding, NONE_ARGB_CONTROL_BINDING};

mod direct_control_binding;
pub use self::direct_control_binding::{DirectControlBinding, NONE_DIRECT_CONTROL_BINDING};

mod interpolation_control_source;
pub use self::interpolation_control_source::{
    InterpolationControlSource, NONE_INTERPOLATION_CONTROL_SOURCE,
};

mod lfo_control_source;
pub use self::lfo_control_source::{LFOControlSource, NONE_LFO_CONTROL_SOURCE};

#[cfg(any(feature = "v1_12", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_12")))]
mod proxy_control_binding;
#[cfg(any(feature = "v1_12", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_12")))]
pub use self::proxy_control_binding::{ProxyControlBinding, NONE_PROXY_CONTROL_BINDING};

mod timed_value_control_source;
pub use self::timed_value_control_source::{
    TimedValueControlSource, NONE_TIMED_VALUE_CONTROL_SOURCE,
};

mod trigger_control_source;
pub use self::trigger_control_source::{TriggerControlSource, NONE_TRIGGER_CONTROL_SOURCE};

mod enums;
pub use self::enums::InterpolationMode;
pub use self::enums::LFOWaveform;

#[doc(hidden)]
pub mod traits {
    pub use super::argb_control_binding::ARGBControlBindingExt;
    pub use super::direct_control_binding::DirectControlBindingExt;
    pub use super::interpolation_control_source::InterpolationControlSourceExt;
    pub use super::lfo_control_source::LFOControlSourceExt;
    pub use super::timed_value_control_source::TimedValueControlSourceExt;
    pub use super::trigger_control_source::TriggerControlSourceExt;
}
