// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git)
// DO NOT EDIT

use crate::BaseSrc;

glib::wrapper! {
    pub struct PushSrc(Object<ffi::GstPushSrc, ffi::GstPushSrcClass>) @extends BaseSrc, gst::Element, gst::Object;

    match fn {
        type_ => || ffi::gst_push_src_get_type(),
    }
}

impl PushSrc {}

unsafe impl Send for PushSrc {}
unsafe impl Sync for PushSrc {}

pub const NONE_PUSH_SRC: Option<&PushSrc> = None;
