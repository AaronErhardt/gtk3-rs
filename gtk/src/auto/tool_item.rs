// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::Align;
use crate::Bin;
use crate::Buildable;
use crate::Container;
use crate::IconSize;
use crate::Orientation;
use crate::ReliefStyle;
use crate::ResizeMode;
use crate::SizeGroup;
use crate::ToolbarStyle;
use crate::Widget;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::StaticType;
use glib::ToValue;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib::wrapper! {
    pub struct ToolItem(Object<ffi::GtkToolItem, ffi::GtkToolItemClass>) @extends Bin, Container, Widget, @implements Buildable;

    match fn {
        type_ => || ffi::gtk_tool_item_get_type(),
    }
}

impl ToolItem {
    #[doc(alias = "gtk_tool_item_new")]
    pub fn new() -> ToolItem {
        assert_initialized_main_thread!();
        unsafe { from_glib_none(ffi::gtk_tool_item_new()) }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-style object to construct a [`ToolItem`]
    /// This method returns an instance of [`ToolItemBuilder`] which can be used to create a [`ToolItem`].
    pub fn builder() -> ToolItemBuilder {
        ToolItemBuilder::default()
    }
}

impl Default for ToolItem {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Default)]
// rustdoc-stripper-ignore-next
/// A builder for generating a [`ToolItem`].
pub struct ToolItemBuilder {
    is_important: Option<bool>,
    visible_horizontal: Option<bool>,
    visible_vertical: Option<bool>,
    border_width: Option<u32>,
    child: Option<Widget>,
    resize_mode: Option<ResizeMode>,
    app_paintable: Option<bool>,
    can_default: Option<bool>,
    can_focus: Option<bool>,
    events: Option<gdk::EventMask>,
    expand: Option<bool>,
    #[cfg(any(feature = "v3_20", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v3_20")))]
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

impl ToolItemBuilder {
    // rustdoc-stripper-ignore-next
    /// Create a new [`ToolItemBuilder`].
    pub fn new() -> Self {
        Self::default()
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`ToolItem`].
    pub fn build(self) -> ToolItem {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref is_important) = self.is_important {
            properties.push(("is-important", is_important));
        }
        if let Some(ref visible_horizontal) = self.visible_horizontal {
            properties.push(("visible-horizontal", visible_horizontal));
        }
        if let Some(ref visible_vertical) = self.visible_vertical {
            properties.push(("visible-vertical", visible_vertical));
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
        if let Some(ref focus_on_click) = self.focus_on_click {
            properties.push(("focus-on-click", focus_on_click));
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
        glib::Object::new::<ToolItem>(&properties)
            .expect("Failed to create an instance of ToolItem")
    }

    pub fn is_important(mut self, is_important: bool) -> Self {
        self.is_important = Some(is_important);
        self
    }

    pub fn visible_horizontal(mut self, visible_horizontal: bool) -> Self {
        self.visible_horizontal = Some(visible_horizontal);
        self
    }

    pub fn visible_vertical(mut self, visible_vertical: bool) -> Self {
        self.visible_vertical = Some(visible_vertical);
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
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v3_20")))]
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

pub const NONE_TOOL_ITEM: Option<&ToolItem> = None;

pub trait ToolItemExt: 'static {
    #[doc(alias = "gtk_tool_item_get_ellipsize_mode")]
    #[doc(alias = "get_ellipsize_mode")]
    fn ellipsize_mode(&self) -> pango::EllipsizeMode;

    #[doc(alias = "gtk_tool_item_get_expand")]
    #[doc(alias = "get_expand")]
    fn expands(&self) -> bool;

    #[doc(alias = "gtk_tool_item_get_homogeneous")]
    #[doc(alias = "get_homogeneous")]
    fn is_homogeneous(&self) -> bool;

    #[doc(alias = "gtk_tool_item_get_icon_size")]
    #[doc(alias = "get_icon_size")]
    fn icon_size(&self) -> IconSize;

    #[doc(alias = "gtk_tool_item_get_is_important")]
    #[doc(alias = "get_is_important")]
    fn is_important(&self) -> bool;

    #[doc(alias = "gtk_tool_item_get_orientation")]
    #[doc(alias = "get_orientation")]
    fn orientation(&self) -> Orientation;

    #[doc(alias = "gtk_tool_item_get_proxy_menu_item")]
    #[doc(alias = "get_proxy_menu_item")]
    fn proxy_menu_item(&self, menu_item_id: &str) -> Option<Widget>;

    #[doc(alias = "gtk_tool_item_get_relief_style")]
    #[doc(alias = "get_relief_style")]
    fn relief_style(&self) -> ReliefStyle;

    #[doc(alias = "gtk_tool_item_get_text_alignment")]
    #[doc(alias = "get_text_alignment")]
    fn text_alignment(&self) -> f32;

    #[doc(alias = "gtk_tool_item_get_text_orientation")]
    #[doc(alias = "get_text_orientation")]
    fn text_orientation(&self) -> Orientation;

    #[doc(alias = "gtk_tool_item_get_text_size_group")]
    #[doc(alias = "get_text_size_group")]
    fn text_size_group(&self) -> Option<SizeGroup>;

    #[doc(alias = "gtk_tool_item_get_toolbar_style")]
    #[doc(alias = "get_toolbar_style")]
    fn toolbar_style(&self) -> ToolbarStyle;

    #[doc(alias = "gtk_tool_item_get_use_drag_window")]
    #[doc(alias = "get_use_drag_window")]
    fn uses_drag_window(&self) -> bool;

    #[doc(alias = "gtk_tool_item_get_visible_horizontal")]
    #[doc(alias = "get_visible_horizontal")]
    fn is_visible_horizontal(&self) -> bool;

    #[doc(alias = "gtk_tool_item_get_visible_vertical")]
    #[doc(alias = "get_visible_vertical")]
    fn is_visible_vertical(&self) -> bool;

    #[doc(alias = "gtk_tool_item_rebuild_menu")]
    fn rebuild_menu(&self);

    #[doc(alias = "gtk_tool_item_retrieve_proxy_menu_item")]
    fn retrieve_proxy_menu_item(&self) -> Option<Widget>;

    #[doc(alias = "gtk_tool_item_set_expand")]
    fn set_expand(&self, expand: bool);

    #[doc(alias = "gtk_tool_item_set_homogeneous")]
    fn set_homogeneous(&self, homogeneous: bool);

    #[doc(alias = "gtk_tool_item_set_is_important")]
    fn set_is_important(&self, is_important: bool);

    #[doc(alias = "gtk_tool_item_set_proxy_menu_item")]
    fn set_proxy_menu_item<P: IsA<Widget>>(&self, menu_item_id: &str, menu_item: Option<&P>);

    #[doc(alias = "gtk_tool_item_set_use_drag_window")]
    fn set_use_drag_window(&self, use_drag_window: bool);

    #[doc(alias = "gtk_tool_item_set_visible_horizontal")]
    fn set_visible_horizontal(&self, visible_horizontal: bool);

    #[doc(alias = "gtk_tool_item_set_visible_vertical")]
    fn set_visible_vertical(&self, visible_vertical: bool);

    #[doc(alias = "gtk_tool_item_toolbar_reconfigured")]
    fn toolbar_reconfigured(&self);

    #[doc(alias = "create-menu-proxy")]
    fn connect_create_menu_proxy<F: Fn(&Self) -> glib::signal::Inhibit + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    #[doc(alias = "toolbar-reconfigured")]
    fn connect_toolbar_reconfigured<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "is-important")]
    fn connect_is_important_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "visible-horizontal")]
    fn connect_visible_horizontal_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "visible-vertical")]
    fn connect_visible_vertical_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<ToolItem>> ToolItemExt for O {
    fn ellipsize_mode(&self) -> pango::EllipsizeMode {
        unsafe {
            from_glib(ffi::gtk_tool_item_get_ellipsize_mode(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn expands(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_tool_item_get_expand(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn is_homogeneous(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_tool_item_get_homogeneous(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn icon_size(&self) -> IconSize {
        unsafe {
            from_glib(ffi::gtk_tool_item_get_icon_size(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn is_important(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_tool_item_get_is_important(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn orientation(&self) -> Orientation {
        unsafe {
            from_glib(ffi::gtk_tool_item_get_orientation(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn proxy_menu_item(&self, menu_item_id: &str) -> Option<Widget> {
        unsafe {
            from_glib_none(ffi::gtk_tool_item_get_proxy_menu_item(
                self.as_ref().to_glib_none().0,
                menu_item_id.to_glib_none().0,
            ))
        }
    }

    fn relief_style(&self) -> ReliefStyle {
        unsafe {
            from_glib(ffi::gtk_tool_item_get_relief_style(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn text_alignment(&self) -> f32 {
        unsafe { ffi::gtk_tool_item_get_text_alignment(self.as_ref().to_glib_none().0) }
    }

    fn text_orientation(&self) -> Orientation {
        unsafe {
            from_glib(ffi::gtk_tool_item_get_text_orientation(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn text_size_group(&self) -> Option<SizeGroup> {
        unsafe {
            from_glib_none(ffi::gtk_tool_item_get_text_size_group(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn toolbar_style(&self) -> ToolbarStyle {
        unsafe {
            from_glib(ffi::gtk_tool_item_get_toolbar_style(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn uses_drag_window(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_tool_item_get_use_drag_window(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn is_visible_horizontal(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_tool_item_get_visible_horizontal(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn is_visible_vertical(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_tool_item_get_visible_vertical(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn rebuild_menu(&self) {
        unsafe {
            ffi::gtk_tool_item_rebuild_menu(self.as_ref().to_glib_none().0);
        }
    }

    fn retrieve_proxy_menu_item(&self) -> Option<Widget> {
        unsafe {
            from_glib_none(ffi::gtk_tool_item_retrieve_proxy_menu_item(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn set_expand(&self, expand: bool) {
        unsafe {
            ffi::gtk_tool_item_set_expand(self.as_ref().to_glib_none().0, expand.into_glib());
        }
    }

    fn set_homogeneous(&self, homogeneous: bool) {
        unsafe {
            ffi::gtk_tool_item_set_homogeneous(
                self.as_ref().to_glib_none().0,
                homogeneous.into_glib(),
            );
        }
    }

    fn set_is_important(&self, is_important: bool) {
        unsafe {
            ffi::gtk_tool_item_set_is_important(
                self.as_ref().to_glib_none().0,
                is_important.into_glib(),
            );
        }
    }

    fn set_proxy_menu_item<P: IsA<Widget>>(&self, menu_item_id: &str, menu_item: Option<&P>) {
        unsafe {
            ffi::gtk_tool_item_set_proxy_menu_item(
                self.as_ref().to_glib_none().0,
                menu_item_id.to_glib_none().0,
                menu_item.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    fn set_use_drag_window(&self, use_drag_window: bool) {
        unsafe {
            ffi::gtk_tool_item_set_use_drag_window(
                self.as_ref().to_glib_none().0,
                use_drag_window.into_glib(),
            );
        }
    }

    fn set_visible_horizontal(&self, visible_horizontal: bool) {
        unsafe {
            ffi::gtk_tool_item_set_visible_horizontal(
                self.as_ref().to_glib_none().0,
                visible_horizontal.into_glib(),
            );
        }
    }

    fn set_visible_vertical(&self, visible_vertical: bool) {
        unsafe {
            ffi::gtk_tool_item_set_visible_vertical(
                self.as_ref().to_glib_none().0,
                visible_vertical.into_glib(),
            );
        }
    }

    fn toolbar_reconfigured(&self) {
        unsafe {
            ffi::gtk_tool_item_toolbar_reconfigured(self.as_ref().to_glib_none().0);
        }
    }

    #[doc(alias = "create-menu-proxy")]
    fn connect_create_menu_proxy<F: Fn(&Self) -> glib::signal::Inhibit + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn create_menu_proxy_trampoline<
            P,
            F: Fn(&P) -> glib::signal::Inhibit + 'static,
        >(
            this: *mut ffi::GtkToolItem,
            f: glib::ffi::gpointer,
        ) -> glib::ffi::gboolean
        where
            P: IsA<ToolItem>,
        {
            let f: &F = &*(f as *const F);
            f(&ToolItem::from_glib_borrow(this).unsafe_cast_ref()).into_glib()
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"create-menu-proxy\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    create_menu_proxy_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "toolbar-reconfigured")]
    fn connect_toolbar_reconfigured<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn toolbar_reconfigured_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkToolItem,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<ToolItem>,
        {
            let f: &F = &*(f as *const F);
            f(&ToolItem::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"toolbar-reconfigured\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    toolbar_reconfigured_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "is-important")]
    fn connect_is_important_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_is_important_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkToolItem,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<ToolItem>,
        {
            let f: &F = &*(f as *const F);
            f(&ToolItem::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::is-important\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_is_important_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "visible-horizontal")]
    fn connect_visible_horizontal_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_visible_horizontal_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkToolItem,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<ToolItem>,
        {
            let f: &F = &*(f as *const F);
            f(&ToolItem::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::visible-horizontal\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_visible_horizontal_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "visible-vertical")]
    fn connect_visible_vertical_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_visible_vertical_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkToolItem,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<ToolItem>,
        {
            let f: &F = &*(f as *const F);
            f(&ToolItem::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::visible-vertical\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_visible_vertical_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for ToolItem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("ToolItem")
    }
}
