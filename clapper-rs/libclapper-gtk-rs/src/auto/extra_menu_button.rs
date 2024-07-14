// This file was generated by gir (https://github.com/gtk-rs/gir)
// from
// from ..\gir-files-gstreamer
// from ..\gir-files-gtk
// from ..\libclapper-rs
// DO NOT EDIT

use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::boxed::Box as Box_;

glib::wrapper! {
    #[doc(alias = "ClapperGtkExtraMenuButton")]
    pub struct ExtraMenuButton(Object<ffi::ClapperGtkExtraMenuButton, ffi::ClapperGtkExtraMenuButtonClass>) @extends gtk::Widget, @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;

    match fn {
        type_ => || ffi::clapper_gtk_extra_menu_button_get_type(),
    }
}

impl ExtraMenuButton {
    #[doc(alias = "clapper_gtk_extra_menu_button_new")]
    pub fn new() -> ExtraMenuButton {
        assert_initialized_main_thread!();
        unsafe {
            gtk::Widget::from_glib_none(ffi::clapper_gtk_extra_menu_button_new()).unsafe_cast()
        }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`ExtraMenuButton`] objects.
    ///
    /// This method returns an instance of [`ExtraMenuButtonBuilder`](crate::builders::ExtraMenuButtonBuilder) which can be used to create [`ExtraMenuButton`] objects.
    pub fn builder() -> ExtraMenuButtonBuilder {
        ExtraMenuButtonBuilder::new()
    }

    #[doc(alias = "clapper_gtk_extra_menu_button_get_can_open_subtitles")]
    #[doc(alias = "get_can_open_subtitles")]
    #[doc(alias = "can-open-subtitles")]
    pub fn can_open_subtitles(&self) -> bool {
        unsafe {
            from_glib(ffi::clapper_gtk_extra_menu_button_get_can_open_subtitles(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "clapper_gtk_extra_menu_button_get_speed_visible")]
    #[doc(alias = "get_speed_visible")]
    #[doc(alias = "speed-visible")]
    pub fn is_speed_visible(&self) -> bool {
        unsafe {
            from_glib(ffi::clapper_gtk_extra_menu_button_get_speed_visible(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "clapper_gtk_extra_menu_button_get_volume_visible")]
    #[doc(alias = "get_volume_visible")]
    #[doc(alias = "volume-visible")]
    pub fn is_volume_visible(&self) -> bool {
        unsafe {
            from_glib(ffi::clapper_gtk_extra_menu_button_get_volume_visible(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "clapper_gtk_extra_menu_button_set_can_open_subtitles")]
    #[doc(alias = "can-open-subtitles")]
    pub fn set_can_open_subtitles(&self, allowed: bool) {
        unsafe {
            ffi::clapper_gtk_extra_menu_button_set_can_open_subtitles(
                self.to_glib_none().0,
                allowed.into_glib(),
            );
        }
    }

    #[doc(alias = "clapper_gtk_extra_menu_button_set_speed_visible")]
    #[doc(alias = "speed-visible")]
    pub fn set_speed_visible(&self, visible: bool) {
        unsafe {
            ffi::clapper_gtk_extra_menu_button_set_speed_visible(
                self.to_glib_none().0,
                visible.into_glib(),
            );
        }
    }

    #[doc(alias = "clapper_gtk_extra_menu_button_set_volume_visible")]
    #[doc(alias = "volume-visible")]
    pub fn set_volume_visible(&self, visible: bool) {
        unsafe {
            ffi::clapper_gtk_extra_menu_button_set_volume_visible(
                self.to_glib_none().0,
                visible.into_glib(),
            );
        }
    }

    #[doc(alias = "open-subtitles")]
    pub fn connect_open_subtitles<F: Fn(&Self, &clapper::MediaItem) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn open_subtitles_trampoline<
            F: Fn(&ExtraMenuButton, &clapper::MediaItem) + 'static,
        >(
            this: *mut ffi::ClapperGtkExtraMenuButton,
            item: *mut clapper::ffi::ClapperMediaItem,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), &from_glib_borrow(item))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"open-subtitles\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    open_subtitles_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "can-open-subtitles")]
    pub fn connect_can_open_subtitles_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_can_open_subtitles_trampoline<
            F: Fn(&ExtraMenuButton) + 'static,
        >(
            this: *mut ffi::ClapperGtkExtraMenuButton,
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
                b"notify::can-open-subtitles\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_can_open_subtitles_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "speed-visible")]
    pub fn connect_speed_visible_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_speed_visible_trampoline<F: Fn(&ExtraMenuButton) + 'static>(
            this: *mut ffi::ClapperGtkExtraMenuButton,
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
                b"notify::speed-visible\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_speed_visible_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "volume-visible")]
    pub fn connect_volume_visible_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_volume_visible_trampoline<F: Fn(&ExtraMenuButton) + 'static>(
            this: *mut ffi::ClapperGtkExtraMenuButton,
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
                b"notify::volume-visible\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_volume_visible_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl Default for ExtraMenuButton {
    fn default() -> Self {
        Self::new()
    }
}

// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`ExtraMenuButton`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct ExtraMenuButtonBuilder {
    builder: glib::object::ObjectBuilder<'static, ExtraMenuButton>,
}

impl ExtraMenuButtonBuilder {
    fn new() -> Self {
        Self {
            builder: glib::object::Object::builder(),
        }
    }

    pub fn can_open_subtitles(self, can_open_subtitles: bool) -> Self {
        Self {
            builder: self
                .builder
                .property("can-open-subtitles", can_open_subtitles),
        }
    }

    pub fn speed_visible(self, speed_visible: bool) -> Self {
        Self {
            builder: self.builder.property("speed-visible", speed_visible),
        }
    }

    pub fn volume_visible(self, volume_visible: bool) -> Self {
        Self {
            builder: self.builder.property("volume-visible", volume_visible),
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
    /// Build the [`ExtraMenuButton`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> ExtraMenuButton {
        self.builder.build()
    }
}
