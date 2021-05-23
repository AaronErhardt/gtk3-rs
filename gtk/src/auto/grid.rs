// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::Align;
use crate::BaselinePosition;
use crate::Buildable;
use crate::Container;
use crate::Orientable;
use crate::Orientation;
use crate::PositionType;
use crate::ResizeMode;
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
    pub struct Grid(Object<ffi::GtkGrid, ffi::GtkGridClass>) @extends Container, Widget, @implements Buildable, Orientable;

    match fn {
        type_ => || ffi::gtk_grid_get_type(),
    }
}

impl Grid {
    #[doc(alias = "gtk_grid_new")]
    pub fn new() -> Grid {
        assert_initialized_main_thread!();
        unsafe { Widget::from_glib_none(ffi::gtk_grid_new()).unsafe_cast() }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-style object to construct a [`Grid`]
    /// This method returns an instance of [`GridBuilder`] which can be used to create a [`Grid`].
    pub fn builder() -> GridBuilder {
        GridBuilder::default()
    }
}

impl Default for Grid {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Default)]
// rustdoc-stripper-ignore-next
/// A builder for generating a [`Grid`].
pub struct GridBuilder {
    baseline_row: Option<i32>,
    column_homogeneous: Option<bool>,
    column_spacing: Option<i32>,
    row_homogeneous: Option<bool>,
    row_spacing: Option<i32>,
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
    orientation: Option<Orientation>,
}

impl GridBuilder {
    // rustdoc-stripper-ignore-next
    /// Create a new [`GridBuilder`].
    pub fn new() -> Self {
        Self::default()
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`Grid`].
    pub fn build(self) -> Grid {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref baseline_row) = self.baseline_row {
            properties.push(("baseline-row", baseline_row));
        }
        if let Some(ref column_homogeneous) = self.column_homogeneous {
            properties.push(("column-homogeneous", column_homogeneous));
        }
        if let Some(ref column_spacing) = self.column_spacing {
            properties.push(("column-spacing", column_spacing));
        }
        if let Some(ref row_homogeneous) = self.row_homogeneous {
            properties.push(("row-homogeneous", row_homogeneous));
        }
        if let Some(ref row_spacing) = self.row_spacing {
            properties.push(("row-spacing", row_spacing));
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
        if let Some(ref orientation) = self.orientation {
            properties.push(("orientation", orientation));
        }
        glib::Object::new::<Grid>(&properties).expect("Failed to create an instance of Grid")
    }

    pub fn baseline_row(mut self, baseline_row: i32) -> Self {
        self.baseline_row = Some(baseline_row);
        self
    }

    pub fn column_homogeneous(mut self, column_homogeneous: bool) -> Self {
        self.column_homogeneous = Some(column_homogeneous);
        self
    }

    pub fn column_spacing(mut self, column_spacing: i32) -> Self {
        self.column_spacing = Some(column_spacing);
        self
    }

    pub fn row_homogeneous(mut self, row_homogeneous: bool) -> Self {
        self.row_homogeneous = Some(row_homogeneous);
        self
    }

    pub fn row_spacing(mut self, row_spacing: i32) -> Self {
        self.row_spacing = Some(row_spacing);
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

    pub fn orientation(mut self, orientation: Orientation) -> Self {
        self.orientation = Some(orientation);
        self
    }
}

pub const NONE_GRID: Option<&Grid> = None;

pub trait GridExt: 'static {
    #[doc(alias = "gtk_grid_attach")]
    fn attach<P: IsA<Widget>>(&self, child: &P, left: i32, top: i32, width: i32, height: i32);

    #[doc(alias = "gtk_grid_attach_next_to")]
    fn attach_next_to<P: IsA<Widget>, Q: IsA<Widget>>(
        &self,
        child: &P,
        sibling: Option<&Q>,
        side: PositionType,
        width: i32,
        height: i32,
    );

    #[doc(alias = "gtk_grid_get_baseline_row")]
    #[doc(alias = "get_baseline_row")]
    fn baseline_row(&self) -> i32;

    #[doc(alias = "gtk_grid_get_child_at")]
    #[doc(alias = "get_child_at")]
    fn child_at(&self, left: i32, top: i32) -> Option<Widget>;

