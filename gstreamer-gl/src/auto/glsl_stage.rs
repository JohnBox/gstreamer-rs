// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use Error;
use GLContext;
use GLSLProfile;
use GLSLVersion;
use ffi;
use glib::GString;
use glib::object::IsA;
use glib::translate::*;
use gst;
use std::ptr;

glib_wrapper! {
    pub struct GLSLStage(Object<ffi::GstGLSLStage, ffi::GstGLSLStageClass, GLSLStageClass>) @extends gst::Object;

    match fn {
        get_type => || ffi::gst_glsl_stage_get_type(),
    }
}

impl GLSLStage {
    pub fn new<P: IsA<GLContext>>(context: &P, type_: u32) -> GLSLStage {
        skip_assert_initialized!();
        unsafe {
            from_glib_none(ffi::gst_glsl_stage_new(context.as_ref().to_glib_none().0, type_))
        }
    }

    pub fn new_default_fragment<P: IsA<GLContext>>(context: &P) -> GLSLStage {
        skip_assert_initialized!();
        unsafe {
            from_glib_none(ffi::gst_glsl_stage_new_default_fragment(context.as_ref().to_glib_none().0))
        }
    }

    pub fn new_default_vertex<P: IsA<GLContext>>(context: &P) -> GLSLStage {
        skip_assert_initialized!();
        unsafe {
            from_glib_none(ffi::gst_glsl_stage_new_default_vertex(context.as_ref().to_glib_none().0))
        }
    }

    pub fn new_with_string<P: IsA<GLContext>>(context: &P, type_: u32, version: GLSLVersion, profile: GLSLProfile, str: &str) -> GLSLStage {
        skip_assert_initialized!();
        unsafe {
            from_glib_none(ffi::gst_glsl_stage_new_with_string(context.as_ref().to_glib_none().0, type_, version.to_glib(), profile.to_glib(), str.to_glib_none().0))
        }
    }

    pub fn new_with_strings<P: IsA<GLContext>>(context: &P, type_: u32, version: GLSLVersion, profile: GLSLProfile, str: &[&str]) -> GLSLStage {
        skip_assert_initialized!();
        let n_strings = str.len() as i32;
        unsafe {
            from_glib_none(ffi::gst_glsl_stage_new_with_strings(context.as_ref().to_glib_none().0, type_, version.to_glib(), profile.to_glib(), n_strings, str.to_glib_none().0))
        }
    }

    pub fn compile(&self) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::gst_glsl_stage_compile(self.to_glib_none().0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    pub fn get_handle(&self) -> u32 {
        unsafe {
            ffi::gst_glsl_stage_get_handle(self.to_glib_none().0)
        }
    }

    pub fn get_profile(&self) -> GLSLProfile {
        unsafe {
            from_glib(ffi::gst_glsl_stage_get_profile(self.to_glib_none().0))
        }
    }

    pub fn get_shader_type(&self) -> u32 {
        unsafe {
            ffi::gst_glsl_stage_get_shader_type(self.to_glib_none().0)
        }
    }

    pub fn get_version(&self) -> GLSLVersion {
        unsafe {
            from_glib(ffi::gst_glsl_stage_get_version(self.to_glib_none().0))
        }
    }

    pub fn set_strings(&self, version: GLSLVersion, profile: GLSLProfile, str: &[&str]) -> bool {
        let n_strings = str.len() as i32;
        unsafe {
            from_glib(ffi::gst_glsl_stage_set_strings(self.to_glib_none().0, version.to_glib(), profile.to_glib(), n_strings, str.to_glib_none().0))
        }
    }
}

unsafe impl Send for GLSLStage {}
unsafe impl Sync for GLSLStage {}

pub const NONE_GLSL_STAGE: Option<&GLSLStage> = None;