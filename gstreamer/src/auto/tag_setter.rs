// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git)
// DO NOT EDIT

use crate::Element;
use crate::Object;
use crate::TagList;
use crate::TagMergeMode;
use glib::object::IsA;
use glib::translate::*;

glib::wrapper! {
    pub struct TagSetter(Interface<ffi::GstTagSetter, ffi::GstTagSetterInterface>) @requires Element, Object;

    match fn {
        type_ => || ffi::gst_tag_setter_get_type(),
    }
}

unsafe impl Send for TagSetter {}
unsafe impl Sync for TagSetter {}

pub const NONE_TAG_SETTER: Option<&TagSetter> = None;

pub trait TagSetterExt: 'static {
    //#[doc(alias = "gst_tag_setter_add_tag_valist")]
    //fn add_tag_valist(&self, mode: TagMergeMode, tag: &str, var_args: /*Unknown conversion*//*Unimplemented*/Unsupported);

    //#[doc(alias = "gst_tag_setter_add_tag_valist_values")]
    //fn add_tag_valist_values(&self, mode: TagMergeMode, tag: &str, var_args: /*Unknown conversion*//*Unimplemented*/Unsupported);

    //#[doc(alias = "gst_tag_setter_add_tag_values")]
    //fn add_tag_values(&self, mode: TagMergeMode, tag: &str, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs);

    //#[doc(alias = "gst_tag_setter_add_tags")]
    //fn add_tags(&self, mode: TagMergeMode, tag: &str, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs);

    #[doc(alias = "gst_tag_setter_get_tag_list")]
    fn tag_list(&self) -> Option<TagList>;

    #[doc(alias = "gst_tag_setter_get_tag_merge_mode")]
    fn tag_merge_mode(&self) -> TagMergeMode;

    #[doc(alias = "gst_tag_setter_merge_tags")]
    fn merge_tags(&self, list: &TagList, mode: TagMergeMode);

    #[doc(alias = "gst_tag_setter_reset_tags")]
    fn reset_tags(&self);

    #[doc(alias = "gst_tag_setter_set_tag_merge_mode")]
    fn set_tag_merge_mode(&self, mode: TagMergeMode);
}

impl<O: IsA<TagSetter>> TagSetterExt for O {
    //fn add_tag_valist(&self, mode: TagMergeMode, tag: &str, var_args: /*Unknown conversion*//*Unimplemented*/Unsupported) {
    //    unsafe { TODO: call ffi:gst_tag_setter_add_tag_valist() }
    //}

    //fn add_tag_valist_values(&self, mode: TagMergeMode, tag: &str, var_args: /*Unknown conversion*//*Unimplemented*/Unsupported) {
    //    unsafe { TODO: call ffi:gst_tag_setter_add_tag_valist_values() }
    //}

    //fn add_tag_values(&self, mode: TagMergeMode, tag: &str, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) {
    //    unsafe { TODO: call ffi:gst_tag_setter_add_tag_values() }
    //}

    //fn add_tags(&self, mode: TagMergeMode, tag: &str, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) {
    //    unsafe { TODO: call ffi:gst_tag_setter_add_tags() }
    //}

    fn tag_list(&self) -> Option<TagList> {
        unsafe {
            from_glib_none(ffi::gst_tag_setter_get_tag_list(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn tag_merge_mode(&self) -> TagMergeMode {
        unsafe {
            from_glib(ffi::gst_tag_setter_get_tag_merge_mode(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn merge_tags(&self, list: &TagList, mode: TagMergeMode) {
        unsafe {
            ffi::gst_tag_setter_merge_tags(
                self.as_ref().to_glib_none().0,
                list.to_glib_none().0,
                mode.to_glib(),
            );
        }
    }

    fn reset_tags(&self) {
        unsafe {
            ffi::gst_tag_setter_reset_tags(self.as_ref().to_glib_none().0);
        }
    }

    fn set_tag_merge_mode(&self, mode: TagMergeMode) {
        unsafe {
            ffi::gst_tag_setter_set_tag_merge_mode(self.as_ref().to_glib_none().0, mode.to_glib());
        }
    }
}
