// This file was generated by gir (d50d839) from gir-files (???)
// DO NOT EDIT

use ffi;
use glib::StaticType;
use glib::Value;
use glib::object::Downcast;
use glib::signal::SignalHandlerId;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use gst;
use gst_ffi;
use std::boxed::Box as Box_;
use std::mem;
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct PtpClock(Object<ffi::GstPtpClock, ffi::GstPtpClockClass>): [
        gst::Clock => gst_ffi::GstClock,
        gst::Object => gst_ffi::GstObject,
    ];

    match fn {
        get_type => || ffi::gst_ptp_clock_get_type(),
    }
}

impl PtpClock {
    pub fn new<'a, P: Into<Option<&'a str>>>(name: P, domain: u32) -> PtpClock {
        assert_initialized_main_thread!();
        let name = name.into();
        let name = name.to_glib_none();
        unsafe {
            gst::Clock::from_glib_none(ffi::gst_ptp_clock_new(name.0, domain)).downcast_unchecked()
        }
    }

    pub fn get_property_domain(&self) -> u32 {
        unsafe {
            let mut value = Value::uninitialized();
            gobject_ffi::g_value_init(value.to_glib_none_mut().0, <u32 as StaticType>::static_type().to_glib());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "domain".to_glib_none().0, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    pub fn get_property_grandmaster_clock_id(&self) -> u64 {
        unsafe {
            let mut value = Value::uninitialized();
            gobject_ffi::g_value_init(value.to_glib_none_mut().0, <u64 as StaticType>::static_type().to_glib());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "grandmaster-clock-id".to_glib_none().0, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    pub fn get_property_internal_clock(&self) -> Option<gst::Clock> {
        unsafe {
            let mut value = Value::uninitialized();
            gobject_ffi::g_value_init(value.to_glib_none_mut().0, <gst::Clock as StaticType>::static_type().to_glib());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "internal-clock".to_glib_none().0, value.to_glib_none_mut().0);
            value.get()
        }
    }

    pub fn get_property_master_clock_id(&self) -> u64 {
        unsafe {
            let mut value = Value::uninitialized();
            gobject_ffi::g_value_init(value.to_glib_none_mut().0, <u64 as StaticType>::static_type().to_glib());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "master-clock-id".to_glib_none().0, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    pub fn connect_property_domain_notify<F: Fn(&PtpClock) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&PtpClock) + Send + Sync + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::domain",
                transmute(notify_domain_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    pub fn connect_property_grandmaster_clock_id_notify<F: Fn(&PtpClock) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&PtpClock) + Send + Sync + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::grandmaster-clock-id",
                transmute(notify_grandmaster_clock_id_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    pub fn connect_property_internal_clock_notify<F: Fn(&PtpClock) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&PtpClock) + Send + Sync + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::internal-clock",
                transmute(notify_internal_clock_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    pub fn connect_property_master_clock_id_notify<F: Fn(&PtpClock) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&PtpClock) + Send + Sync + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::master-clock-id",
                transmute(notify_master_clock_id_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe impl Send for PtpClock {}
unsafe impl Sync for PtpClock {}

unsafe extern "C" fn notify_domain_trampoline(this: *mut ffi::GstPtpClock, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer) {
    callback_guard!();
    let f: &&(Fn(&PtpClock) + Send + Sync + 'static) = transmute(f);
    f(&from_glib_borrow(this))
}

unsafe extern "C" fn notify_grandmaster_clock_id_trampoline(this: *mut ffi::GstPtpClock, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer) {
    callback_guard!();
    let f: &&(Fn(&PtpClock) + Send + Sync + 'static) = transmute(f);
    f(&from_glib_borrow(this))
}

unsafe extern "C" fn notify_internal_clock_trampoline(this: *mut ffi::GstPtpClock, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer) {
    callback_guard!();
    let f: &&(Fn(&PtpClock) + Send + Sync + 'static) = transmute(f);
    f(&from_glib_borrow(this))
}

unsafe extern "C" fn notify_master_clock_id_trampoline(this: *mut ffi::GstPtpClock, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer) {
    callback_guard!();
    let f: &&(Fn(&PtpClock) + Send + Sync + 'static) = transmute(f);
    f(&from_glib_borrow(this))
}
