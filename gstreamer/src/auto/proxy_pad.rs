// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git)
// DO NOT EDIT

use crate::Object;
use crate::Pad;
use glib::object::IsA;
use glib::translate::*;

glib::wrapper! {
    pub struct ProxyPad(Object<ffi::GstProxyPad, ffi::GstProxyPadClass>) @extends Pad, Object;

    match fn {
        type_ => || ffi::gst_proxy_pad_get_type(),
    }
}

impl ProxyPad {
    //#[doc(alias = "gst_proxy_pad_iterate_internal_links_default")]
    //pub fn iterate_internal_links_default<P: IsA<Pad>, Q: IsA<Object>>(pad: &P, parent: Option<&Q>) -> /*Ignored*/Option<Iterator> {
    //    unsafe { TODO: call ffi:gst_proxy_pad_iterate_internal_links_default() }
    //}
}

unsafe impl Send for ProxyPad {}
unsafe impl Sync for ProxyPad {}

pub const NONE_PROXY_PAD: Option<&ProxyPad> = None;

pub trait ProxyPadExt: 'static {
    #[doc(alias = "gst_proxy_pad_get_internal")]
    fn internal(&self) -> Option<ProxyPad>;
}

impl<O: IsA<ProxyPad>> ProxyPadExt for O {
    fn internal(&self) -> Option<ProxyPad> {
        unsafe {
            from_glib_full(ffi::gst_proxy_pad_get_internal(
                self.as_ref().to_glib_none().0,
            ))
        }
    }
}
