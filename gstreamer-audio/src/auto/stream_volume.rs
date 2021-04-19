// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git)
// DO NOT EDIT

use crate::StreamVolumeFormat;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use std::boxed::Box as Box_;
use std::mem::transmute;

glib::wrapper! {
    pub struct StreamVolume(Interface<ffi::GstStreamVolume, ffi::GstStreamVolumeInterface>);

    match fn {
        type_ => || ffi::gst_stream_volume_get_type(),
    }
}

impl StreamVolume {
    #[doc(alias = "gst_stream_volume_convert_volume")]
    pub fn convert_volume(from: StreamVolumeFormat, to: StreamVolumeFormat, val: f64) -> f64 {
        assert_initialized_main_thread!();
        unsafe { ffi::gst_stream_volume_convert_volume(from.to_glib(), to.to_glib(), val) }
    }
}

unsafe impl Send for StreamVolume {}
unsafe impl Sync for StreamVolume {}

pub const NONE_STREAM_VOLUME: Option<&StreamVolume> = None;

pub trait StreamVolumeExt: 'static {
    #[doc(alias = "gst_stream_volume_get_mute")]
    fn is_muted(&self) -> bool;

    #[doc(alias = "gst_stream_volume_get_volume")]
    fn get_volume(&self, format: StreamVolumeFormat) -> f64;

    #[doc(alias = "gst_stream_volume_set_mute")]
    fn set_mute(&self, mute: bool);

    #[doc(alias = "gst_stream_volume_set_volume")]
    fn set_volume(&self, format: StreamVolumeFormat, val: f64);

    fn connect_property_mute_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_volume_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;
}

impl<O: IsA<StreamVolume>> StreamVolumeExt for O {
    fn is_muted(&self) -> bool {
        unsafe {
            from_glib(ffi::gst_stream_volume_get_mute(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_volume(&self, format: StreamVolumeFormat) -> f64 {
        unsafe {
            ffi::gst_stream_volume_get_volume(self.as_ref().to_glib_none().0, format.to_glib())
        }
    }

    fn set_mute(&self, mute: bool) {
        unsafe {
            ffi::gst_stream_volume_set_mute(self.as_ref().to_glib_none().0, mute.to_glib());
        }
    }

    fn set_volume(&self, format: StreamVolumeFormat, val: f64) {
        unsafe {
            ffi::gst_stream_volume_set_volume(
                self.as_ref().to_glib_none().0,
                format.to_glib(),
                val,
            );
        }
    }

    fn connect_property_mute_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_mute_trampoline<P, F: Fn(&P) + Send + Sync + 'static>(
            this: *mut ffi::GstStreamVolume,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<StreamVolume>,
        {
            let f: &F = &*(f as *const F);
            f(&StreamVolume::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::mute\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_mute_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_volume_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_volume_trampoline<P, F: Fn(&P) + Send + Sync + 'static>(
            this: *mut ffi::GstStreamVolume,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<StreamVolume>,
        {
            let f: &F = &*(f as *const F);
            f(&StreamVolume::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::volume\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_volume_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}
