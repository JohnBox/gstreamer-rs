// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git)
// DO NOT EDIT

use crate::GLContext;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::StaticType;
use std::boxed::Box as Box_;
use std::mem::transmute;

glib::wrapper! {
    pub struct GLBaseFilter(Object<ffi::GstGLBaseFilter, ffi::GstGLBaseFilterClass>) @extends gst_base::BaseTransform, gst::Element, gst::Object;

    match fn {
        type_ => || ffi::gst_gl_base_filter_get_type(),
    }
}

unsafe impl Send for GLBaseFilter {}
unsafe impl Sync for GLBaseFilter {}

pub const NONE_GL_BASE_FILTER: Option<&GLBaseFilter> = None;

pub trait GLBaseFilterExt: 'static {
    #[cfg(any(feature = "v1_16", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_16")))]
    #[doc(alias = "gst_gl_base_filter_find_gl_context")]
    fn find_gl_context(&self) -> bool;

    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    #[doc(alias = "gst_gl_base_filter_get_gl_context")]
    fn gl_context(&self) -> Option<GLContext>;

    #[doc(alias = "get_property_context")]
    fn context(&self) -> Option<GLContext>;

    fn connect_property_context_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;
}

impl<O: IsA<GLBaseFilter>> GLBaseFilterExt for O {
    #[cfg(any(feature = "v1_16", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_16")))]
    fn find_gl_context(&self) -> bool {
        unsafe {
            from_glib(ffi::gst_gl_base_filter_find_gl_context(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    fn gl_context(&self) -> Option<GLContext> {
        unsafe {
            from_glib_full(ffi::gst_gl_base_filter_get_gl_context(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn context(&self) -> Option<GLContext> {
        unsafe {
            let mut value = glib::Value::from_type(<GLContext as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"context\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `context` getter")
        }
    }

    fn connect_property_context_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_context_trampoline<P, F: Fn(&P) + Send + Sync + 'static>(
            this: *mut ffi::GstGLBaseFilter,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<GLBaseFilter>,
        {
            let f: &F = &*(f as *const F);
            f(&GLBaseFilter::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::context\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_context_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}