    #[doc(alias = "gtk_grid_get_column_homogeneous")]
    #[doc(alias = "get_column_homogeneous")]
    fn is_column_homogeneous(&self) -> bool;

    #[doc(alias = "gtk_grid_get_column_spacing")]
    #[doc(alias = "get_column_spacing")]
    fn column_spacing(&self) -> u32;

    #[doc(alias = "gtk_grid_get_row_baseline_position")]
    #[doc(alias = "get_row_baseline_position")]
    fn row_baseline_position(&self, row: i32) -> BaselinePosition;

    #[doc(alias = "gtk_grid_get_row_homogeneous")]
    #[doc(alias = "get_row_homogeneous")]
    fn is_row_homogeneous(&self) -> bool;

    #[doc(alias = "gtk_grid_get_row_spacing")]
    #[doc(alias = "get_row_spacing")]
    fn row_spacing(&self) -> u32;

    #[doc(alias = "gtk_grid_insert_column")]
    fn insert_column(&self, position: i32);

    #[doc(alias = "gtk_grid_insert_next_to")]
    fn insert_next_to<P: IsA<Widget>>(&self, sibling: &P, side: PositionType);

    #[doc(alias = "gtk_grid_insert_row")]
    fn insert_row(&self, position: i32);

    #[doc(alias = "gtk_grid_remove_column")]
    fn remove_column(&self, position: i32);

    #[doc(alias = "gtk_grid_remove_row")]
    fn remove_row(&self, position: i32);

    #[doc(alias = "gtk_grid_set_baseline_row")]
    fn set_baseline_row(&self, row: i32);

    #[doc(alias = "gtk_grid_set_column_homogeneous")]
    fn set_column_homogeneous(&self, homogeneous: bool);

    #[doc(alias = "gtk_grid_set_column_spacing")]
    fn set_column_spacing(&self, spacing: u32);

    #[doc(alias = "gtk_grid_set_row_baseline_position")]
    fn set_row_baseline_position(&self, row: i32, pos: BaselinePosition);

    #[doc(alias = "gtk_grid_set_row_homogeneous")]
    fn set_row_homogeneous(&self, homogeneous: bool);

    #[doc(alias = "gtk_grid_set_row_spacing")]
    fn set_row_spacing(&self, spacing: u32);

    fn cell_height<T: IsA<Widget>>(&self, item: &T) -> i32;

    fn set_cell_height<T: IsA<Widget>>(&self, item: &T, height: i32);

    fn cell_width<T: IsA<Widget>>(&self, item: &T) -> i32;

    fn set_cell_width<T: IsA<Widget>>(&self, item: &T, width: i32);

    #[doc(alias = "cell.left-attach")]
    fn cell_left_attach<T: IsA<Widget>>(&self, item: &T) -> i32;

    #[doc(alias = "cell.left-attach")]
    fn set_cell_left_attach<T: IsA<Widget>>(&self, item: &T, left_attach: i32);

    #[doc(alias = "cell.top-attach")]
    fn cell_top_attach<T: IsA<Widget>>(&self, item: &T) -> i32;

    #[doc(alias = "cell.top-attach")]
    fn set_cell_top_attach<T: IsA<Widget>>(&self, item: &T, top_attach: i32);

