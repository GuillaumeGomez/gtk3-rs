// This file was generated by gir (25bba39) from gir-files (71d73f0)
// DO NOT EDIT

use AccelGroup;
use Container;
use MenuItem;
use MenuShell;
use ScrollType;
use Widget;
use ffi;
use gdk;
use gio;
use glib::Value;
use glib::object::Downcast;
use glib::object::IsA;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use std::boxed::Box as Box_;
use std::mem::transmute;

glib_wrapper! {
    pub struct Menu(Object<ffi::GtkMenu>): MenuShell, Container, Widget;

    match fn {
        get_type => || ffi::gtk_menu_get_type(),
    }
}

impl Menu {
    pub fn new() -> Menu {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_menu_new()).downcast_unchecked()
        }
    }

    pub fn new_from_model<T: IsA<gio::MenuModel>>(model: &T) -> Menu {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_menu_new_from_model(model.to_glib_none().0)).downcast_unchecked()
        }
    }

    pub fn attach<T: IsA<Widget>>(&self, child: &T, left_attach: u32, right_attach: u32, top_attach: u32, bottom_attach: u32) {
        unsafe {
            ffi::gtk_menu_attach(self.to_glib_none().0, child.to_glib_none().0, left_attach, right_attach, top_attach, bottom_attach);
        }
    }

    //pub fn attach_to_widget<T: IsA<Widget>>(&self, attach_widget: &T, detacher: /*Unknown conversion*//*Unimplemented*/MenuDetachFunc) {
    //    unsafe { TODO: call ffi::gtk_menu_attach_to_widget() }
    //}

    pub fn detach(&self) {
        unsafe {
            ffi::gtk_menu_detach(self.to_glib_none().0);
        }
    }

    pub fn get_accel_group(&self) -> Option<AccelGroup> {
        unsafe {
            from_glib_none(ffi::gtk_menu_get_accel_group(self.to_glib_none().0))
        }
    }

    pub fn get_accel_path(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_menu_get_accel_path(self.to_glib_none().0))
        }
    }

    pub fn get_active(&self) -> Option<Widget> {
        unsafe {
            from_glib_none(ffi::gtk_menu_get_active(self.to_glib_none().0))
        }
    }

    pub fn get_attach_widget(&self) -> Option<Widget> {
        unsafe {
            from_glib_none(ffi::gtk_menu_get_attach_widget(self.to_glib_none().0))
        }
    }

    pub fn get_monitor(&self) -> i32 {
        unsafe {
            ffi::gtk_menu_get_monitor(self.to_glib_none().0)
        }
    }

    pub fn get_reserve_toggle_size(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_menu_get_reserve_toggle_size(self.to_glib_none().0))
        }
    }

    pub fn get_tearoff_state(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_menu_get_tearoff_state(self.to_glib_none().0))
        }
    }

    pub fn get_title(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_menu_get_title(self.to_glib_none().0))
        }
    }

    //pub fn place_on_monitor(&self, monitor: /*Ignored*/&gdk::Monitor) {
    //    unsafe { TODO: call ffi::gtk_menu_place_on_monitor() }
    //}

    pub fn popdown(&self) {
        unsafe {
            ffi::gtk_menu_popdown(self.to_glib_none().0);
        }
    }

    //pub fn popup<T: IsA<Widget>, U: IsA<Widget>>(&self, parent_menu_shell: Option<&T>, parent_menu_item: Option<&U>, func: /*Unknown conversion*//*Unimplemented*/MenuPositionFunc, data: /*Unimplemented*/Option<Fundamental: Pointer>, button: u32, activate_time: u32) {
    //    unsafe { TODO: call ffi::gtk_menu_popup() }
    //}

    #[cfg(feature = "v3_22")]
    pub fn popup_at_pointer(&self, trigger_event: Option<&gdk::Event>) {
        unsafe {
            ffi::gtk_menu_popup_at_pointer(self.to_glib_none().0, trigger_event.to_glib_none().0);
        }
    }

    #[cfg(feature = "v3_22")]
    pub fn popup_at_rect(&self, rect_window: &gdk::Window, rect: &gdk::Rectangle, rect_anchor: gdk::Gravity, menu_anchor: gdk::Gravity, trigger_event: Option<&gdk::Event>) {
        unsafe {
            ffi::gtk_menu_popup_at_rect(self.to_glib_none().0, rect_window.to_glib_none().0, rect.to_glib_none().0, rect_anchor.to_glib(), menu_anchor.to_glib(), trigger_event.to_glib_none().0);
        }
    }

    #[cfg(feature = "v3_22")]
    pub fn popup_at_widget<T: IsA<Widget>>(&self, widget: &T, widget_anchor: gdk::Gravity, menu_anchor: gdk::Gravity, trigger_event: Option<&gdk::Event>) {
        unsafe {
            ffi::gtk_menu_popup_at_widget(self.to_glib_none().0, widget.to_glib_none().0, widget_anchor.to_glib(), menu_anchor.to_glib(), trigger_event.to_glib_none().0);
        }
    }

    //pub fn popup_for_device<T: IsA<gdk::Device>, U: IsA<Widget>, V: IsA<Widget>>(&self, device: Option<&T>, parent_menu_shell: Option<&U>, parent_menu_item: Option<&V>, func: /*Unknown conversion*//*Unimplemented*/MenuPositionFunc, data: /*Unimplemented*/Option<Fundamental: Pointer>, destroy: /*Unknown conversion*//*Unimplemented*/DestroyNotify, button: u32, activate_time: u32) {
    //    unsafe { TODO: call ffi::gtk_menu_popup_for_device() }
    //}

    pub fn reorder_child<T: IsA<Widget>>(&self, child: &T, position: i32) {
        unsafe {
            ffi::gtk_menu_reorder_child(self.to_glib_none().0, child.to_glib_none().0, position);
        }
    }

    pub fn reposition(&self) {
        unsafe {
            ffi::gtk_menu_reposition(self.to_glib_none().0);
        }
    }

    pub fn set_accel_group(&self, accel_group: Option<&AccelGroup>) {
        unsafe {
            ffi::gtk_menu_set_accel_group(self.to_glib_none().0, accel_group.to_glib_none().0);
        }
    }

    pub fn set_accel_path(&self, accel_path: Option<&str>) {
        unsafe {
            ffi::gtk_menu_set_accel_path(self.to_glib_none().0, accel_path.to_glib_none().0);
        }
    }

    pub fn set_active(&self, index: u32) {
        unsafe {
            ffi::gtk_menu_set_active(self.to_glib_none().0, index);
        }
    }

    pub fn set_monitor(&self, monitor_num: i32) {
        unsafe {
            ffi::gtk_menu_set_monitor(self.to_glib_none().0, monitor_num);
        }
    }

    pub fn set_reserve_toggle_size(&self, reserve_toggle_size: bool) {
        unsafe {
            ffi::gtk_menu_set_reserve_toggle_size(self.to_glib_none().0, reserve_toggle_size.to_glib());
        }
    }

    pub fn set_screen(&self, screen: Option<&gdk::Screen>) {
        unsafe {
            ffi::gtk_menu_set_screen(self.to_glib_none().0, screen.to_glib_none().0);
        }
    }

    pub fn set_tearoff_state(&self, torn_off: bool) {
        unsafe {
            ffi::gtk_menu_set_tearoff_state(self.to_glib_none().0, torn_off.to_glib());
        }
    }

    pub fn set_title(&self, title: &str) {
        unsafe {
            ffi::gtk_menu_set_title(self.to_glib_none().0, title.to_glib_none().0);
        }
    }

    pub fn get_item_bottom_attach<T: IsA<MenuItem> + IsA<Widget>>(&self, item: &T) -> i32 {
        let mut value = Value::from(&0);
        unsafe {
            ffi::gtk_container_child_get_property(self.to_glib_none().0, item.to_glib_none().0, "bottom-attach".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    pub fn set_item_bottom_attach<T: IsA<MenuItem> + IsA<Widget>>(&self, item: &T, bottom_attach: i32) {
        unsafe {
            ffi::gtk_container_child_set_property(self.to_glib_none().0, item.to_glib_none().0, "bottom-attach".to_glib_none().0, Value::from(&bottom_attach).to_glib_none().0);
        }
    }

    pub fn get_item_left_attach<T: IsA<MenuItem> + IsA<Widget>>(&self, item: &T) -> i32 {
        let mut value = Value::from(&0);
        unsafe {
            ffi::gtk_container_child_get_property(self.to_glib_none().0, item.to_glib_none().0, "left-attach".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    pub fn set_item_left_attach<T: IsA<MenuItem> + IsA<Widget>>(&self, item: &T, left_attach: i32) {
        unsafe {
            ffi::gtk_container_child_set_property(self.to_glib_none().0, item.to_glib_none().0, "left-attach".to_glib_none().0, Value::from(&left_attach).to_glib_none().0);
        }
    }

    pub fn get_item_right_attach<T: IsA<MenuItem> + IsA<Widget>>(&self, item: &T) -> i32 {
        let mut value = Value::from(&0);
        unsafe {
            ffi::gtk_container_child_get_property(self.to_glib_none().0, item.to_glib_none().0, "right-attach".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    pub fn set_item_right_attach<T: IsA<MenuItem> + IsA<Widget>>(&self, item: &T, right_attach: i32) {
        unsafe {
            ffi::gtk_container_child_set_property(self.to_glib_none().0, item.to_glib_none().0, "right-attach".to_glib_none().0, Value::from(&right_attach).to_glib_none().0);
        }
    }

    pub fn get_item_top_attach<T: IsA<MenuItem> + IsA<Widget>>(&self, item: &T) -> i32 {
        let mut value = Value::from(&0);
        unsafe {
            ffi::gtk_container_child_get_property(self.to_glib_none().0, item.to_glib_none().0, "top-attach".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    pub fn set_item_top_attach<T: IsA<MenuItem> + IsA<Widget>>(&self, item: &T, top_attach: i32) {
        unsafe {
            ffi::gtk_container_child_set_property(self.to_glib_none().0, item.to_glib_none().0, "top-attach".to_glib_none().0, Value::from(&top_attach).to_glib_none().0);
        }
    }

    pub fn get_for_attach_widget<T: IsA<Widget>>(widget: &T) -> Vec<Widget> {
        skip_assert_initialized!();
        unsafe {
            FromGlibPtrContainer::from_glib_none(ffi::gtk_menu_get_for_attach_widget(widget.to_glib_none().0))
        }
    }

    pub fn connect_move_scroll<F: Fn(&Menu, ScrollType) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Menu, ScrollType) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "move-scroll",
                transmute(move_scroll_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    //#[cfg(feature = "v3_22")]
    //pub fn connect_popped_up<Unsupported or ignored types>(&self, f: F) -> u64 {
    //    Unimplemented flipped_rect: *.Pointer
    //    Unimplemented final_rect: *.Pointer
    //}
}

unsafe extern "C" fn move_scroll_trampoline(this: *mut ffi::GtkMenu, scroll_type: ffi::GtkScrollType, f: glib_ffi::gpointer) {
    callback_guard!();
    let f: &Box_<Fn(&Menu, ScrollType) + 'static> = transmute(f);
    f(&from_glib_none(this), from_glib(scroll_type))
}
