// This file was generated by gir (25bba39) from gir-files (71d73f0)
// DO NOT EDIT

use Widget;
use ffi;
use glib::object::Downcast;
use glib::translate::*;

glib_wrapper! {
    pub struct DrawingArea(Object<ffi::GtkDrawingArea>): Widget;

    match fn {
        get_type => || ffi::gtk_drawing_area_get_type(),
    }
}

impl DrawingArea {
    pub fn new() -> DrawingArea {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_drawing_area_new()).downcast_unchecked()
        }
    }
}
