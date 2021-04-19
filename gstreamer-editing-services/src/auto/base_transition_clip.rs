// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git)
// DO NOT EDIT

use crate::Clip;
use crate::Container;
use crate::Extractable;
use crate::OperationClip;
use crate::TimelineElement;

glib::wrapper! {
    pub struct BaseTransitionClip(Object<ffi::GESBaseTransitionClip, ffi::GESBaseTransitionClipClass>) @extends OperationClip, Clip, Container, TimelineElement, @implements Extractable;

    match fn {
        type_ => || ffi::ges_base_transition_clip_get_type(),
    }
}

impl BaseTransitionClip {}

pub const NONE_BASE_TRANSITION_CLIP: Option<&BaseTransitionClip> = None;
