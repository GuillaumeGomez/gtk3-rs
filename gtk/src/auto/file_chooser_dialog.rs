// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::{
    Align, Application, Bin, Buildable, Container, Dialog, FileChooser, FileChooserAction,
    FileFilter, ResizeMode, Widget, Window, WindowPosition, WindowType,
};
use glib::prelude::*;

glib::wrapper! {
    #[doc(alias = "GtkFileChooserDialog")]
    pub struct FileChooserDialog(Object<ffi::GtkFileChooserDialog, ffi::GtkFileChooserDialogClass>) @extends Dialog, Window, Bin, Container, Widget, @implements Buildable, FileChooser;

    match fn {
        type_ => || ffi::gtk_file_chooser_dialog_get_type(),
    }
}

impl FileChooserDialog {
    pub const NONE: Option<&'static FileChooserDialog> = None;

    //#[doc(alias = "gtk_file_chooser_dialog_new")]
    //pub fn new(title: Option<&str>, parent: Option<&impl IsA<Window>>, action: FileChooserAction, first_button_text: Option<&str>, : /*Unknown conversion*//*Unimplemented*/Basic: VarArgs) -> FileChooserDialog {
    //    unsafe { TODO: call ffi:gtk_file_chooser_dialog_new() }
    //}

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`FileChooserDialog`] objects.
    ///
    /// This method returns an instance of [`FileChooserDialogBuilder`](crate::builders::FileChooserDialogBuilder) which can be used to create [`FileChooserDialog`] objects.
    pub fn builder() -> FileChooserDialogBuilder {
        FileChooserDialogBuilder::new()
    }
}

impl Default for FileChooserDialog {
    fn default() -> Self {
        glib::object::Object::new::<Self>()
    }
}

// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`FileChooserDialog`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct FileChooserDialogBuilder {
    builder: glib::object::ObjectBuilder<'static, FileChooserDialog>,
}

impl FileChooserDialogBuilder {
    fn new() -> Self {
        Self {
            builder: glib::object::Object::builder(),
        }
    }

    pub fn use_header_bar(self, use_header_bar: i32) -> Self {
        Self {
            builder: self.builder.property("use-header-bar", use_header_bar),
        }
    }

    pub fn accept_focus(self, accept_focus: bool) -> Self {
        Self {
            builder: self.builder.property("accept-focus", accept_focus),
        }
    }

    pub fn application(self, application: &impl IsA<Application>) -> Self {
        Self {
            builder: self
                .builder
                .property("application", application.clone().upcast()),
        }
    }

    pub fn attached_to(self, attached_to: &impl IsA<Widget>) -> Self {
        Self {
            builder: self
                .builder
                .property("attached-to", attached_to.clone().upcast()),
        }
    }

    pub fn decorated(self, decorated: bool) -> Self {
        Self {
            builder: self.builder.property("decorated", decorated),
        }
    }

    pub fn default_height(self, default_height: i32) -> Self {
        Self {
            builder: self.builder.property("default-height", default_height),
        }
    }

    pub fn default_width(self, default_width: i32) -> Self {
        Self {
            builder: self.builder.property("default-width", default_width),
        }
    }

    pub fn deletable(self, deletable: bool) -> Self {
        Self {
            builder: self.builder.property("deletable", deletable),
        }
    }

    pub fn destroy_with_parent(self, destroy_with_parent: bool) -> Self {
        Self {
            builder: self
                .builder
                .property("destroy-with-parent", destroy_with_parent),
        }
    }

    pub fn focus_on_map(self, focus_on_map: bool) -> Self {
        Self {
            builder: self.builder.property("focus-on-map", focus_on_map),
        }
    }

    pub fn focus_visible(self, focus_visible: bool) -> Self {
        Self {
            builder: self.builder.property("focus-visible", focus_visible),
        }
    }

    pub fn gravity(self, gravity: gdk::Gravity) -> Self {
        Self {
            builder: self.builder.property("gravity", gravity),
        }
    }

    pub fn hide_titlebar_when_maximized(self, hide_titlebar_when_maximized: bool) -> Self {
        Self {
            builder: self
                .builder
                .property("hide-titlebar-when-maximized", hide_titlebar_when_maximized),
        }
    }

    pub fn icon(self, icon: &gdk_pixbuf::Pixbuf) -> Self {
        Self {
            builder: self.builder.property("icon", icon.clone()),
        }
    }

