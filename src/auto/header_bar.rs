// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use gdk;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::GString;
use glib::StaticType;
use glib::ToValue;
use glib::Value;
use glib_sys;
use gobject_sys;
use gtk_sys;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;
use Align;
use Buildable;
use Container;
use PackType;
use ResizeMode;
use Widget;

glib_wrapper! {
    pub struct HeaderBar(Object<gtk_sys::GtkHeaderBar, gtk_sys::GtkHeaderBarClass, HeaderBarClass>) @extends Container, Widget, @implements Buildable;

    match fn {
        get_type => || gtk_sys::gtk_header_bar_get_type(),
    }
}

impl HeaderBar {
    pub fn new() -> HeaderBar {
        assert_initialized_main_thread!();
        unsafe { Widget::from_glib_none(gtk_sys::gtk_header_bar_new()).unsafe_cast() }
    }
}

impl Default for HeaderBar {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Default)]
pub struct HeaderBarBuilder {
    custom_title: Option<Widget>,
    decoration_layout: Option<String>,
    decoration_layout_set: Option<bool>,
    has_subtitle: Option<bool>,
    show_close_button: Option<bool>,
    spacing: Option<i32>,
    subtitle: Option<String>,
    title: Option<String>,
    border_width: Option<u32>,
    child: Option<Widget>,
    resize_mode: Option<ResizeMode>,
    app_paintable: Option<bool>,
    can_default: Option<bool>,
    can_focus: Option<bool>,
    events: Option<gdk::EventMask>,
    expand: Option<bool>,
    #[cfg(any(feature = "v3_20", feature = "dox"))]
    focus_on_click: Option<bool>,
    halign: Option<Align>,
    has_default: Option<bool>,
    has_focus: Option<bool>,
    has_tooltip: Option<bool>,
    height_request: Option<i32>,
    hexpand: Option<bool>,
    hexpand_set: Option<bool>,
    is_focus: Option<bool>,
    margin: Option<i32>,
    margin_bottom: Option<i32>,
    margin_end: Option<i32>,
    margin_start: Option<i32>,
    margin_top: Option<i32>,
    name: Option<String>,
    no_show_all: Option<bool>,
    opacity: Option<f64>,
    parent: Option<Container>,
    receives_default: Option<bool>,
    sensitive: Option<bool>,
    tooltip_markup: Option<String>,
    tooltip_text: Option<String>,
    valign: Option<Align>,
    vexpand: Option<bool>,
    vexpand_set: Option<bool>,
    visible: Option<bool>,
    width_request: Option<i32>,
}

