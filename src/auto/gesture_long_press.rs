// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use gdk;
use glib::object::Cast;
use glib::object::IsA;
use glib::object::ObjectType as ObjectType_;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::StaticType;
use glib::ToValue;
use glib::Value;
use glib_sys;
use gobject_sys;
use gtk_sys;
use libc;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;
use EventController;
use Gesture;
use GestureSingle;
use PropagationPhase;
use Widget;

glib_wrapper! {
    pub struct GestureLongPress(Object<gtk_sys::GtkGestureLongPress, gtk_sys::GtkGestureLongPressClass, GestureLongPressClass>) @extends GestureSingle, Gesture, EventController;

    match fn {
        get_type => || gtk_sys::gtk_gesture_long_press_get_type(),
    }
}

impl GestureLongPress {
    pub fn new<P: IsA<Widget>>(widget: &P) -> GestureLongPress {
        skip_assert_initialized!();
        unsafe {
            Gesture::from_glib_full(gtk_sys::gtk_gesture_long_press_new(
                widget.as_ref().to_glib_none().0,
            ))
            .unsafe_cast()
        }
    }

    pub fn get_property_delay_factor(&self) -> f64 {
        unsafe {
            let mut value = Value::from_type(<f64 as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.as_ptr() as *mut gobject_sys::GObject,
                b"delay-factor\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `delay-factor` getter")
                .unwrap()
        }
    }

    pub fn set_property_delay_factor(&self, delay_factor: f64) {
        unsafe {
            gobject_sys::g_object_set_property(
                self.as_ptr() as *mut gobject_sys::GObject,
                b"delay-factor\0".as_ptr() as *const _,
                Value::from(&delay_factor).to_glib_none().0,
            );
        }
    }

    pub fn connect_cancelled<F: Fn(&GestureLongPress) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn cancelled_trampoline<F: Fn(&GestureLongPress) + 'static>(
            this: *mut gtk_sys::GtkGestureLongPress,
            f: glib_sys::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"cancelled\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    cancelled_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_pressed<F: Fn(&GestureLongPress, f64, f64) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn pressed_trampoline<F: Fn(&GestureLongPress, f64, f64) + 'static>(
            this: *mut gtk_sys::GtkGestureLongPress,
            x: libc::c_double,
            y: libc::c_double,
            f: glib_sys::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), x, y)
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"pressed\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    pressed_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_delay_factor_notify<F: Fn(&GestureLongPress) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_delay_factor_trampoline<F: Fn(&GestureLongPress) + 'static>(
            this: *mut gtk_sys::GtkGestureLongPress,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::delay-factor\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_delay_factor_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

#[derive(Clone, Default)]
pub struct GestureLongPressBuilder {
    delay_factor: Option<f64>,
    button: Option<u32>,
    exclusive: Option<bool>,
    touch_only: Option<bool>,
    n_points: Option<u32>,
    window: Option<gdk::Window>,
    propagation_phase: Option<PropagationPhase>,
    widget: Option<Widget>,
}

impl GestureLongPressBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(self) -> GestureLongPress {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref delay_factor) = self.delay_factor {
            properties.push(("delay-factor", delay_factor));
        }
        if let Some(ref button) = self.button {
            properties.push(("button", button));
        }
        if let Some(ref exclusive) = self.exclusive {
            properties.push(("exclusive", exclusive));
        }
        if let Some(ref touch_only) = self.touch_only {
            properties.push(("touch-only", touch_only));
        }
        if let Some(ref n_points) = self.n_points {
            properties.push(("n-points", n_points));
        }
        if let Some(ref window) = self.window {
            properties.push(("window", window));
        }
        if let Some(ref propagation_phase) = self.propagation_phase {
            properties.push(("propagation-phase", propagation_phase));
        }
        if let Some(ref widget) = self.widget {
            properties.push(("widget", widget));
        }
        glib::Object::new(GestureLongPress::static_type(), &properties)
            .expect("object new")
            .downcast()
            .expect("downcast")
    }

    pub fn delay_factor(mut self, delay_factor: f64) -> Self {
        self.delay_factor = Some(delay_factor);
        self
    }

    pub fn button(mut self, button: u32) -> Self {
        self.button = Some(button);
        self
    }

    pub fn exclusive(mut self, exclusive: bool) -> Self {
        self.exclusive = Some(exclusive);
        self
    }

    pub fn touch_only(mut self, touch_only: bool) -> Self {
        self.touch_only = Some(touch_only);
        self
    }

    pub fn n_points(mut self, n_points: u32) -> Self {
        self.n_points = Some(n_points);
        self
    }

    pub fn window<P: IsA<gdk::Window>>(mut self, window: &P) -> Self {
        self.window = Some(window.clone().upcast());
        self
    }

    pub fn propagation_phase(mut self, propagation_phase: PropagationPhase) -> Self {
        self.propagation_phase = Some(propagation_phase);
        self
    }

    pub fn widget<P: IsA<Widget>>(mut self, widget: &P) -> Self {
        self.widget = Some(widget.clone().upcast());
        self
    }
}

impl fmt::Display for GestureLongPress {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "GestureLongPress")
    }
}
