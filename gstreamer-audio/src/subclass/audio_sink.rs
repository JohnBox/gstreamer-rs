// Take a look at the license at the top of the repository in the LICENSE file.

use glib::prelude::*;
use glib::subclass::prelude::*;
use glib::translate::*;

use gst::subclass::prelude::*;
use gst::LoggableError;
use gst_base::subclass::prelude::*;

use crate::AudioRingBufferSpec;
use crate::AudioSink;

pub trait AudioSinkImpl: AudioSinkImplExt + BaseSinkImpl {
    fn close(&self, sink: &Self::Type) -> Result<(), LoggableError> {
        self.parent_close(sink)
    }

    fn delay(&self, sink: &Self::Type) -> u32 {
        self.parent_delay(sink)
    }

    fn open(&self, sink: &Self::Type) -> Result<(), LoggableError> {
        self.parent_open(sink)
    }

    fn prepare(
        &self,
        sink: &Self::Type,
        spec: &mut AudioRingBufferSpec,
    ) -> Result<(), LoggableError> {
        AudioSinkImplExt::parent_prepare(self, sink, spec)
    }

    fn unprepare(&self, sink: &Self::Type) -> Result<(), LoggableError> {
        self.parent_unprepare(sink)
    }

    fn write(&self, sink: &Self::Type, audio_data: &[u8]) -> Result<i32, LoggableError> {
        self.parent_write(sink, audio_data)
    }

    fn reset(&self, sink: &Self::Type) {
        self.parent_reset(sink)
    }
}

pub trait AudioSinkImplExt: ObjectSubclass {
    fn parent_close(&self, sink: &Self::Type) -> Result<(), LoggableError>;
    fn parent_delay(&self, sink: &Self::Type) -> u32;
    fn parent_open(&self, sink: &Self::Type) -> Result<(), LoggableError>;
    fn parent_prepare(
        &self,
        sink: &Self::Type,
        spec: &mut AudioRingBufferSpec,
    ) -> Result<(), LoggableError>;
    fn parent_unprepare(&self, sink: &Self::Type) -> Result<(), LoggableError>;
    fn parent_write(&self, sink: &Self::Type, audio_data: &[u8]) -> Result<i32, LoggableError>;
    fn parent_reset(&self, sink: &Self::Type);
}

impl<T: AudioSinkImpl> AudioSinkImplExt for T {
    fn parent_close(&self, sink: &Self::Type) -> Result<(), LoggableError> {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut ffi::GstAudioSinkClass;
            let f = match (*parent_class).close {
                None => return Ok(()),
                Some(f) => f,
            };
            gst::gst_result_from_gboolean!(
                f(sink.unsafe_cast_ref::<AudioSink>().to_glib_none().0),
                gst::CAT_RUST,
                "Failed to close element using the parent function"
            )
        }
    }

    fn parent_delay(&self, sink: &Self::Type) -> u32 {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut ffi::GstAudioSinkClass;
            let f = match (*parent_class).delay {
                Some(f) => f,
                None => return 0,
            };
            f(sink.unsafe_cast_ref::<AudioSink>().to_glib_none().0)
        }
    }

    fn parent_open(&self, sink: &Self::Type) -> Result<(), LoggableError> {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut ffi::GstAudioSinkClass;
            let f = match (*parent_class).open {
                Some(f) => f,
                None => return Ok(()),
            };
            gst::gst_result_from_gboolean!(
                f(sink.unsafe_cast_ref::<AudioSink>().to_glib_none().0),
                gst::CAT_RUST,
                "Failed to open element using the parent function"
            )
        }
    }

    fn parent_prepare(
        &self,
        sink: &Self::Type,
        spec: &mut AudioRingBufferSpec,
    ) -> Result<(), LoggableError> {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut ffi::GstAudioSinkClass;
            let f = match (*parent_class).prepare {
                Some(f) => f,
                None => return Ok(()),
            };
            gst::gst_result_from_gboolean!(
                f(
                    sink.unsafe_cast_ref::<AudioSink>().to_glib_none().0,
                    &mut spec.0
                ),
                gst::CAT_RUST,
                "Failed to prepare element using the parent function"
            )
        }
    }

    fn parent_unprepare(&self, sink: &Self::Type) -> Result<(), LoggableError> {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut ffi::GstAudioSinkClass;
            let f = match (*parent_class).unprepare {
                Some(f) => f,
                None => {
                    return Err(gst::gst_loggable_error!(
                        gst::CAT_RUST,
                        "Unprepare is not implemented!"
                    ))
                }
            };
            gst::gst_result_from_gboolean!(
                f(sink.unsafe_cast_ref::<AudioSink>().to_glib_none().0),
                gst::CAT_RUST,
                "Failed to unprepare element using the parent function"
            )
        }
    }

    fn parent_write(&self, sink: &Self::Type, buffer: &[u8]) -> Result<i32, LoggableError> {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut ffi::GstAudioSinkClass;
            let f = match (*parent_class).write {
                Some(f) => f,
                None => return Ok(-1),
            };
            let buffer_ptr = buffer.as_ptr() as *const _ as *mut _;
            let ret = f(
                sink.unsafe_cast_ref::<AudioSink>().to_glib_none().0,
                buffer_ptr,
                buffer.len() as u32,
            );
            if ret > 0 {
                Ok(ret)
            } else {
                Err(gst::gst_loggable_error!(
                    gst::CAT_RUST,
                    "Failed to write using the parent function"
                ))
            }
        }
    }

    fn parent_reset(&self, sink: &Self::Type) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut ffi::GstAudioSinkClass;
            if let Some(f) = (*parent_class).reset {
                f(sink.unsafe_cast_ref::<AudioSink>().to_glib_none().0)
            }
        }
    }
}

