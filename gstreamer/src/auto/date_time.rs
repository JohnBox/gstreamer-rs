// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git)
// DO NOT EDIT

use glib::translate::*;

glib::wrapper! {
    #[derive()]
    pub struct DateTime(Shared<ffi::GstDateTime>);

    match fn {
        ref => |ptr| ffi::gst_date_time_ref(ptr),
        unref => |ptr| ffi::gst_date_time_unref(ptr),
        type_ => || ffi::gst_date_time_get_type(),
    }
}

impl DateTime {
    #[doc(alias = "gst_date_time_new_from_g_date_time")]
    pub fn from_g_date_time(dt: &glib::DateTime) -> DateTime {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(ffi::gst_date_time_new_from_g_date_time(dt.to_glib_full())) }
    }

    #[doc(alias = "gst_date_time_new_from_iso8601_string")]
    pub fn from_iso8601_string(string: &str) -> Result<DateTime, glib::BoolError> {
        assert_initialized_main_thread!();
        unsafe {
            Option::<_>::from_glib_full(ffi::gst_date_time_new_from_iso8601_string(
                string.to_glib_none().0,
            ))
            .ok_or_else(|| glib::bool_error!("Failed to create DateTime from ISO-8601 string"))
        }
    }

    #[doc(alias = "gst_date_time_new_from_unix_epoch_local_time")]
    pub fn from_unix_epoch_local_time(secs: i64) -> Result<DateTime, glib::BoolError> {
        assert_initialized_main_thread!();
        unsafe {
            Option::<_>::from_glib_full(ffi::gst_date_time_new_from_unix_epoch_local_time(secs))
                .ok_or_else(|| glib::bool_error!("Can't create DateTime from UNIX epoch"))
        }
    }

    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    #[doc(alias = "gst_date_time_new_from_unix_epoch_local_time_usecs")]
    pub fn from_unix_epoch_local_time_usecs(usecs: i64) -> Result<DateTime, glib::BoolError> {
        assert_initialized_main_thread!();
        unsafe {
            Option::<_>::from_glib_full(ffi::gst_date_time_new_from_unix_epoch_local_time_usecs(
                usecs,
            ))
            .ok_or_else(|| glib::bool_error!("Can't create DateTime from UNIX epoch"))
        }
    }

    #[doc(alias = "gst_date_time_new_from_unix_epoch_utc")]
    pub fn from_unix_epoch_utc(secs: i64) -> Result<DateTime, glib::BoolError> {
        assert_initialized_main_thread!();
        unsafe {
            Option::<_>::from_glib_full(ffi::gst_date_time_new_from_unix_epoch_utc(secs))
                .ok_or_else(|| glib::bool_error!("Can't create DateTime from UNIX epoch"))
        }
    }

    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    #[doc(alias = "gst_date_time_new_from_unix_epoch_utc_usecs")]
    pub fn from_unix_epoch_utc_usecs(usecs: i64) -> Result<DateTime, glib::BoolError> {
        assert_initialized_main_thread!();
        unsafe {
            Option::<_>::from_glib_full(ffi::gst_date_time_new_from_unix_epoch_utc_usecs(usecs))
                .ok_or_else(|| glib::bool_error!("Can't create DateTime from UNIX epoch"))
        }
    }

    #[doc(alias = "gst_date_time_new_now_local_time")]
    pub fn new_now_local_time() -> DateTime {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(ffi::gst_date_time_new_now_local_time()) }
    }

    #[doc(alias = "gst_date_time_new_now_utc")]
    pub fn new_now_utc() -> DateTime {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(ffi::gst_date_time_new_now_utc()) }
    }

    #[doc(alias = "gst_date_time_get_year")]
    pub fn year(&self) -> i32 {
        unsafe { ffi::gst_date_time_get_year(self.to_glib_none().0) }
    }

    #[doc(alias = "gst_date_time_has_day")]
    pub fn has_day(&self) -> bool {
        unsafe { from_glib(ffi::gst_date_time_has_day(self.to_glib_none().0)) }
    }

    #[doc(alias = "gst_date_time_has_month")]
    pub fn has_month(&self) -> bool {
        unsafe { from_glib(ffi::gst_date_time_has_month(self.to_glib_none().0)) }
    }

    #[doc(alias = "gst_date_time_has_second")]
    pub fn has_second(&self) -> bool {
        unsafe { from_glib(ffi::gst_date_time_has_second(self.to_glib_none().0)) }
    }

    #[doc(alias = "gst_date_time_has_time")]
    pub fn has_time(&self) -> bool {
        unsafe { from_glib(ffi::gst_date_time_has_time(self.to_glib_none().0)) }
    }

    #[doc(alias = "gst_date_time_has_year")]
    pub fn has_year(&self) -> bool {
        unsafe { from_glib(ffi::gst_date_time_has_year(self.to_glib_none().0)) }
    }

    #[doc(alias = "gst_date_time_to_g_date_time")]
    pub fn to_g_date_time(&self) -> Result<glib::DateTime, glib::BoolError> {
        unsafe {
            Option::<_>::from_glib_full(ffi::gst_date_time_to_g_date_time(self.to_glib_none().0))
                .ok_or_else(|| glib::bool_error!("Can't create glib::DateTime from DateTime"))
        }
    }

    #[doc(alias = "gst_date_time_to_iso8601_string")]
    pub fn to_iso8601_string(&self) -> Result<glib::GString, glib::BoolError> {
        unsafe {
            Option::<_>::from_glib_full(ffi::gst_date_time_to_iso8601_string(self.to_glib_none().0))
                .ok_or_else(|| glib::bool_error!("Failed to create ISO-8601 string from DateTime"))
        }
    }
}

unsafe impl Send for DateTime {}
unsafe impl Sync for DateTime {}
