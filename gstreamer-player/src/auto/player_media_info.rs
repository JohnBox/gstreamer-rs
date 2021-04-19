// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git)
// DO NOT EDIT

use crate::PlayerAudioInfo;
use crate::PlayerStreamInfo;
use crate::PlayerSubtitleInfo;
use crate::PlayerVideoInfo;
use glib::translate::*;

glib::wrapper! {
    pub struct PlayerMediaInfo(Object<ffi::GstPlayerMediaInfo, ffi::GstPlayerMediaInfoClass>);

    match fn {
        type_ => || ffi::gst_player_media_info_get_type(),
    }
}

impl PlayerMediaInfo {
    #[doc(alias = "gst_player_media_info_get_audio_streams")]
    pub fn audio_streams(&self) -> Vec<PlayerAudioInfo> {
        unsafe {
            FromGlibPtrContainer::from_glib_none(ffi::gst_player_media_info_get_audio_streams(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gst_player_media_info_get_container_format")]
    pub fn container_format(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::gst_player_media_info_get_container_format(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gst_player_media_info_get_duration")]
    pub fn duration(&self) -> gst::ClockTime {
        unsafe {
            from_glib(ffi::gst_player_media_info_get_duration(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gst_player_media_info_get_image_sample")]
    pub fn image_sample(&self) -> Option<gst::Sample> {
        unsafe {
            from_glib_none(ffi::gst_player_media_info_get_image_sample(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gst_player_media_info_get_number_of_audio_streams")]
    pub fn number_of_audio_streams(&self) -> u32 {
        unsafe { ffi::gst_player_media_info_get_number_of_audio_streams(self.to_glib_none().0) }
    }

    #[doc(alias = "gst_player_media_info_get_number_of_streams")]
    pub fn number_of_streams(&self) -> u32 {
        unsafe { ffi::gst_player_media_info_get_number_of_streams(self.to_glib_none().0) }
    }

    #[doc(alias = "gst_player_media_info_get_number_of_subtitle_streams")]
    pub fn number_of_subtitle_streams(&self) -> u32 {
        unsafe { ffi::gst_player_media_info_get_number_of_subtitle_streams(self.to_glib_none().0) }
    }

    #[doc(alias = "gst_player_media_info_get_number_of_video_streams")]
    pub fn number_of_video_streams(&self) -> u32 {
        unsafe { ffi::gst_player_media_info_get_number_of_video_streams(self.to_glib_none().0) }
    }

    #[doc(alias = "gst_player_media_info_get_stream_list")]
    pub fn stream_list(&self) -> Vec<PlayerStreamInfo> {
        unsafe {
            FromGlibPtrContainer::from_glib_none(ffi::gst_player_media_info_get_stream_list(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gst_player_media_info_get_subtitle_streams")]
    pub fn subtitle_streams(&self) -> Vec<PlayerSubtitleInfo> {
        unsafe {
            FromGlibPtrContainer::from_glib_none(ffi::gst_player_media_info_get_subtitle_streams(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gst_player_media_info_get_tags")]
    pub fn tags(&self) -> Option<gst::TagList> {
        unsafe { from_glib_none(ffi::gst_player_media_info_get_tags(self.to_glib_none().0)) }
    }

    #[doc(alias = "gst_player_media_info_get_title")]
    pub fn title(&self) -> Option<glib::GString> {
        unsafe { from_glib_none(ffi::gst_player_media_info_get_title(self.to_glib_none().0)) }
    }

    #[doc(alias = "gst_player_media_info_get_uri")]
    pub fn uri(&self) -> glib::GString {
        unsafe { from_glib_none(ffi::gst_player_media_info_get_uri(self.to_glib_none().0)) }
    }

    #[doc(alias = "gst_player_media_info_get_video_streams")]
    pub fn video_streams(&self) -> Vec<PlayerVideoInfo> {
        unsafe {
            FromGlibPtrContainer::from_glib_none(ffi::gst_player_media_info_get_video_streams(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gst_player_media_info_is_live")]
    pub fn is_live(&self) -> bool {
        unsafe { from_glib(ffi::gst_player_media_info_is_live(self.to_glib_none().0)) }
    }

    #[doc(alias = "gst_player_media_info_is_seekable")]
    pub fn is_seekable(&self) -> bool {
        unsafe {
            from_glib(ffi::gst_player_media_info_is_seekable(
                self.to_glib_none().0,
            ))
        }
    }
}

unsafe impl Send for PlayerMediaInfo {}
unsafe impl Sync for PlayerMediaInfo {}