    #[doc(alias = "baseline-row")]
    fn connect_baseline_row_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "column-homogeneous")]
    fn connect_column_homogeneous_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "column-spacing")]
    fn connect_column_spacing_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "row-homogeneous")]
    fn connect_row_homogeneous_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "row-spacing")]
    fn connect_row_spacing_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<Grid>> GridExt for O {
    fn attach<P: IsA<Widget>>(&self, child: &P, left: i32, top: i32, width: i32, height: i32) {
        unsafe {
            ffi::gtk_grid_attach(
                self.as_ref().to_glib_none().0,
                child.as_ref().to_glib_none().0,
                left,
                top,
                width,
                height,
            );
        }
    }

    fn attach_next_to<P: IsA<Widget>, Q: IsA<Widget>>(
        &self,
        child: &P,
        sibling: Option<&Q>,
        side: PositionType,
        width: i32,
        height: i32,
    ) {
        unsafe {
            ffi::gtk_grid_attach_next_to(
                self.as_ref().to_glib_none().0,
                child.as_ref().to_glib_none().0,
                sibling.map(|p| p.as_ref()).to_glib_none().0,
                side.into_glib(),
                width,
                height,
            );
        }
    }

    fn baseline_row(&self) -> i32 {
        unsafe { ffi::gtk_grid_get_baseline_row(self.as_ref().to_glib_none().0) }
    }

    fn child_at(&self, left: i32, top: i32) -> Option<Widget> {
        unsafe {
            from_glib_none(ffi::gtk_grid_get_child_at(
                self.as_ref().to_glib_none().0,
                left,
                top,
            ))
        }
    }

    fn is_column_homogeneous(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_grid_get_column_homogeneous(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn column_spacing(&self) -> u32 {
        unsafe { ffi::gtk_grid_get_column_spacing(self.as_ref().to_glib_none().0) }
    }

    fn row_baseline_position(&self, row: i32) -> BaselinePosition {
        unsafe {
            from_glib(ffi::gtk_grid_get_row_baseline_position(
                self.as_ref().to_glib_none().0,
                row,
            ))
        }
    }

    fn is_row_homogeneous(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_grid_get_row_homogeneous(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn row_spacing(&self) -> u32 {
        unsafe { ffi::gtk_grid_get_row_spacing(self.as_ref().to_glib_none().0) }
    }

    fn insert_column(&self, position: i32) {
        unsafe {
            ffi::gtk_grid_insert_column(self.as_ref().to_glib_none().0, position);
        }
    }

    fn insert_next_to<P: IsA<Widget>>(&self, sibling: &P, side: PositionType) {
        unsafe {
            ffi::gtk_grid_insert_next_to(
                self.as_ref().to_glib_none().0,
                sibling.as_ref().to_glib_none().0,
                side.into_glib(),
            );
        }
    }

    fn insert_row(&self, position: i32) {
        unsafe {
            ffi::gtk_grid_insert_row(self.as_ref().to_glib_none().0, position);
        }
    }

    fn remove_column(&self, position: i32) {
        unsafe {
            ffi::gtk_grid_remove_column(self.as_ref().to_glib_none().0, position);
        }
    }

    fn remove_row(&self, position: i32) {
        unsafe {
            ffi::gtk_grid_remove_row(self.as_ref().to_glib_none().0, position);
        }
    }

    fn set_baseline_row(&self, row: i32) {
        unsafe {
            ffi::gtk_grid_set_baseline_row(self.as_ref().to_glib_none().0, row);
        }
    }

    fn set_column_homogeneous(&self, homogeneous: bool) {
        unsafe {
            ffi::gtk_grid_set_column_homogeneous(
                self.as_ref().to_glib_none().0,
                homogeneous.into_glib(),
            );
        }
    }

    fn set_column_spacing(&self, spacing: u32) {
        unsafe {
            ffi::gtk_grid_set_column_spacing(self.as_ref().to_glib_none().0, spacing);
        }
    }

    fn set_row_baseline_position(&self, row: i32, pos: BaselinePosition) {
        unsafe {
            ffi::gtk_grid_set_row_baseline_position(
                self.as_ref().to_glib_none().0,
                row,
                pos.into_glib(),
            );
        }
    }

    fn set_row_homogeneous(&self, homogeneous: bool) {
        unsafe {
            ffi::gtk_grid_set_row_homogeneous(
                self.as_ref().to_glib_none().0,
                homogeneous.into_glib(),
            );
        }
    }

    fn set_row_spacing(&self, spacing: u32) {
        unsafe {
            ffi::gtk_grid_set_row_spacing(self.as_ref().to_glib_none().0, spacing);
        }
    }

    fn cell_height<T: IsA<Widget>>(&self, item: &T) -> i32 {
        unsafe {
            let mut value = glib::Value::from_type(<i32 as StaticType>::static_type());
            crate::ffi::gtk_container_child_get_property(
                self.to_glib_none().0 as *mut crate::ffi::GtkContainer,
                item.to_glib_none().0 as *mut _,
                b"height\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `height` getter")
        }
    }

    fn set_cell_height<T: IsA<Widget>>(&self, item: &T, height: i32) {
        unsafe {
            crate::ffi::gtk_container_child_set_property(
                self.to_glib_none().0 as *mut crate::ffi::GtkContainer,
                item.to_glib_none().0 as *mut _,
                b"height\0".as_ptr() as *const _,
                height.to_value().to_glib_none().0,
            );
        }
    }

    fn cell_width<T: IsA<Widget>>(&self, item: &T) -> i32 {
        unsafe {
            let mut value = glib::Value::from_type(<i32 as StaticType>::static_type());
            crate::ffi::gtk_container_child_get_property(
                self.to_glib_none().0 as *mut crate::ffi::GtkContainer,
                item.to_glib_none().0 as *mut _,
                b"width\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `width` getter")
        }
    }

    fn set_cell_width<T: IsA<Widget>>(&self, item: &T, width: i32) {
        unsafe {
            crate::ffi::gtk_container_child_set_property(
                self.to_glib_none().0 as *mut crate::ffi::GtkContainer,
                item.to_glib_none().0 as *mut _,
                b"width\0".as_ptr() as *const _,
                width.to_value().to_glib_none().0,
            );
        }
    }

    fn cell_left_attach<T: IsA<Widget>>(&self, item: &T) -> i32 {
        unsafe {
            let mut value = glib::Value::from_type(<i32 as StaticType>::static_type());
            crate::ffi::gtk_container_child_get_property(
                self.to_glib_none().0 as *mut crate::ffi::GtkContainer,
                item.to_glib_none().0 as *mut _,
                b"left-attach\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `left-attach` getter")
        }
    }

    fn set_cell_left_attach<T: IsA<Widget>>(&self, item: &T, left_attach: i32) {
        unsafe {
            crate::ffi::gtk_container_child_set_property(
                self.to_glib_none().0 as *mut crate::ffi::GtkContainer,
                item.to_glib_none().0 as *mut _,
                b"left-attach\0".as_ptr() as *const _,
                left_attach.to_value().to_glib_none().0,
            );
        }
    }

    fn cell_top_attach<T: IsA<Widget>>(&self, item: &T) -> i32 {
        unsafe {
            let mut value = glib::Value::from_type(<i32 as StaticType>::static_type());
            crate::ffi::gtk_container_child_get_property(
                self.to_glib_none().0 as *mut crate::ffi::GtkContainer,
                item.to_glib_none().0 as *mut _,
                b"top-attach\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `top-attach` getter")
        }
    }

    fn set_cell_top_attach<T: IsA<Widget>>(&self, item: &T, top_attach: i32) {
        unsafe {
            crate::ffi::gtk_container_child_set_property(
                self.to_glib_none().0 as *mut crate::ffi::GtkContainer,
                item.to_glib_none().0 as *mut _,
                b"top-attach\0".as_ptr() as *const _,
                top_attach.to_value().to_glib_none().0,
            );
        }
    }

    #[doc(alias = "baseline-row")]
    fn connect_baseline_row_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_baseline_row_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkGrid,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<Grid>,
        {
            let f: &F = &*(f as *const F);
            f(&Grid::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::baseline-row\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_baseline_row_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "column-homogeneous")]
    fn connect_column_homogeneous_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_column_homogeneous_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkGrid,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<Grid>,
        {
            let f: &F = &*(f as *const F);
            f(&Grid::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::column-homogeneous\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_column_homogeneous_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "column-spacing")]
    fn connect_column_spacing_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_column_spacing_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkGrid,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<Grid>,
        {
            let f: &F = &*(f as *const F);
            f(&Grid::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::column-spacing\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_column_spacing_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "row-homogeneous")]
    fn connect_row_homogeneous_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_row_homogeneous_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkGrid,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<Grid>,
        {
            let f: &F = &*(f as *const F);
            f(&Grid::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::row-homogeneous\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_row_homogeneous_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "row-spacing")]
    fn connect_row_spacing_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_row_spacing_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkGrid,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<Grid>,
        {
            let f: &F = &*(f as *const F);
            f(&Grid::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::row-spacing\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_row_spacing_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Grid")
    }
}
