// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::Window;
use glib::translate::*;

glib::wrapper! {
    #[doc(alias = "GdkDrawingContext")]
    pub struct DrawingContext(Object<ffi::GdkDrawingContext, ffi::GdkDrawingContextClass>);

    match fn {
        type_ => || ffi::gdk_drawing_context_get_type(),
    }
}

impl DrawingContext {
    #[doc(alias = "gdk_drawing_context_get_cairo_context")]
    #[doc(alias = "get_cairo_context")]
    pub fn cairo_context(&self) -> Option<cairo::Context> {
        unsafe {
            from_glib_none(ffi::gdk_drawing_context_get_cairo_context(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gdk_drawing_context_get_clip")]
    #[doc(alias = "get_clip")]
    pub fn clip(&self) -> Option<cairo::Region> {
        unsafe { from_glib_full(ffi::gdk_drawing_context_get_clip(self.to_glib_none().0)) }
    }

    #[doc(alias = "gdk_drawing_context_get_window")]
    #[doc(alias = "get_window")]
    pub fn window(&self) -> Option<Window> {
        unsafe { from_glib_none(ffi::gdk_drawing_context_get_window(self.to_glib_none().0)) }
    }

    #[doc(alias = "gdk_drawing_context_is_valid")]
    pub fn is_valid(&self) -> bool {
        unsafe { from_glib(ffi::gdk_drawing_context_is_valid(self.to_glib_none().0)) }
    }
}
