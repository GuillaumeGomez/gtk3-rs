// Take a look at the license at the top of the repository in the LICENSE file.

use glib::subclass::prelude::*;
use glib::translate::*;
use glib::Cast;

use super::bin::BinImpl;
use crate::Button;

pub trait ButtonImpl: ButtonImplExt + BinImpl {
    fn activate(&self) {
        self.parent_activate()
    }

    fn clicked(&self) {
        self.parent_clicked()
    }
}

pub trait ButtonImplExt: ObjectSubclass {
    fn parent_activate(&self);
    fn parent_clicked(&self);
}

impl<T: ButtonImpl> ButtonImplExt for T {
    fn parent_activate(&self) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkButtonClass;
            if let Some(f) = (*parent_class).activate {
                f(self.instance().unsafe_cast_ref::<Button>().to_glib_none().0)
            }
        }
    }

    fn parent_clicked(&self) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkButtonClass;
            if let Some(f) = (*parent_class).clicked {
                f(self.instance().unsafe_cast_ref::<Button>().to_glib_none().0)
            }
        }
    }
}

unsafe impl<T: ButtonImpl> IsSubclassable<T> for Button {
    fn class_init(class: &mut glib::Class<Self>) {
        Self::parent_class_init::<T>(class);

        if !crate::rt::is_initialized() {
            panic!("GTK has to be initialized first");
        }

        let klass = class.as_mut();
        klass.activate = Some(button_activate::<T>);
        klass.clicked = Some(button_clicked::<T>);
    }
}

unsafe extern "C" fn button_activate<T: ButtonImpl>(ptr: *mut ffi::GtkButton) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    imp.activate()
}

unsafe extern "C" fn button_clicked<T: ButtonImpl>(ptr: *mut ffi::GtkButton) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    imp.clicked()
}
