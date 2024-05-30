// This file was generated by gir (https://github.com/gtk-rs/gir)
// from 
// from ../gir-files-gstreamer
// from ../gir-files-gtk
// DO NOT EDIT

use glib::{prelude::*,translate::*};

glib::wrapper! {
    #[doc(alias = "ClapperThreadedObject")]
    pub struct ThreadedObject(Object<ffi::ClapperThreadedObject, ffi::ClapperThreadedObjectClass>) @extends gst::Object;

    match fn {
        type_ => || ffi::clapper_threaded_object_get_type(),
    }
}

impl ThreadedObject {
        pub const NONE: Option<&'static ThreadedObject> = None;
    
}

mod sealed {
    pub trait Sealed {}
    impl<T: super::IsA<super::ThreadedObject>> Sealed for T {}
}

pub trait ThreadedObjectExt: IsA<ThreadedObject> + sealed::Sealed + 'static {
    #[doc(alias = "clapper_threaded_object_get_context")]
    #[doc(alias = "get_context")]
    fn context(&self) -> Option<glib::MainContext> {
        unsafe {
            from_glib_full(ffi::clapper_threaded_object_get_context(self.as_ref().to_glib_none().0))
        }
    }
}

impl<O: IsA<ThreadedObject>> ThreadedObjectExt for O {}