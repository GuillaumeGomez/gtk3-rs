// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

glib::wrapper! {
    #[doc(alias = "GdkX11GLContext")]
    pub struct X11GLContext(Object<ffi::GdkX11GLContext, ffi::GdkX11GLContextClass>) @extends gdk::GLContext;

    match fn {
        type_ => || ffi::gdk_x11_gl_context_get_type(),
    }
}

impl X11GLContext {}
