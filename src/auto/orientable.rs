// This file was generated by gir (25bba39) from gir-files (71d73f0)
// DO NOT EDIT

use Orientation;
use ffi;
use glib::object::IsA;
use glib::translate::*;

glib_wrapper! {
    pub struct Orientable(Object<ffi::GtkOrientable>);

    match fn {
        get_type => || ffi::gtk_orientable_get_type(),
    }
}

pub trait OrientableExt {
    fn get_orientation(&self) -> Orientation;

    fn set_orientation(&self, orientation: Orientation);
}

impl<O: IsA<Orientable>> OrientableExt for O {
    fn get_orientation(&self) -> Orientation {
        unsafe {
            from_glib(ffi::gtk_orientable_get_orientation(self.to_glib_none().0))
        }
    }

    fn set_orientation(&self, orientation: Orientation) {
        unsafe {
            ffi::gtk_orientable_set_orientation(self.to_glib_none().0, orientation.to_glib());
        }
    }
}
