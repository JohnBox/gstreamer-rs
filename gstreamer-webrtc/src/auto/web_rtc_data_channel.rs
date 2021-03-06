// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git)
// DO NOT EDIT

use crate::WebRTCDataChannelState;
use crate::WebRTCPriorityType;
use glib::object::ObjectExt;
use glib::object::ObjectType as ObjectType_;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::StaticType;
use glib::ToValue;
use std::boxed::Box as Box_;
use std::mem::transmute;

glib::wrapper! {
    #[doc(alias = "GstWebRTCDataChannel")]
    pub struct WebRTCDataChannel(Object<ffi::GstWebRTCDataChannel, ffi::GstWebRTCDataChannelClass>);

    match fn {
        type_ => || ffi::gst_webrtc_data_channel_get_type(),
    }
}

impl WebRTCDataChannel {
    #[doc(alias = "gst_webrtc_data_channel_close")]
    pub fn close(&self) {
        unsafe {
            ffi::gst_webrtc_data_channel_close(self.to_glib_none().0);
        }
    }

    #[doc(alias = "gst_webrtc_data_channel_send_data")]
    pub fn send_data(&self, data: Option<&glib::Bytes>) {
        unsafe {
            ffi::gst_webrtc_data_channel_send_data(self.to_glib_none().0, data.to_glib_none().0);
        }
    }

    #[doc(alias = "gst_webrtc_data_channel_send_string")]
    pub fn send_string(&self, str: Option<&str>) {
        unsafe {
            ffi::gst_webrtc_data_channel_send_string(self.to_glib_none().0, str.to_glib_none().0);
        }
    }

    #[doc(alias = "buffered-amount")]
    pub fn buffered_amount(&self) -> u64 {
        unsafe {
            let mut value = glib::Value::from_type(<u64 as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"buffered-amount\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `buffered-amount` getter")
        }
    }

    #[doc(alias = "buffered-amount-low-threshold")]
    pub fn buffered_amount_low_threshold(&self) -> u64 {
        unsafe {
            let mut value = glib::Value::from_type(<u64 as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"buffered-amount-low-threshold\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `buffered-amount-low-threshold` getter")
        }
    }

