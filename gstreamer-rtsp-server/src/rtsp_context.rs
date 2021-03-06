// Take a look at the license at the top of the repository in the LICENSE file.

use std::ffi::CStr;
use std::ptr;

use glib::translate::*;

#[derive(Debug, PartialEq, Eq)]
#[doc(alias = "GstRTSPContext")]
pub struct RTSPContext(ptr::NonNull<ffi::GstRTSPContext>);

impl RTSPContext {
    pub fn with_current_context<F: FnOnce(&RTSPContext) -> T, T>(func: F) -> Option<T> {
        unsafe {
            let ptr = ffi::gst_rtsp_context_get_current();
            if ptr.is_null() {
                return None;
            }

            let ctx = RTSPContext(ptr::NonNull::new_unchecked(ptr));
            Some(func(&ctx))
        }
    }


    pub fn uri_query(&self) -> Option<String> {
        let ctx = self.0.as_ptr();
        let query = unsafe { (*(*ctx).uri).query };
        if query.is_null() {
            return None;
        }
        unsafe { Some(CStr::from_ptr(query).to_string_lossy().into_owned()) }
    }

    // TODO: Add various getters for all the contained fields as needed
}

#[doc(hidden)]
impl FromGlibPtrBorrow<*mut ffi::GstRTSPContext> for RTSPContext {
    #[inline]
    unsafe fn from_glib_borrow(ptr: *mut ffi::GstRTSPContext) -> Borrowed<Self> {
        assert!(!ptr.is_null());
        Borrowed::new(RTSPContext(ptr::NonNull::new_unchecked(ptr)))
    }
}

#[doc(hidden)]
impl<'a> ToGlibPtr<'a, *mut ffi::GstRTSPContext> for RTSPContext {
    type Storage = &'a RTSPContext;

    fn to_glib_none(&'a self) -> Stash<'a, *mut ffi::GstRTSPContext, Self> {
        Stash(self.0.as_ptr(), self)
    }

    fn to_glib_full(&self) -> *mut ffi::GstRTSPContext {
        unimplemented!()
    }
}
