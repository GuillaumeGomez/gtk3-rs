// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::{Component, Object};
use glib::{prelude::*, translate::*};

glib::wrapper! {
    #[doc(alias = "AtkPlug")]
    pub struct Plug(Object<ffi::AtkPlug, ffi::AtkPlugClass>) @extends Object, @implements Component;

    match fn {
        type_ => || ffi::atk_plug_get_type(),
    }
}

impl Plug {
    pub const NONE: Option<&'static Plug> = None;

    #[doc(alias = "atk_plug_new")]
    pub fn new() -> Plug {
        assert_initialized_main_thread!();
        unsafe { Object::from_glib_full(ffi::atk_plug_new()).unsafe_cast() }
    }
}

impl Default for Plug {
    fn default() -> Self {
        Self::new()
    }
}

mod sealed {
    pub trait Sealed {}
    impl<T: super::IsA<super::Plug>> Sealed for T {}
}

pub trait AtkPlugExt: IsA<Plug> + sealed::Sealed + 'static {
    #[doc(alias = "atk_plug_get_id")]
    #[doc(alias = "get_id")]
    fn id(&self) -> Option<glib::GString> {
        unsafe { from_glib_full(ffi::atk_plug_get_id(self.as_ref().to_glib_none().0)) }
    }
}

impl<O: IsA<Plug>> AtkPlugExt for O {}