    #[doc(alias = "buffered-amount-low-threshold")]
    pub fn set_buffered_amount_low_threshold(&self, buffered_amount_low_threshold: u64) {
        unsafe {
            glib::gobject_ffi::g_object_set_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"buffered-amount-low-threshold\0".as_ptr() as *const _,
                buffered_amount_low_threshold.to_value().to_glib_none().0,
            );
        }
    }

    pub fn id(&self) -> i32 {
        unsafe {
            let mut value = glib::Value::from_type(<i32 as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"id\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value.get().expect("Return Value for property `id` getter")
        }
    }

    pub fn label(&self) -> Option<glib::GString> {
        unsafe {
            let mut value = glib::Value::from_type(<glib::GString as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"label\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `label` getter")
        }
    }

    #[doc(alias = "max-packet-lifetime")]
    pub fn max_packet_lifetime(&self) -> i32 {
        unsafe {
            let mut value = glib::Value::from_type(<i32 as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"max-packet-lifetime\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `max-packet-lifetime` getter")
        }
    }

    #[doc(alias = "max-retransmits")]
    pub fn max_retransmits(&self) -> i32 {
        unsafe {
            let mut value = glib::Value::from_type(<i32 as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"max-retransmits\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `max-retransmits` getter")
        }
    }

    pub fn is_negotiated(&self) -> bool {
        unsafe {
            let mut value = glib::Value::from_type(<bool as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"negotiated\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `negotiated` getter")
        }
    }

    pub fn is_ordered(&self) -> bool {
        unsafe {
            let mut value = glib::Value::from_type(<bool as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"ordered\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `ordered` getter")
        }
    }

    pub fn priority(&self) -> WebRTCPriorityType {
        unsafe {
            let mut value =
                glib::Value::from_type(<WebRTCPriorityType as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"priority\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `priority` getter")
        }
    }

    pub fn protocol(&self) -> Option<glib::GString> {
        unsafe {
            let mut value = glib::Value::from_type(<glib::GString as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"protocol\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `protocol` getter")
        }
    }

    #[doc(alias = "ready-state")]
    pub fn ready_state(&self) -> WebRTCDataChannelState {
        unsafe {
            let mut value =
                glib::Value::from_type(<WebRTCDataChannelState as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"ready-state\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `ready-state` getter")
        }
    }

    #[doc(alias = "close")]
    pub fn connect_close<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn close_trampoline<F: Fn(&WebRTCDataChannel) + Send + Sync + 'static>(
            this: *mut ffi::GstWebRTCDataChannel,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"close\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    close_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn emit_close(&self) {
        let _ = unsafe {
            glib::Object::from_glib_borrow(self.as_ptr() as *mut glib::gobject_ffi::GObject)
                .emit_by_name("close", &[])
                .unwrap()
        };
    }

    #[doc(alias = "on-buffered-amount-low")]
    pub fn connect_on_buffered_amount_low<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn on_buffered_amount_low_trampoline<
            F: Fn(&WebRTCDataChannel) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstWebRTCDataChannel,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"on-buffered-amount-low\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    on_buffered_amount_low_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "on-close")]
    pub fn connect_on_close<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn on_close_trampoline<
            F: Fn(&WebRTCDataChannel) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstWebRTCDataChannel,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"on-close\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    on_close_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "on-error")]
    pub fn connect_on_error<F: Fn(&Self, &glib::Error) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn on_error_trampoline<
            F: Fn(&WebRTCDataChannel, &glib::Error) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstWebRTCDataChannel,
            error: *mut glib::ffi::GError,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), &from_glib_borrow(error))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"on-error\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    on_error_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "on-message-data")]
    pub fn connect_on_message_data<F: Fn(&Self, Option<&glib::Bytes>) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn on_message_data_trampoline<
            F: Fn(&WebRTCDataChannel, Option<&glib::Bytes>) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstWebRTCDataChannel,
            data: *mut glib::ffi::GBytes,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                &from_glib_borrow(this),
                Option::<glib::Bytes>::from_glib_borrow(data)
                    .as_ref()
                    .as_ref(),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"on-message-data\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    on_message_data_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "on-message-string")]
    pub fn connect_on_message_string<F: Fn(&Self, Option<&str>) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn on_message_string_trampoline<
            F: Fn(&WebRTCDataChannel, Option<&str>) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstWebRTCDataChannel,
            data: *mut libc::c_char,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                &from_glib_borrow(this),
                Option::<glib::GString>::from_glib_borrow(data)
                    .as_ref()
                    .as_deref(),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"on-message-string\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    on_message_string_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "on-open")]
    pub fn connect_on_open<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn on_open_trampoline<
            F: Fn(&WebRTCDataChannel) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstWebRTCDataChannel,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"on-open\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    on_open_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "send-data")]
    pub fn connect_send_data<F: Fn(&Self, Option<&glib::Bytes>) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn send_data_trampoline<
            F: Fn(&WebRTCDataChannel, Option<&glib::Bytes>) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstWebRTCDataChannel,
            data: *mut glib::ffi::GBytes,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                &from_glib_borrow(this),
                Option::<glib::Bytes>::from_glib_borrow(data)
                    .as_ref()
                    .as_ref(),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"send-data\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    send_data_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn emit_send_data(&self, data: Option<&glib::Bytes>) {
        let _ = unsafe {
            glib::Object::from_glib_borrow(self.as_ptr() as *mut glib::gobject_ffi::GObject)
                .emit_by_name("send-data", &[&data])
                .unwrap()
        };
    }

    #[doc(alias = "send-string")]
    pub fn connect_send_string<F: Fn(&Self, Option<&str>) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn send_string_trampoline<
            F: Fn(&WebRTCDataChannel, Option<&str>) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstWebRTCDataChannel,
            data: *mut libc::c_char,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                &from_glib_borrow(this),
                Option::<glib::GString>::from_glib_borrow(data)
                    .as_ref()
                    .as_deref(),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"send-string\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    send_string_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn emit_send_string(&self, data: Option<&str>) {
        let _ = unsafe {
            glib::Object::from_glib_borrow(self.as_ptr() as *mut glib::gobject_ffi::GObject)
                .emit_by_name("send-string", &[&data])
                .unwrap()
        };
    }

    #[doc(alias = "buffered-amount")]
    pub fn connect_buffered_amount_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_buffered_amount_trampoline<
            F: Fn(&WebRTCDataChannel) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstWebRTCDataChannel,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::buffered-amount\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_buffered_amount_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "buffered-amount-low-threshold")]
    pub fn connect_buffered_amount_low_threshold_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_buffered_amount_low_threshold_trampoline<
            F: Fn(&WebRTCDataChannel) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstWebRTCDataChannel,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::buffered-amount-low-threshold\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_buffered_amount_low_threshold_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "ready-state")]
    pub fn connect_ready_state_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_ready_state_trampoline<
            F: Fn(&WebRTCDataChannel) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstWebRTCDataChannel,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::ready-state\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_ready_state_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

unsafe impl Send for WebRTCDataChannel {}
unsafe impl Sync for WebRTCDataChannel {}