impl HeaderBarBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(self) -> HeaderBar {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref custom_title) = self.custom_title {
            properties.push(("custom-title", custom_title));
        }
        if let Some(ref decoration_layout) = self.decoration_layout {
            properties.push(("decoration-layout", decoration_layout));
        }
        if let Some(ref decoration_layout_set) = self.decoration_layout_set {
            properties.push(("decoration-layout-set", decoration_layout_set));
        }
        if let Some(ref has_subtitle) = self.has_subtitle {
            properties.push(("has-subtitle", has_subtitle));
        }
        if let Some(ref show_close_button) = self.show_close_button {
            properties.push(("show-close-button", show_close_button));
        }
        if let Some(ref spacing) = self.spacing {
            properties.push(("spacing", spacing));
        }
        if let Some(ref subtitle) = self.subtitle {
            properties.push(("subtitle", subtitle));
        }
        if let Some(ref title) = self.title {
            properties.push(("title", title));
        }
        if let Some(ref border_width) = self.border_width {
            properties.push(("border-width", border_width));
        }
        if let Some(ref child) = self.child {
            properties.push(("child", child));
        }
        if let Some(ref resize_mode) = self.resize_mode {
            properties.push(("resize-mode", resize_mode));
        }
        if let Some(ref app_paintable) = self.app_paintable {
            properties.push(("app-paintable", app_paintable));
        }
        if let Some(ref can_default) = self.can_default {
            properties.push(("can-default", can_default));
        }
        if let Some(ref can_focus) = self.can_focus {
            properties.push(("can-focus", can_focus));
        }
        if let Some(ref events) = self.events {
            properties.push(("events", events));
        }
        if let Some(ref expand) = self.expand {
            properties.push(("expand", expand));
        }
        #[cfg(any(feature = "v3_20", feature = "dox"))]
        {
            if let Some(ref focus_on_click) = self.focus_on_click {
                properties.push(("focus-on-click", focus_on_click));
            }
        }
        if let Some(ref halign) = self.halign {
            properties.push(("halign", halign));
        }
        if let Some(ref has_default) = self.has_default {
            properties.push(("has-default", has_default));
        }
        if let Some(ref has_focus) = self.has_focus {
            properties.push(("has-focus", has_focus));
        }
        if let Some(ref has_tooltip) = self.has_tooltip {
            properties.push(("has-tooltip", has_tooltip));
        }
        if let Some(ref height_request) = self.height_request {
            properties.push(("height-request", height_request));
        }
        if let Some(ref hexpand) = self.hexpand {
            properties.push(("hexpand", hexpand));
        }
        if let Some(ref hexpand_set) = self.hexpand_set {
            properties.push(("hexpand-set", hexpand_set));
        }
        if let Some(ref is_focus) = self.is_focus {
            properties.push(("is-focus", is_focus));
        }
        if let Some(ref margin) = self.margin {
            properties.push(("margin", margin));
        }
        if let Some(ref margin_bottom) = self.margin_bottom {
            properties.push(("margin-bottom", margin_bottom));
        }
        if let Some(ref margin_end) = self.margin_end {
            properties.push(("margin-end", margin_end));
        }
        if let Some(ref margin_start) = self.margin_start {
            properties.push(("margin-start", margin_start));
        }
        if let Some(ref margin_top) = self.margin_top {
            properties.push(("margin-top", margin_top));
        }
        if let Some(ref name) = self.name {
            properties.push(("name", name));
        }
        if let Some(ref no_show_all) = self.no_show_all {
            properties.push(("no-show-all", no_show_all));
        }
        if let Some(ref opacity) = self.opacity {
            properties.push(("opacity", opacity));
        }
        if let Some(ref parent) = self.parent {
            properties.push(("parent", parent));
        }
        if let Some(ref receives_default) = self.receives_default {
            properties.push(("receives-default", receives_default));
        }
        if let Some(ref sensitive) = self.sensitive {
            properties.push(("sensitive", sensitive));
        }
        if let Some(ref tooltip_markup) = self.tooltip_markup {
            properties.push(("tooltip-markup", tooltip_markup));
        }
        if let Some(ref tooltip_text) = self.tooltip_text {
            properties.push(("tooltip-text", tooltip_text));
        }
        if let Some(ref valign) = self.valign {
            properties.push(("valign", valign));
        }
        if let Some(ref vexpand) = self.vexpand {
            properties.push(("vexpand", vexpand));
        }
        if let Some(ref vexpand_set) = self.vexpand_set {
            properties.push(("vexpand-set", vexpand_set));
        }
        if let Some(ref visible) = self.visible {
            properties.push(("visible", visible));
        }
        if let Some(ref width_request) = self.width_request {
            properties.push(("width-request", width_request));
        }
        glib::Object::new(HeaderBar::static_type(), &properties)
            .expect("object new")
            .downcast()
            .expect("downcast")
    }

    pub fn custom_title<P: IsA<Widget>>(mut self, custom_title: &P) -> Self {
        self.custom_title = Some(custom_title.clone().upcast());
        self
    }

    pub fn decoration_layout(mut self, decoration_layout: &str) -> Self {
        self.decoration_layout = Some(decoration_layout.to_string());
        self
    }

    pub fn decoration_layout_set(mut self, decoration_layout_set: bool) -> Self {
        self.decoration_layout_set = Some(decoration_layout_set);
        self
    }

    pub fn has_subtitle(mut self, has_subtitle: bool) -> Self {
        self.has_subtitle = Some(has_subtitle);
        self
    }

    pub fn show_close_button(mut self, show_close_button: bool) -> Self {
        self.show_close_button = Some(show_close_button);
        self
    }

    pub fn spacing(mut self, spacing: i32) -> Self {
        self.spacing = Some(spacing);
        self
    }

    pub fn subtitle(mut self, subtitle: &str) -> Self {
        self.subtitle = Some(subtitle.to_string());
        self
    }

    pub fn title(mut self, title: &str) -> Self {
        self.title = Some(title.to_string());
        self
    }

    pub fn border_width(mut self, border_width: u32) -> Self {
        self.border_width = Some(border_width);
        self
    }

    pub fn child<P: IsA<Widget>>(mut self, child: &P) -> Self {
        self.child = Some(child.clone().upcast());
        self
    }

    pub fn resize_mode(mut self, resize_mode: ResizeMode) -> Self {
        self.resize_mode = Some(resize_mode);
        self
    }

    pub fn app_paintable(mut self, app_paintable: bool) -> Self {
        self.app_paintable = Some(app_paintable);
        self
    }

    pub fn can_default(mut self, can_default: bool) -> Self {
        self.can_default = Some(can_default);
        self
    }

    pub fn can_focus(mut self, can_focus: bool) -> Self {
        self.can_focus = Some(can_focus);
        self
    }

    pub fn events(mut self, events: gdk::EventMask) -> Self {
        self.events = Some(events);
        self
    }

    pub fn expand(mut self, expand: bool) -> Self {
        self.expand = Some(expand);
        self
    }

    #[cfg(any(feature = "v3_20", feature = "dox"))]
    pub fn focus_on_click(mut self, focus_on_click: bool) -> Self {
        self.focus_on_click = Some(focus_on_click);
        self
    }

    pub fn halign(mut self, halign: Align) -> Self {
        self.halign = Some(halign);
        self
    }

    pub fn has_default(mut self, has_default: bool) -> Self {
        self.has_default = Some(has_default);
        self
    }

    pub fn has_focus(mut self, has_focus: bool) -> Self {
        self.has_focus = Some(has_focus);
        self
    }

    pub fn has_tooltip(mut self, has_tooltip: bool) -> Self {
        self.has_tooltip = Some(has_tooltip);
        self
    }

    pub fn height_request(mut self, height_request: i32) -> Self {
        self.height_request = Some(height_request);
        self
    }

    pub fn hexpand(mut self, hexpand: bool) -> Self {
        self.hexpand = Some(hexpand);
        self
    }

    pub fn hexpand_set(mut self, hexpand_set: bool) -> Self {
        self.hexpand_set = Some(hexpand_set);
        self
    }

    pub fn is_focus(mut self, is_focus: bool) -> Self {
        self.is_focus = Some(is_focus);
        self
    }

    pub fn margin(mut self, margin: i32) -> Self {
        self.margin = Some(margin);
        self
    }

    pub fn margin_bottom(mut self, margin_bottom: i32) -> Self {
        self.margin_bottom = Some(margin_bottom);
        self
    }

    pub fn margin_end(mut self, margin_end: i32) -> Self {
        self.margin_end = Some(margin_end);
        self
    }

    pub fn margin_start(mut self, margin_start: i32) -> Self {
        self.margin_start = Some(margin_start);
        self
    }

    pub fn margin_top(mut self, margin_top: i32) -> Self {
        self.margin_top = Some(margin_top);
        self
    }

    pub fn name(mut self, name: &str) -> Self {
        self.name = Some(name.to_string());
        self
    }

    pub fn no_show_all(mut self, no_show_all: bool) -> Self {
        self.no_show_all = Some(no_show_all);
        self
    }

    pub fn opacity(mut self, opacity: f64) -> Self {
        self.opacity = Some(opacity);
        self
    }

    pub fn parent<P: IsA<Container>>(mut self, parent: &P) -> Self {
        self.parent = Some(parent.clone().upcast());
        self
    }

    pub fn receives_default(mut self, receives_default: bool) -> Self {
        self.receives_default = Some(receives_default);
        self
    }

    pub fn sensitive(mut self, sensitive: bool) -> Self {
        self.sensitive = Some(sensitive);
        self
    }

    pub fn tooltip_markup(mut self, tooltip_markup: &str) -> Self {
        self.tooltip_markup = Some(tooltip_markup.to_string());
        self
    }

    pub fn tooltip_text(mut self, tooltip_text: &str) -> Self {
        self.tooltip_text = Some(tooltip_text.to_string());
        self
    }

    pub fn valign(mut self, valign: Align) -> Self {
        self.valign = Some(valign);
        self
    }

    pub fn vexpand(mut self, vexpand: bool) -> Self {
        self.vexpand = Some(vexpand);
        self
    }

    pub fn vexpand_set(mut self, vexpand_set: bool) -> Self {
        self.vexpand_set = Some(vexpand_set);
        self
    }

    pub fn visible(mut self, visible: bool) -> Self {
        self.visible = Some(visible);
        self
    }

    pub fn width_request(mut self, width_request: i32) -> Self {
        self.width_request = Some(width_request);
        self
    }
}

