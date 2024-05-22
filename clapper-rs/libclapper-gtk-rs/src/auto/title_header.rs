// This file was generated by gir (https://github.com/gtk-rs/gir)
// from
// from ../gir-files-gstreamer
// from ../gir-files-gtk
// from ../libclapper-rs
// DO NOT EDIT

use crate::{Container, LeadContainer, VideoActionMask};
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::boxed::Box as Box_;

glib::wrapper! {
    #[doc(alias = "ClapperGtkTitleHeader")]
    pub struct TitleHeader(Object<ffi::ClapperGtkTitleHeader, ffi::ClapperGtkTitleHeaderClass>) @extends LeadContainer, Container, gtk::Widget, @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;

    match fn {
        type_ => || ffi::clapper_gtk_title_header_get_type(),
    }
}

impl TitleHeader {
    #[doc(alias = "clapper_gtk_title_header_new")]
    pub fn new() -> TitleHeader {
        assert_initialized_main_thread!();
        unsafe { gtk::Widget::from_glib_none(ffi::clapper_gtk_title_header_new()).unsafe_cast() }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`TitleHeader`] objects.
    ///
    /// This method returns an instance of [`TitleHeaderBuilder`](crate::builders::TitleHeaderBuilder) which can be used to create [`TitleHeader`] objects.
    pub fn builder() -> TitleHeaderBuilder {
        TitleHeaderBuilder::new()
    }

    #[doc(alias = "clapper_gtk_title_header_get_current_title")]
    #[doc(alias = "get_current_title")]
    pub fn current_title(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::clapper_gtk_title_header_get_current_title(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "clapper_gtk_title_header_get_fallback_to_uri")]
    #[doc(alias = "get_fallback_to_uri")]
    pub fn is_fallback_to_uri(&self) -> bool {
        unsafe {
            from_glib(ffi::clapper_gtk_title_header_get_fallback_to_uri(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "clapper_gtk_title_header_set_fallback_to_uri")]
    pub fn set_fallback_to_uri(&self, enabled: bool) {
        unsafe {
            ffi::clapper_gtk_title_header_set_fallback_to_uri(
                self.to_glib_none().0,
                enabled.into_glib(),
            );
        }
    }

    #[doc(alias = "current-title")]
    pub fn connect_current_title_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_current_title_trampoline<F: Fn(&TitleHeader) + 'static>(
            this: *mut ffi::ClapperGtkTitleHeader,
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
                b"notify::current-title\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(
                    notify_current_title_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "fallback-to-uri")]
    pub fn connect_fallback_to_uri_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_fallback_to_uri_trampoline<F: Fn(&TitleHeader) + 'static>(
            this: *mut ffi::ClapperGtkTitleHeader,
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
                b"notify::fallback-to-uri\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(
                    notify_fallback_to_uri_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl Default for TitleHeader {
    fn default() -> Self {
        Self::new()
    }
}

// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`TitleHeader`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct TitleHeaderBuilder {
    builder: glib::object::ObjectBuilder<'static, TitleHeader>,
}

impl TitleHeaderBuilder {
    fn new() -> Self {
        Self {
            builder: glib::object::Object::builder(),
        }
    }

    pub fn fallback_to_uri(self, fallback_to_uri: bool) -> Self {
        Self {
            builder: self.builder.property("fallback-to-uri", fallback_to_uri),
        }
    }

    pub fn blocked_actions(self, blocked_actions: VideoActionMask) -> Self {
        Self {
            builder: self.builder.property("blocked-actions", blocked_actions),
        }
    }

    pub fn leading(self, leading: bool) -> Self {
        Self {
            builder: self.builder.property("leading", leading),
        }
    }

    pub fn adaptive_height(self, adaptive_height: i32) -> Self {
        Self {
            builder: self.builder.property("adaptive-height", adaptive_height),
        }
    }

    pub fn adaptive_width(self, adaptive_width: i32) -> Self {
        Self {
            builder: self.builder.property("adaptive-width", adaptive_width),
        }
    }

    pub fn height_target(self, height_target: i32) -> Self {
        Self {
            builder: self.builder.property("height-target", height_target),
        }
    }

    pub fn width_target(self, width_target: i32) -> Self {
        Self {
            builder: self.builder.property("width-target", width_target),
        }
    }

    pub fn can_focus(self, can_focus: bool) -> Self {
        Self {
            builder: self.builder.property("can-focus", can_focus),
        }
    }

    pub fn can_target(self, can_target: bool) -> Self {
        Self {
            builder: self.builder.property("can-target", can_target),
        }
    }

    pub fn css_classes(self, css_classes: impl Into<glib::StrV>) -> Self {
        Self {
            builder: self.builder.property("css-classes", css_classes.into()),
        }
    }

    pub fn css_name(self, css_name: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("css-name", css_name.into()),
        }
    }

    pub fn cursor(self, cursor: &gdk::Cursor) -> Self {
        Self {
            builder: self.builder.property("cursor", cursor.clone()),
        }
    }

    pub fn focus_on_click(self, focus_on_click: bool) -> Self {
        Self {
            builder: self.builder.property("focus-on-click", focus_on_click),
        }
    }

    pub fn focusable(self, focusable: bool) -> Self {
        Self {
            builder: self.builder.property("focusable", focusable),
        }
    }

    pub fn halign(self, halign: gtk::Align) -> Self {
        Self {
            builder: self.builder.property("halign", halign),
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

    pub fn layout_manager(self, layout_manager: &impl IsA<gtk::LayoutManager>) -> Self {
        Self {
            builder: self
                .builder
                .property("layout-manager", layout_manager.clone().upcast()),
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

    pub fn opacity(self, opacity: f64) -> Self {
        Self {
            builder: self.builder.property("opacity", opacity),
        }
    }

    pub fn overflow(self, overflow: gtk::Overflow) -> Self {
        Self {
            builder: self.builder.property("overflow", overflow),
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

    pub fn valign(self, valign: gtk::Align) -> Self {
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

    pub fn accessible_role(self, accessible_role: gtk::AccessibleRole) -> Self {
        Self {
            builder: self.builder.property("accessible-role", accessible_role),
        }
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`TitleHeader`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> TitleHeader {
        self.builder.build()
    }
}
