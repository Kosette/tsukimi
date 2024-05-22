// This file was generated by gir (https://github.com/gtk-rs/gir)
// from
// from ../gir-files-gstreamer
// from ../gir-files-gtk
// DO NOT EDIT

use crate::Stream;
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::boxed::Box as Box_;

glib::wrapper! {
    #[doc(alias = "ClapperStreamList")]
    pub struct StreamList(Object<ffi::ClapperStreamList, ffi::ClapperStreamListClass>) @extends gst::Object, @implements gio::ListModel;

    match fn {
        type_ => || ffi::clapper_stream_list_get_type(),
    }
}

impl StreamList {
    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`StreamList`] objects.
    ///
    /// This method returns an instance of [`StreamListBuilder`](crate::builders::StreamListBuilder) which can be used to create [`StreamList`] objects.
    pub fn builder() -> StreamListBuilder {
        StreamListBuilder::new()
    }

    #[doc(alias = "clapper_stream_list_get_current_index")]
    #[doc(alias = "get_current_index")]
    pub fn current_index(&self) -> u32 {
        unsafe { ffi::clapper_stream_list_get_current_index(self.to_glib_none().0) }
    }

    #[doc(alias = "clapper_stream_list_get_current_stream")]
    #[doc(alias = "get_current_stream")]
    pub fn current_stream(&self) -> Option<Stream> {
        unsafe {
            from_glib_full(ffi::clapper_stream_list_get_current_stream(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "clapper_stream_list_get_n_streams")]
    #[doc(alias = "get_n_streams")]
    pub fn n_streams(&self) -> u32 {
        unsafe { ffi::clapper_stream_list_get_n_streams(self.to_glib_none().0) }
    }

    #[doc(alias = "clapper_stream_list_get_stream")]
    #[doc(alias = "get_stream")]
    pub fn stream(&self, index: u32) -> Option<Stream> {
        unsafe {
            from_glib_full(ffi::clapper_stream_list_get_stream(
                self.to_glib_none().0,
                index,
            ))
        }
    }

    #[doc(alias = "clapper_stream_list_select_index")]
    pub fn select_index(&self, index: u32) -> bool {
        unsafe {
            from_glib(ffi::clapper_stream_list_select_index(
                self.to_glib_none().0,
                index,
            ))
        }
    }

    #[doc(alias = "clapper_stream_list_select_stream")]
    pub fn select_stream(&self, stream: &impl IsA<Stream>) -> bool {
        unsafe {
            from_glib(ffi::clapper_stream_list_select_stream(
                self.to_glib_none().0,
                stream.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "current-index")]
    pub fn set_current_index(&self, current_index: u32) {
        ObjectExt::set_property(self, "current-index", current_index)
    }

    #[doc(alias = "current-index")]
    pub fn connect_current_index_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_current_index_trampoline<F: Fn(&StreamList) + 'static>(
            this: *mut ffi::ClapperStreamList,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::current-index\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(
                    notify_current_index_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "current-stream")]
    pub fn connect_current_stream_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_current_stream_trampoline<F: Fn(&StreamList) + 'static>(
            this: *mut ffi::ClapperStreamList,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::current-stream\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(
                    notify_current_stream_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "n-streams")]
    pub fn connect_n_streams_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_n_streams_trampoline<F: Fn(&StreamList) + 'static>(
            this: *mut ffi::ClapperStreamList,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::n-streams\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(
                    notify_n_streams_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`StreamList`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct StreamListBuilder {
    builder: glib::object::ObjectBuilder<'static, StreamList>,
}

impl StreamListBuilder {
    fn new() -> Self {
        Self {
            builder: glib::object::Object::builder(),
        }
    }

    pub fn current_index(self, current_index: u32) -> Self {
        Self {
            builder: self.builder.property("current-index", current_index),
        }
    }

    pub fn name(self, name: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("name", name.into()),
        }
    }

    pub fn parent(self, parent: &impl IsA<gst::Object>) -> Self {
        Self {
            builder: self.builder.property("parent", parent.clone().upcast()),
        }
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`StreamList`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> StreamList {
        self.builder.build()
    }
}
