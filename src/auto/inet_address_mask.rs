// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use Error;
use InetAddress;
use SocketFamily;
use ffi;
use glib;
use glib::GString;
use glib::Value;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect_raw;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct InetAddressMask(Object<ffi::GInetAddressMask, ffi::GInetAddressMaskClass, InetAddressMaskClass>);

    match fn {
        get_type => || ffi::g_inet_address_mask_get_type(),
    }
}

impl InetAddressMask {
    pub fn new<P: IsA<InetAddress>>(addr: &P, length: u32) -> Result<InetAddressMask, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::g_inet_address_mask_new(addr.as_ref().to_glib_none().0, length, &mut error);
            if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    pub fn new_from_string(mask_string: &str) -> Result<InetAddressMask, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::g_inet_address_mask_new_from_string(mask_string.to_glib_none().0, &mut error);
            if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) }
        }
    }
}

unsafe impl Send for InetAddressMask {}
unsafe impl Sync for InetAddressMask {}

pub const NONE_INET_ADDRESS_MASK: Option<&InetAddressMask> = None;

pub trait InetAddressMaskExt: 'static {
    fn get_address(&self) -> InetAddress;

    fn get_family(&self) -> SocketFamily;

    fn get_length(&self) -> u32;

    fn matches<P: IsA<InetAddress>>(&self, address: &P) -> bool;

    fn to_string(&self) -> GString;

    fn set_property_address<P: IsA<InetAddress> + glib::value::SetValueOptional>(&self, address: Option<&P>);

    fn set_property_length(&self, length: u32);

    fn connect_property_address_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_family_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_length_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<InetAddressMask>> InetAddressMaskExt for O {
    fn get_address(&self) -> InetAddress {
        unsafe {
            from_glib_none(ffi::g_inet_address_mask_get_address(self.as_ref().to_glib_none().0))
        }
    }

    fn get_family(&self) -> SocketFamily {
        unsafe {
            from_glib(ffi::g_inet_address_mask_get_family(self.as_ref().to_glib_none().0))
        }
    }

    fn get_length(&self) -> u32 {
        unsafe {
            ffi::g_inet_address_mask_get_length(self.as_ref().to_glib_none().0)
        }
    }

    fn matches<P: IsA<InetAddress>>(&self, address: &P) -> bool {
        unsafe {
            from_glib(ffi::g_inet_address_mask_matches(self.as_ref().to_glib_none().0, address.as_ref().to_glib_none().0))
        }
    }

    fn to_string(&self) -> GString {
        unsafe {
            from_glib_full(ffi::g_inet_address_mask_to_string(self.as_ref().to_glib_none().0))
        }
    }

    fn set_property_address<P: IsA<InetAddress> + glib::value::SetValueOptional>(&self, address: Option<&P>) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0 as *mut gobject_ffi::GObject, b"address\0".as_ptr() as *const _, Value::from(address).to_glib_none().0);
        }
    }

    fn set_property_length(&self, length: u32) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0 as *mut gobject_ffi::GObject, b"length\0".as_ptr() as *const _, Value::from(&length).to_glib_none().0);
        }
    }

    fn connect_property_address_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + Send + Sync + 'static>> = Box_::new(Box_::new(f));
            connect_raw(self.as_ptr() as *mut _, b"notify::address\0".as_ptr() as *const _,
                transmute(notify_address_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_family_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + Send + Sync + 'static>> = Box_::new(Box_::new(f));
            connect_raw(self.as_ptr() as *mut _, b"notify::family\0".as_ptr() as *const _,
                transmute(notify_family_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_length_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + Send + Sync + 'static>> = Box_::new(Box_::new(f));
            connect_raw(self.as_ptr() as *mut _, b"notify::length\0".as_ptr() as *const _,
                transmute(notify_length_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn notify_address_trampoline<P>(this: *mut ffi::GInetAddressMask, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<InetAddressMask> {
    let f: &&(Fn(&P) + Send + Sync + 'static) = transmute(f);
    f(&InetAddressMask::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_family_trampoline<P>(this: *mut ffi::GInetAddressMask, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<InetAddressMask> {
    let f: &&(Fn(&P) + Send + Sync + 'static) = transmute(f);
    f(&InetAddressMask::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_length_trampoline<P>(this: *mut ffi::GInetAddressMask, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<InetAddressMask> {
    let f: &&(Fn(&P) + Send + Sync + 'static) = transmute(f);
    f(&InetAddressMask::from_glib_borrow(this).unsafe_cast())
}

impl fmt::Display for InetAddressMask {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "InetAddressMask")
    }
}