unsafe impl<T: AudioSinkImpl> IsSubclassable<T> for AudioSink
where
    <T as ObjectSubclass>::Instance: PanicPoison,
{
    fn override_vfuncs(klass: &mut glib::Class<Self>) {
        <gst_base::BaseSink as IsSubclassable<T>>::override_vfuncs(klass);
        let klass = klass.as_mut();
        klass.close = Some(audiosink_close::<T>);
        klass.delay = Some(audiosink_delay::<T>);
        klass.open = Some(audiosink_open::<T>);
        klass.prepare = Some(audiosink_prepare::<T>);
        klass.unprepare = Some(audiosink_unprepare::<T>);
        klass.write = Some(audiosink_write::<T>);
        klass.reset = Some(audiosink_reset::<T>);
    }
}

unsafe extern "C" fn audiosink_close<T: AudioSinkImpl>(
    ptr: *mut ffi::GstAudioSink,
) -> glib::ffi::gboolean
where
    T::Instance: PanicPoison,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<AudioSink> = from_glib_borrow(ptr);

    gst::gst_panic_to_error!(&wrap, &instance.panicked(), false, {
        match imp.close(wrap.unsafe_cast_ref()) {
            Ok(()) => true,
            Err(err) => {
                err.log_with_object(&*wrap);
                false
            }
        }
    })
    .to_glib()
}

unsafe extern "C" fn audiosink_delay<T: AudioSinkImpl>(ptr: *mut ffi::GstAudioSink) -> u32
where
    T::Instance: PanicPoison,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<AudioSink> = from_glib_borrow(ptr);

    gst::gst_panic_to_error!(&wrap, &instance.panicked(), 0, {
        imp.delay(wrap.unsafe_cast_ref())
    })
}

unsafe extern "C" fn audiosink_open<T: AudioSinkImpl>(
    ptr: *mut ffi::GstAudioSink,
) -> glib::ffi::gboolean
where
    T::Instance: PanicPoison,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<AudioSink> = from_glib_borrow(ptr);

    gst::gst_panic_to_error!(&wrap, &instance.panicked(), false, {
        match imp.open(wrap.unsafe_cast_ref()) {
            Ok(()) => true,
            Err(err) => {
                err.log_with_object(&*wrap);
                false
            }
        }
    })
    .to_glib()
}

unsafe extern "C" fn audiosink_prepare<T: AudioSinkImpl>(
    ptr: *mut ffi::GstAudioSink,
    spec: *mut ffi::GstAudioRingBufferSpec,
) -> glib::ffi::gboolean
where
    T::Instance: PanicPoison,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<AudioSink> = from_glib_borrow(ptr);

    let spec = &mut *(spec as *mut AudioRingBufferSpec);

    gst::gst_panic_to_error!(&wrap, &instance.panicked(), false, {
        match AudioSinkImpl::prepare(imp, wrap.unsafe_cast_ref(), spec) {
            Ok(()) => true,
            Err(err) => {
                err.log_with_object(&*wrap);
                false
            }
        }
    })
    .to_glib()
}

unsafe extern "C" fn audiosink_unprepare<T: AudioSinkImpl>(
    ptr: *mut ffi::GstAudioSink,
) -> glib::ffi::gboolean
where
    T::Instance: PanicPoison,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<AudioSink> = from_glib_borrow(ptr);

    gst::gst_panic_to_error!(&wrap, &instance.panicked(), false, {
        match imp.unprepare(wrap.unsafe_cast_ref()) {
            Ok(()) => true,
            Err(err) => {
                err.log_with_object(&*wrap);
                false
            }
        }
    })
    .to_glib()
}

unsafe extern "C" fn audiosink_write<T: AudioSinkImpl>(
    ptr: *mut ffi::GstAudioSink,
    data: glib::ffi::gpointer,
    length: u32,
) -> i32
where
    T::Instance: PanicPoison,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<AudioSink> = from_glib_borrow(ptr);
    let data_slice = std::slice::from_raw_parts(data as *const u8, length as usize);

    gst::gst_panic_to_error!(&wrap, &instance.panicked(), -1, {
        imp.write(wrap.unsafe_cast_ref(), data_slice).unwrap_or(-1)
    })
}

unsafe extern "C" fn audiosink_reset<T: AudioSinkImpl>(ptr: *mut ffi::GstAudioSink)
where
    T::Instance: PanicPoison,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<AudioSink> = from_glib_borrow(ptr);

    gst::gst_panic_to_error!(&wrap, &instance.panicked(), (), {
        imp.reset(wrap.unsafe_cast_ref());
    });
}
