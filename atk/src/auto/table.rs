// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::Object;
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::boxed::Box as Box_;

glib::wrapper! {
    #[doc(alias = "AtkTable")]
    pub struct Table(Interface<ffi::AtkTable, ffi::AtkTableIface>);

    match fn {
        type_ => || ffi::atk_table_get_type(),
    }
}

impl Table {
    pub const NONE: Option<&'static Table> = None;
}

mod sealed {
    pub trait Sealed {}
    impl<T: super::IsA<super::Table>> Sealed for T {}
}

pub trait TableExt: IsA<Table> + sealed::Sealed + 'static {
    #[doc(alias = "atk_table_add_column_selection")]
    fn add_column_selection(&self, column: i32) -> bool {
        unsafe {
            from_glib(ffi::atk_table_add_column_selection(
                self.as_ref().to_glib_none().0,
                column,
            ))
        }
    }

    #[doc(alias = "atk_table_add_row_selection")]
    fn add_row_selection(&self, row: i32) -> bool {
        unsafe {
            from_glib(ffi::atk_table_add_row_selection(
                self.as_ref().to_glib_none().0,
                row,
            ))
        }
    }

    #[doc(alias = "atk_table_get_caption")]
    #[doc(alias = "get_caption")]
    fn caption(&self) -> Option<Object> {
        unsafe { from_glib_none(ffi::atk_table_get_caption(self.as_ref().to_glib_none().0)) }
    }

    #[doc(alias = "atk_table_get_column_at_index")]
    #[doc(alias = "get_column_at_index")]
    fn column_at_index(&self, index_: i32) -> i32 {
        unsafe { ffi::atk_table_get_column_at_index(self.as_ref().to_glib_none().0, index_) }
    }

    #[doc(alias = "atk_table_get_column_description")]
    #[doc(alias = "get_column_description")]
    fn column_description(&self, column: i32) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::atk_table_get_column_description(
                self.as_ref().to_glib_none().0,
                column,
            ))
        }
    }

    #[doc(alias = "atk_table_get_column_extent_at")]
    #[doc(alias = "get_column_extent_at")]
    fn column_extent_at(&self, row: i32, column: i32) -> i32 {
        unsafe { ffi::atk_table_get_column_extent_at(self.as_ref().to_glib_none().0, row, column) }
    }

    #[doc(alias = "atk_table_get_column_header")]
    #[doc(alias = "get_column_header")]
    fn column_header(&self, column: i32) -> Option<Object> {
        unsafe {
            from_glib_none(ffi::atk_table_get_column_header(
                self.as_ref().to_glib_none().0,
                column,
            ))
        }
    }

    #[doc(alias = "atk_table_get_index_at")]
    #[doc(alias = "get_index_at")]
    fn index_at(&self, row: i32, column: i32) -> i32 {
        unsafe { ffi::atk_table_get_index_at(self.as_ref().to_glib_none().0, row, column) }
    }

    #[doc(alias = "atk_table_get_n_columns")]
    #[doc(alias = "get_n_columns")]
    fn n_columns(&self) -> i32 {
        unsafe { ffi::atk_table_get_n_columns(self.as_ref().to_glib_none().0) }
    }

    #[doc(alias = "atk_table_get_n_rows")]
    #[doc(alias = "get_n_rows")]
    fn n_rows(&self) -> i32 {
        unsafe { ffi::atk_table_get_n_rows(self.as_ref().to_glib_none().0) }
    }

    #[doc(alias = "atk_table_get_row_at_index")]
    #[doc(alias = "get_row_at_index")]
    fn row_at_index(&self, index_: i32) -> i32 {
        unsafe { ffi::atk_table_get_row_at_index(self.as_ref().to_glib_none().0, index_) }
    }

    #[doc(alias = "atk_table_get_row_description")]
    #[doc(alias = "get_row_description")]
    fn row_description(&self, row: i32) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::atk_table_get_row_description(
                self.as_ref().to_glib_none().0,
                row,
            ))
        }
    }

    #[doc(alias = "atk_table_get_row_extent_at")]
    #[doc(alias = "get_row_extent_at")]
    fn row_extent_at(&self, row: i32, column: i32) -> i32 {
        unsafe { ffi::atk_table_get_row_extent_at(self.as_ref().to_glib_none().0, row, column) }
    }

    #[doc(alias = "atk_table_get_row_header")]
    #[doc(alias = "get_row_header")]
    fn row_header(&self, row: i32) -> Option<Object> {
        unsafe {
            from_glib_none(ffi::atk_table_get_row_header(
                self.as_ref().to_glib_none().0,
                row,
            ))
        }
    }

    #[doc(alias = "atk_table_get_summary")]
    #[doc(alias = "get_summary")]
    fn summary(&self) -> Option<Object> {
        unsafe { from_glib_full(ffi::atk_table_get_summary(self.as_ref().to_glib_none().0)) }
    }

    #[doc(alias = "atk_table_is_column_selected")]
    fn is_column_selected(&self, column: i32) -> bool {
        unsafe {
            from_glib(ffi::atk_table_is_column_selected(
                self.as_ref().to_glib_none().0,
                column,
            ))
        }
    }

    #[doc(alias = "atk_table_is_row_selected")]
    fn is_row_selected(&self, row: i32) -> bool {
        unsafe {
            from_glib(ffi::atk_table_is_row_selected(
                self.as_ref().to_glib_none().0,
                row,
            ))
        }
    }

    #[doc(alias = "atk_table_is_selected")]
    fn is_selected(&self, row: i32, column: i32) -> bool {
        unsafe {
            from_glib(ffi::atk_table_is_selected(
                self.as_ref().to_glib_none().0,
                row,
                column,
            ))
        }
    }

    #[doc(alias = "atk_table_ref_at")]
    fn ref_at(&self, row: i32, column: i32) -> Option<Object> {
        unsafe {
            from_glib_full(ffi::atk_table_ref_at(
                self.as_ref().to_glib_none().0,
                row,
                column,
            ))
        }
    }

    #[doc(alias = "atk_table_remove_column_selection")]
    fn remove_column_selection(&self, column: i32) -> bool {
        unsafe {
            from_glib(ffi::atk_table_remove_column_selection(
                self.as_ref().to_glib_none().0,
                column,
            ))
        }
    }

    #[doc(alias = "atk_table_remove_row_selection")]
    fn remove_row_selection(&self, row: i32) -> bool {
        unsafe {
            from_glib(ffi::atk_table_remove_row_selection(
                self.as_ref().to_glib_none().0,
                row,
            ))
        }
    }

    #[doc(alias = "atk_table_set_caption")]
    fn set_caption(&self, caption: &impl IsA<Object>) {
        unsafe {
            ffi::atk_table_set_caption(
                self.as_ref().to_glib_none().0,
                caption.as_ref().to_glib_none().0,
            );
        }
    }

    #[doc(alias = "atk_table_set_column_description")]
    fn set_column_description(&self, column: i32, description: &str) {
        unsafe {
            ffi::atk_table_set_column_description(
                self.as_ref().to_glib_none().0,
                column,
                description.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "atk_table_set_column_header")]
    fn set_column_header(&self, column: i32, header: &impl IsA<Object>) {
        unsafe {
            ffi::atk_table_set_column_header(
                self.as_ref().to_glib_none().0,
                column,
                header.as_ref().to_glib_none().0,
            );
        }
    }

    #[doc(alias = "atk_table_set_row_description")]
    fn set_row_description(&self, row: i32, description: &str) {
        unsafe {
            ffi::atk_table_set_row_description(
                self.as_ref().to_glib_none().0,
                row,
                description.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "atk_table_set_row_header")]
    fn set_row_header(&self, row: i32, header: &impl IsA<Object>) {
        unsafe {
            ffi::atk_table_set_row_header(
                self.as_ref().to_glib_none().0,
                row,
                header.as_ref().to_glib_none().0,
            );
        }
    }

    #[doc(alias = "atk_table_set_summary")]
    fn set_summary(&self, accessible: &impl IsA<Object>) {
        unsafe {
            ffi::atk_table_set_summary(
                self.as_ref().to_glib_none().0,
                accessible.as_ref().to_glib_none().0,
            );
        }
    }

    #[doc(alias = "column-deleted")]
    fn connect_column_deleted<F: Fn(&Self, i32, i32) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn column_deleted_trampoline<
            P: IsA<Table>,
            F: Fn(&P, i32, i32) + 'static,
        >(
            this: *mut ffi::AtkTable,
            arg1: libc::c_int,
            arg2: libc::c_int,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Table::from_glib_borrow(this).unsafe_cast_ref(), arg1, arg2)
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"column-deleted\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(
                    column_deleted_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "column-inserted")]
    fn connect_column_inserted<F: Fn(&Self, i32, i32) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn column_inserted_trampoline<
            P: IsA<Table>,
            F: Fn(&P, i32, i32) + 'static,
        >(
            this: *mut ffi::AtkTable,
            arg1: libc::c_int,
            arg2: libc::c_int,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Table::from_glib_borrow(this).unsafe_cast_ref(), arg1, arg2)
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"column-inserted\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(
                    column_inserted_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "column-reordered")]
    fn connect_column_reordered<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn column_reordered_trampoline<P: IsA<Table>, F: Fn(&P) + 'static>(
            this: *mut ffi::AtkTable,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Table::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"column-reordered\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(
                    column_reordered_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "model-changed")]
    fn connect_model_changed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn model_changed_trampoline<P: IsA<Table>, F: Fn(&P) + 'static>(
            this: *mut ffi::AtkTable,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Table::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"model-changed\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(
                    model_changed_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "row-deleted")]
    fn connect_row_deleted<F: Fn(&Self, i32, i32) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn row_deleted_trampoline<
            P: IsA<Table>,
            F: Fn(&P, i32, i32) + 'static,
        >(
            this: *mut ffi::AtkTable,
            arg1: libc::c_int,
            arg2: libc::c_int,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Table::from_glib_borrow(this).unsafe_cast_ref(), arg1, arg2)
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"row-deleted\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(
                    row_deleted_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "row-inserted")]
    fn connect_row_inserted<F: Fn(&Self, i32, i32) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn row_inserted_trampoline<
            P: IsA<Table>,
            F: Fn(&P, i32, i32) + 'static,
        >(
            this: *mut ffi::AtkTable,
            arg1: libc::c_int,
            arg2: libc::c_int,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Table::from_glib_borrow(this).unsafe_cast_ref(), arg1, arg2)
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"row-inserted\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(
                    row_inserted_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "row-reordered")]
    fn connect_row_reordered<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn row_reordered_trampoline<P: IsA<Table>, F: Fn(&P) + 'static>(
            this: *mut ffi::AtkTable,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Table::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"row-reordered\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(
                    row_reordered_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl<O: IsA<Table>> TableExt for O {}
