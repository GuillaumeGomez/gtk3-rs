// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::{Buildable, TreeDragDest, TreeDragSource, TreeIter, TreeModel, TreeSortable};
use glib::{prelude::*, translate::*};

glib::wrapper! {
    #[doc(alias = "GtkListStore")]
    pub struct ListStore(Object<ffi::GtkListStore, ffi::GtkListStoreClass>) @implements Buildable, TreeDragDest, TreeDragSource, TreeModel, TreeSortable;

    match fn {
        type_ => || ffi::gtk_list_store_get_type(),
    }
}

impl ListStore {
    pub const NONE: Option<&'static ListStore> = None;

    //#[doc(alias = "gtk_list_store_new")]
    //pub fn new(n_columns: i32, : /*Unknown conversion*//*Unimplemented*/Basic: VarArgs) -> ListStore {
    //    unsafe { TODO: call ffi:gtk_list_store_new() }
    //}

    //#[doc(alias = "gtk_list_store_newv")]
    //pub fn newv(types: /*Unimplemented*/&CArray TypeId { ns_id: 0, id: 30 }) -> ListStore {
    //    unsafe { TODO: call ffi:gtk_list_store_newv() }
    //}
}

mod sealed {
    pub trait Sealed {}
    impl<T: super::IsA<super::ListStore>> Sealed for T {}
}

pub trait GtkListStoreExt: IsA<ListStore> + sealed::Sealed + 'static {
    #[doc(alias = "gtk_list_store_append")]
    fn append(&self) -> TreeIter {
        unsafe {
            let mut iter = TreeIter::uninitialized();
            ffi::gtk_list_store_append(self.as_ref().to_glib_none().0, iter.to_glib_none_mut().0);
            iter
        }
    }

    #[doc(alias = "gtk_list_store_clear")]
    fn clear(&self) {
        unsafe {
            ffi::gtk_list_store_clear(self.as_ref().to_glib_none().0);
        }
    }

    #[doc(alias = "gtk_list_store_insert")]
    fn insert(&self, position: i32) -> TreeIter {
        unsafe {
            let mut iter = TreeIter::uninitialized();
            ffi::gtk_list_store_insert(
                self.as_ref().to_glib_none().0,
                iter.to_glib_none_mut().0,
                position,
            );
            iter
        }
    }

    #[doc(alias = "gtk_list_store_insert_after")]
    fn insert_after(&self, sibling: Option<&TreeIter>) -> TreeIter {
        unsafe {
            let mut iter = TreeIter::uninitialized();
            ffi::gtk_list_store_insert_after(
                self.as_ref().to_glib_none().0,
                iter.to_glib_none_mut().0,
                mut_override(sibling.to_glib_none().0),
            );
            iter
        }
    }

    #[doc(alias = "gtk_list_store_insert_before")]
    fn insert_before(&self, sibling: Option<&TreeIter>) -> TreeIter {
        unsafe {
            let mut iter = TreeIter::uninitialized();
            ffi::gtk_list_store_insert_before(
                self.as_ref().to_glib_none().0,
                iter.to_glib_none_mut().0,
                mut_override(sibling.to_glib_none().0),
            );
            iter
        }
    }

    //#[doc(alias = "gtk_list_store_insert_with_values")]
    //fn insert_with_values(&self, position: i32, : /*Unknown conversion*//*Unimplemented*/Basic: VarArgs) -> TreeIter {
    //    unsafe { TODO: call ffi:gtk_list_store_insert_with_values() }
    //}

    //#[doc(alias = "gtk_list_store_insert_with_valuesv")]
    //fn insert_with_valuesv(&self, position: i32, columns: &[i32], values: &[&glib::Value]) -> TreeIter {
    //    unsafe { TODO: call ffi:gtk_list_store_insert_with_valuesv() }
    //}

    #[doc(alias = "gtk_list_store_iter_is_valid")]
    fn iter_is_valid(&self, iter: &TreeIter) -> bool {
        unsafe {
            from_glib(ffi::gtk_list_store_iter_is_valid(
                self.as_ref().to_glib_none().0,
                mut_override(iter.to_glib_none().0),
            ))
        }
    }

    #[doc(alias = "gtk_list_store_move_after")]
    fn move_after(&self, iter: &TreeIter, position: Option<&TreeIter>) {
        unsafe {
            ffi::gtk_list_store_move_after(
                self.as_ref().to_glib_none().0,
                mut_override(iter.to_glib_none().0),
                mut_override(position.to_glib_none().0),
            );
        }
    }

    #[doc(alias = "gtk_list_store_move_before")]
    fn move_before(&self, iter: &TreeIter, position: Option<&TreeIter>) {
        unsafe {
            ffi::gtk_list_store_move_before(
                self.as_ref().to_glib_none().0,
                mut_override(iter.to_glib_none().0),
                mut_override(position.to_glib_none().0),
            );
        }
    }

    #[doc(alias = "gtk_list_store_prepend")]
    fn prepend(&self) -> TreeIter {
        unsafe {
            let mut iter = TreeIter::uninitialized();
            ffi::gtk_list_store_prepend(self.as_ref().to_glib_none().0, iter.to_glib_none_mut().0);
            iter
        }
    }

    #[doc(alias = "gtk_list_store_remove")]
    fn remove(&self, iter: &TreeIter) -> bool {
        unsafe {
            from_glib(ffi::gtk_list_store_remove(
                self.as_ref().to_glib_none().0,
                mut_override(iter.to_glib_none().0),
            ))
        }
    }

    //#[doc(alias = "gtk_list_store_set_column_types")]
    //fn set_column_types(&self, types: /*Unimplemented*/&CArray TypeId { ns_id: 0, id: 30 }) {
    //    unsafe { TODO: call ffi:gtk_list_store_set_column_types() }
    //}

    //#[doc(alias = "gtk_list_store_set_valist")]
    //fn set_valist(&self, iter: &TreeIter, var_args: /*Unknown conversion*//*Unimplemented*/Unsupported) {
    //    unsafe { TODO: call ffi:gtk_list_store_set_valist() }
    //}

    //#[doc(alias = "gtk_list_store_set_valuesv")]
    //fn set_valuesv(&self, iter: &TreeIter, columns: &[i32], values: &[&glib::Value]) {
    //    unsafe { TODO: call ffi:gtk_list_store_set_valuesv() }
    //}

    #[doc(alias = "gtk_list_store_swap")]
    fn swap(&self, a: &TreeIter, b: &TreeIter) {
        unsafe {
            ffi::gtk_list_store_swap(
                self.as_ref().to_glib_none().0,
                mut_override(a.to_glib_none().0),
                mut_override(b.to_glib_none().0),
            );
        }
    }
}

impl<O: IsA<ListStore>> GtkListStoreExt for O {}
