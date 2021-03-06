// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git)
// DO NOT EDIT

mod gl_base_filter;
pub use self::gl_base_filter::{GLBaseFilter, NONE_GL_BASE_FILTER};

mod gl_base_memory_allocator;
pub use self::gl_base_memory_allocator::{GLBaseMemoryAllocator, NONE_GL_BASE_MEMORY_ALLOCATOR};

#[cfg(any(feature = "v1_18", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
mod gl_base_src;
#[cfg(any(feature = "v1_18", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
pub use self::gl_base_src::{GLBaseSrc, NONE_GL_BASE_SRC};

mod gl_color_convert;
pub use self::gl_color_convert::GLColorConvert;

mod gl_context;
pub use self::gl_context::{GLContext, NONE_GL_CONTEXT};

mod gl_display;
pub use self::gl_display::{GLDisplay, NONE_GL_DISPLAY};

mod gl_filter;
pub use self::gl_filter::{GLFilter, NONE_GL_FILTER};

mod gl_framebuffer;
pub use self::gl_framebuffer::{GLFramebuffer, NONE_GL_FRAMEBUFFER};

mod gl_memory_allocator;
pub use self::gl_memory_allocator::{GLMemoryAllocator, NONE_GL_MEMORY_ALLOCATOR};

mod gl_overlay_compositor;
pub use self::gl_overlay_compositor::GLOverlayCompositor;

mod glsl_stage;
pub use self::glsl_stage::GLSLStage;

mod gl_shader;
pub use self::gl_shader::GLShader;

mod gl_upload;
pub use self::gl_upload::GLUpload;

mod gl_view_convert;
pub use self::gl_view_convert::GLViewConvert;

mod gl_window;
pub use self::gl_window::{GLWindow, NONE_GL_WINDOW};

mod gl_allocation_params;
pub use self::gl_allocation_params::GLAllocationParams;

mod gl_video_allocation_params;
pub use self::gl_video_allocation_params::GLVideoAllocationParams;

mod enums;
#[cfg(any(feature = "v1_20", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_20")))]
pub use self::enums::GLConfigCaveat;
pub use self::enums::GLContextError;
pub use self::enums::GLFormat;
pub use self::enums::GLQueryType;
pub use self::enums::GLSLError;
pub use self::enums::GLSLVersion;
pub use self::enums::GLStereoDownmix;
pub use self::enums::GLTextureTarget;
pub use self::enums::GLUploadReturn;
pub use self::enums::GLWindowError;

mod flags;
#[cfg(any(feature = "v1_20", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_20")))]
pub use self::flags::GLConfigSurfaceType;
pub use self::flags::GLDisplayType;
pub use self::flags::GLPlatform;
pub use self::flags::GLSLProfile;
pub use self::flags::GLAPI;

#[doc(hidden)]
pub mod traits {
    pub use super::gl_base_filter::GLBaseFilterExt;
    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    pub use super::gl_base_src::GLBaseSrcExt;
    pub use super::gl_context::GLContextExt;
    pub use super::gl_display::GLDisplayExt;
    pub use super::gl_filter::GLFilterExt;
    pub use super::gl_framebuffer::GLFramebufferExt;
    pub use super::gl_window::GLWindowExt;
}