pub const NONE_HEADER_BAR: Option<&HeaderBar> = None;

pub trait HeaderBarExt: 'static {
    fn get_custom_title(&self) -> Option<Widget>;

    fn get_decoration_layout(&self) -> Option<GString>;

    fn get_has_subtitle(&self) -> bool;

    fn get_show_close_button(&self) -> bool;

    fn get_subtitle(&self) -> Option<GString>;

    fn get_title(&self) -> Option<GString>;

    fn pack_end<P: IsA<Widget>>(&self, child: &P);

    fn pack_start<P: IsA<Widget>>(&self, child: &P);

    fn set_custom_title<P: IsA<Widget>>(&self, title_widget: Option<&P>);

    fn set_decoration_layout(&self, layout: Option<&str>);

    fn set_has_subtitle(&self, setting: bool);

    fn set_show_close_button(&self, setting: bool);

    fn set_subtitle(&self, subtitle: Option<&str>);

    fn set_title(&self, title: Option<&str>);

    fn get_property_decoration_layout_set(&self) -> bool;

    fn set_property_decoration_layout_set(&self, decoration_layout_set: bool);

    fn get_property_spacing(&self) -> i32;

    fn set_property_spacing(&self, spacing: i32);

    fn get_child_pack_type<T: IsA<Widget>>(&self, item: &T) -> PackType;

    fn set_child_pack_type<T: IsA<Widget>>(&self, item: &T, pack_type: PackType);

    fn get_child_position<T: IsA<Widget>>(&self, item: &T) -> i32;

    fn set_child_position<T: IsA<Widget>>(&self, item: &T, position: i32);

    fn connect_property_custom_title_notify<F: Fn(&Self) + 'static>(&self, f: F)
        -> SignalHandlerId;

    fn connect_property_decoration_layout_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_decoration_layout_set_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_has_subtitle_notify<F: Fn(&Self) + 'static>(&self, f: F)
        -> SignalHandlerId;

    fn connect_property_show_close_button_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_spacing_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_subtitle_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_title_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<HeaderBar>> HeaderBarExt for O {
    fn get_custom_title(&self) -> Option<Widget> {
        unsafe {
            from_glib_none(gtk_sys::gtk_header_bar_get_custom_title(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_decoration_layout(&self) -> Option<GString> {
        unsafe {
            from_glib_none(gtk_sys::gtk_header_bar_get_decoration_layout(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_has_subtitle(&self) -> bool {
        unsafe {
            from_glib(gtk_sys::gtk_header_bar_get_has_subtitle(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_show_close_button(&self) -> bool {
        unsafe {
            from_glib(gtk_sys::gtk_header_bar_get_show_close_button(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_subtitle(&self) -> Option<GString> {
        unsafe {
            from_glib_none(gtk_sys::gtk_header_bar_get_subtitle(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_title(&self) -> Option<GString> {
        unsafe {
            from_glib_none(gtk_sys::gtk_header_bar_get_title(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn pack_end<P: IsA<Widget>>(&self, child: &P) {
        unsafe {
            gtk_sys::gtk_header_bar_pack_end(
                self.as_ref().to_glib_none().0,
                child.as_ref().to_glib_none().0,
            );
        }
    }

    fn pack_start<P: IsA<Widget>>(&self, child: &P) {
        unsafe {
            gtk_sys::gtk_header_bar_pack_start(
                self.as_ref().to_glib_none().0,
                child.as_ref().to_glib_none().0,
            );
        }
    }

    fn set_custom_title<P: IsA<Widget>>(&self, title_widget: Option<&P>) {
        unsafe {
            gtk_sys::gtk_header_bar_set_custom_title(
                self.as_ref().to_glib_none().0,
                title_widget.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    fn set_decoration_layout(&self, layout: Option<&str>) {
        unsafe {
            gtk_sys::gtk_header_bar_set_decoration_layout(
                self.as_ref().to_glib_none().0,
                layout.to_glib_none().0,
            );
        }
    }

    fn set_has_subtitle(&self, setting: bool) {
        unsafe {
            gtk_sys::gtk_header_bar_set_has_subtitle(
                self.as_ref().to_glib_none().0,
                setting.to_glib(),
            );
        }
    }

    fn set_show_close_button(&self, setting: bool) {
        unsafe {
            gtk_sys::gtk_header_bar_set_show_close_button(
                self.as_ref().to_glib_none().0,
                setting.to_glib(),
            );
        }
    }

    fn set_subtitle(&self, subtitle: Option<&str>) {
        unsafe {
            gtk_sys::gtk_header_bar_set_subtitle(
                self.as_ref().to_glib_none().0,
                subtitle.to_glib_none().0,
            );
        }
    }

    fn set_title(&self, title: Option<&str>) {
        unsafe {
            gtk_sys::gtk_header_bar_set_title(
                self.as_ref().to_glib_none().0,
                title.to_glib_none().0,
            );
        }
    }

    fn get_property_decoration_layout_set(&self) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"decoration-layout-set\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `decoration-layout-set` getter")
                .unwrap()
        }
    }

    fn set_property_decoration_layout_set(&self, decoration_layout_set: bool) {
        unsafe {
            gobject_sys::g_object_set_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"decoration-layout-set\0".as_ptr() as *const _,
                Value::from(&decoration_layout_set).to_glib_none().0,
            );
        }
    }

    fn get_property_spacing(&self) -> i32 {
        unsafe {
            let mut value = Value::from_type(<i32 as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"spacing\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `spacing` getter")
                .unwrap()
        }
    }

    fn set_property_spacing(&self, spacing: i32) {
        unsafe {
            gobject_sys::g_object_set_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"spacing\0".as_ptr() as *const _,
                Value::from(&spacing).to_glib_none().0,
            );
        }
    }

    fn get_child_pack_type<T: IsA<Widget>>(&self, item: &T) -> PackType {
        unsafe {
            let mut value = Value::from_type(<PackType as StaticType>::static_type());
            gtk_sys::gtk_container_child_get_property(
                self.to_glib_none().0 as *mut gtk_sys::GtkContainer,
                item.to_glib_none().0 as *mut _,
                b"pack-type\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `pack-type` getter")
                .unwrap()
        }
    }

    fn set_child_pack_type<T: IsA<Widget>>(&self, item: &T, pack_type: PackType) {
        unsafe {
            gtk_sys::gtk_container_child_set_property(
                self.to_glib_none().0 as *mut gtk_sys::GtkContainer,
                item.to_glib_none().0 as *mut _,
                b"pack-type\0".as_ptr() as *const _,
                Value::from(&pack_type).to_glib_none().0,
            );
        }
    }

    fn get_child_position<T: IsA<Widget>>(&self, item: &T) -> i32 {
        unsafe {
            let mut value = Value::from_type(<i32 as StaticType>::static_type());
            gtk_sys::gtk_container_child_get_property(
                self.to_glib_none().0 as *mut gtk_sys::GtkContainer,
                item.to_glib_none().0 as *mut _,
                b"position\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `position` getter")
                .unwrap()
        }
    }

    fn set_child_position<T: IsA<Widget>>(&self, item: &T, position: i32) {
        unsafe {
            gtk_sys::gtk_container_child_set_property(
                self.to_glib_none().0 as *mut gtk_sys::GtkContainer,
                item.to_glib_none().0 as *mut _,
                b"position\0".as_ptr() as *const _,
                Value::from(&position).to_glib_none().0,
            );
        }
    }

    fn connect_property_custom_title_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_custom_title_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_sys::GtkHeaderBar,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<HeaderBar>,
        {
            let f: &F = &*(f as *const F);
            f(&HeaderBar::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::custom-title\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_custom_title_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_decoration_layout_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_decoration_layout_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_sys::GtkHeaderBar,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<HeaderBar>,
        {
            let f: &F = &*(f as *const F);
            f(&HeaderBar::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::decoration-layout\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_decoration_layout_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_decoration_layout_set_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_decoration_layout_set_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_sys::GtkHeaderBar,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<HeaderBar>,
        {
            let f: &F = &*(f as *const F);
            f(&HeaderBar::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::decoration-layout-set\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_decoration_layout_set_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_has_subtitle_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_has_subtitle_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_sys::GtkHeaderBar,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<HeaderBar>,
        {
            let f: &F = &*(f as *const F);
            f(&HeaderBar::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::has-subtitle\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_has_subtitle_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_show_close_button_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_show_close_button_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_sys::GtkHeaderBar,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<HeaderBar>,
        {
            let f: &F = &*(f as *const F);
            f(&HeaderBar::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::show-close-button\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_show_close_button_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_spacing_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_spacing_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_sys::GtkHeaderBar,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<HeaderBar>,
        {
            let f: &F = &*(f as *const F);
            f(&HeaderBar::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::spacing\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_spacing_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_subtitle_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_subtitle_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_sys::GtkHeaderBar,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<HeaderBar>,
        {
            let f: &F = &*(f as *const F);
            f(&HeaderBar::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::subtitle\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_subtitle_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_title_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_title_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_sys::GtkHeaderBar,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<HeaderBar>,
        {
            let f: &F = &*(f as *const F);
            f(&HeaderBar::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::title\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_title_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for HeaderBar {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "HeaderBar")
    }
}