    pub fn icon_name(self, icon_name: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("icon-name", icon_name.into()),
        }
    }

    pub fn mnemonics_visible(self, mnemonics_visible: bool) -> Self {
        Self {
            builder: self
                .builder
                .property("mnemonics-visible", mnemonics_visible),
        }
    }

    pub fn modal(self, modal: bool) -> Self {
        Self {
            builder: self.builder.property("modal", modal),
        }
    }

    pub fn resizable(self, resizable: bool) -> Self {
        Self {
            builder: self.builder.property("resizable", resizable),
        }
    }

    pub fn role(self, role: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("role", role.into()),
        }
    }

    pub fn screen(self, screen: &gdk::Screen) -> Self {
        Self {
            builder: self.builder.property("screen", screen.clone()),
        }
    }

    pub fn skip_pager_hint(self, skip_pager_hint: bool) -> Self {
        Self {
            builder: self.builder.property("skip-pager-hint", skip_pager_hint),
        }
    }

    pub fn skip_taskbar_hint(self, skip_taskbar_hint: bool) -> Self {
        Self {
            builder: self
                .builder
                .property("skip-taskbar-hint", skip_taskbar_hint),
        }
    }

    pub fn startup_id(self, startup_id: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("startup-id", startup_id.into()),
        }
    }

    pub fn title(self, title: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("title", title.into()),
        }
    }

    pub fn transient_for(self, transient_for: &impl IsA<Window>) -> Self {
        Self {
            builder: self
                .builder
                .property("transient-for", transient_for.clone().upcast()),
        }
    }

    pub fn type_(self, type_: WindowType) -> Self {
        Self {
            builder: self.builder.property("type", type_),
        }
    }

    pub fn type_hint(self, type_hint: gdk::WindowTypeHint) -> Self {
        Self {
            builder: self.builder.property("type-hint", type_hint),
        }
    }

    pub fn urgency_hint(self, urgency_hint: bool) -> Self {
        Self {
            builder: self.builder.property("urgency-hint", urgency_hint),
        }
    }

    pub fn window_position(self, window_position: WindowPosition) -> Self {
        Self {
            builder: self.builder.property("window-position", window_position),
        }
    }

    pub fn border_width(self, border_width: u32) -> Self {
        Self {
            builder: self.builder.property("border-width", border_width),
        }
    }

    pub fn child(self, child: &impl IsA<Widget>) -> Self {
        Self {
            builder: self.builder.property("child", child.clone().upcast()),
        }
    }

    pub fn resize_mode(self, resize_mode: ResizeMode) -> Self {
        Self {
            builder: self.builder.property("resize-mode", resize_mode),
        }
    }

    pub fn app_paintable(self, app_paintable: bool) -> Self {
        Self {
            builder: self.builder.property("app-paintable", app_paintable),
        }
    }

    pub fn can_default(self, can_default: bool) -> Self {
        Self {
            builder: self.builder.property("can-default", can_default),
        }
    }

    pub fn can_focus(self, can_focus: bool) -> Self {
        Self {
            builder: self.builder.property("can-focus", can_focus),
        }
    }

    pub fn events(self, events: gdk::EventMask) -> Self {
        Self {
            builder: self.builder.property("events", events),
        }
    }

    pub fn expand(self, expand: bool) -> Self {
        Self {
            builder: self.builder.property("expand", expand),
        }
    }

    pub fn focus_on_click(self, focus_on_click: bool) -> Self {
        Self {
            builder: self.builder.property("focus-on-click", focus_on_click),
        }
    }

    pub fn halign(self, halign: Align) -> Self {
        Self {
            builder: self.builder.property("halign", halign),
        }
    }

    pub fn has_default(self, has_default: bool) -> Self {
        Self {
            builder: self.builder.property("has-default", has_default),
        }
    }

    pub fn has_focus(self, has_focus: bool) -> Self {
        Self {
            builder: self.builder.property("has-focus", has_focus),
        }
    }

    pub fn has_tooltip(self, has_tooltip: bool) -> Self {
        Self {
            builder: self.builder.property("has-tooltip", has_tooltip),
        }
    }

    pub fn height_request(self, height_request: i32) -> Self {
        Self {
            builder: self.builder.property("height-request", height_request),
        }
    }

    pub fn hexpand(self, hexpand: bool) -> Self {
        Self {
            builder: self.builder.property("hexpand", hexpand),
        }
    }

    pub fn hexpand_set(self, hexpand_set: bool) -> Self {
        Self {
            builder: self.builder.property("hexpand-set", hexpand_set),
        }
    }

    pub fn is_focus(self, is_focus: bool) -> Self {
        Self {
            builder: self.builder.property("is-focus", is_focus),
        }
    }

    pub fn margin(self, margin: i32) -> Self {
        Self {
            builder: self.builder.property("margin", margin),
        }
    }

    pub fn margin_bottom(self, margin_bottom: i32) -> Self {
        Self {
            builder: self.builder.property("margin-bottom", margin_bottom),
        }
    }

    pub fn margin_end(self, margin_end: i32) -> Self {
        Self {
            builder: self.builder.property("margin-end", margin_end),
        }
    }

    pub fn margin_start(self, margin_start: i32) -> Self {
        Self {
            builder: self.builder.property("margin-start", margin_start),
        }
    }

    pub fn margin_top(self, margin_top: i32) -> Self {
        Self {
            builder: self.builder.property("margin-top", margin_top),
        }
    }

    pub fn name(self, name: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("name", name.into()),
        }
    }

    pub fn no_show_all(self, no_show_all: bool) -> Self {
        Self {
            builder: self.builder.property("no-show-all", no_show_all),
        }
    }

    pub fn opacity(self, opacity: f64) -> Self {
        Self {
            builder: self.builder.property("opacity", opacity),
        }
    }

    pub fn parent(self, parent: &impl IsA<Container>) -> Self {
        Self {
            builder: self.builder.property("parent", parent.clone().upcast()),
        }
    }

    pub fn receives_default(self, receives_default: bool) -> Self {
        Self {
            builder: self.builder.property("receives-default", receives_default),
        }
    }

    pub fn sensitive(self, sensitive: bool) -> Self {
        Self {
            builder: self.builder.property("sensitive", sensitive),
        }
    }

    pub fn tooltip_markup(self, tooltip_markup: impl Into<glib::GString>) -> Self {
        Self {
            builder: self
                .builder
                .property("tooltip-markup", tooltip_markup.into()),
        }
    }

    pub fn tooltip_text(self, tooltip_text: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("tooltip-text", tooltip_text.into()),
        }
    }

    pub fn valign(self, valign: Align) -> Self {
        Self {
            builder: self.builder.property("valign", valign),
        }
    }

    pub fn vexpand(self, vexpand: bool) -> Self {
        Self {
            builder: self.builder.property("vexpand", vexpand),
        }
    }

    pub fn vexpand_set(self, vexpand_set: bool) -> Self {
        Self {
            builder: self.builder.property("vexpand-set", vexpand_set),
        }
    }

    pub fn visible(self, visible: bool) -> Self {
        Self {
            builder: self.builder.property("visible", visible),
        }
    }

    pub fn width_request(self, width_request: i32) -> Self {
        Self {
            builder: self.builder.property("width-request", width_request),
        }
    }

    pub fn action(self, action: FileChooserAction) -> Self {
        Self {
            builder: self.builder.property("action", action),
        }
    }

    pub fn create_folders(self, create_folders: bool) -> Self {
        Self {
            builder: self.builder.property("create-folders", create_folders),
        }
    }

    pub fn do_overwrite_confirmation(self, do_overwrite_confirmation: bool) -> Self {
        Self {
            builder: self
                .builder
                .property("do-overwrite-confirmation", do_overwrite_confirmation),
        }
    }

    pub fn extra_widget(self, extra_widget: &impl IsA<Widget>) -> Self {
        Self {
            builder: self
                .builder
                .property("extra-widget", extra_widget.clone().upcast()),
        }
    }

    pub fn filter(self, filter: &FileFilter) -> Self {
        Self {
            builder: self.builder.property("filter", filter.clone()),
        }
    }

    pub fn local_only(self, local_only: bool) -> Self {
        Self {
            builder: self.builder.property("local-only", local_only),
        }
    }

    pub fn preview_widget(self, preview_widget: &impl IsA<Widget>) -> Self {
        Self {
            builder: self
                .builder
                .property("preview-widget", preview_widget.clone().upcast()),
        }
    }

    pub fn preview_widget_active(self, preview_widget_active: bool) -> Self {
        Self {
            builder: self
                .builder
                .property("preview-widget-active", preview_widget_active),
        }
    }

    pub fn select_multiple(self, select_multiple: bool) -> Self {
        Self {
            builder: self.builder.property("select-multiple", select_multiple),
        }
    }

    pub fn show_hidden(self, show_hidden: bool) -> Self {
        Self {
            builder: self.builder.property("show-hidden", show_hidden),
        }
    }

    pub fn use_preview_label(self, use_preview_label: bool) -> Self {
        Self {
            builder: self
                .builder
                .property("use-preview-label", use_preview_label),
        }
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`FileChooserDialog`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> FileChooserDialog {
        self.builder.build()
    }
}
