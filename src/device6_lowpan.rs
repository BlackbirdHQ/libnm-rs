// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files
// DO NOT EDIT

use crate::Device;
use crate::Object;
#[cfg(any(feature = "v1_14", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_14")))]
use glib::object::ObjectType as ObjectType_;
#[cfg(any(feature = "v1_14", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_14")))]
use glib::signal::connect_raw;
#[cfg(any(feature = "v1_14", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_14")))]
use glib::signal::SignalHandlerId;
#[cfg(any(feature = "v1_14", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_14")))]
use glib::translate::*;
#[cfg(any(feature = "v1_14", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_14")))]
use std::boxed::Box as Box_;
use std::fmt;
#[cfg(any(feature = "v1_14", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_14")))]
use std::mem::transmute;

glib::wrapper! {
    #[doc(alias = "NMDevice6Lowpan")]
    pub struct Device6Lowpan(Object<ffi::NMDevice6Lowpan, ffi::NMDevice6LowpanClass>) @extends Device, Object;

    match fn {
        type_ => || ffi::nm_device_6lowpan_get_type(),
    }
}

impl Device6Lowpan {
    /// Gets the hardware (MAC) address of the [`Device6Lowpan`][crate::Device6Lowpan]
    ///
    /// # Deprecated since 1.24
    ///
    /// Use [`DeviceExt::hw_address()`][crate::prelude::DeviceExt::hw_address()] instead.
    ///
    /// # Returns
    ///
    /// the hardware address. This is the internal string used by the
    /// device, and must not be modified.
    #[cfg_attr(feature = "v1_24", deprecated = "Since 1.24")]
    #[cfg(any(feature = "v1_14", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_14")))]
    #[doc(alias = "nm_device_6lowpan_get_hw_address")]
    #[doc(alias = "get_hw_address")]
    pub fn hw_address(&self) -> Option<glib::GString> {
        unsafe { from_glib_none(ffi::nm_device_6lowpan_get_hw_address(self.to_glib_none().0)) }
    }

    ///
    /// # Returns
    ///
    /// the device's parent device
    #[cfg(any(feature = "v1_14", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_14")))]
    #[doc(alias = "nm_device_6lowpan_get_parent")]
    #[doc(alias = "get_parent")]
    pub fn parent(&self) -> Option<Device> {
        unsafe { from_glib_none(ffi::nm_device_6lowpan_get_parent(self.to_glib_none().0)) }
    }

    #[cfg(any(feature = "v1_14", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_14")))]
    #[doc(alias = "parent")]
    pub fn connect_parent_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_parent_trampoline<F: Fn(&Device6Lowpan) + 'static>(
            this: *mut ffi::NMDevice6Lowpan,
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
                b"notify::parent\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_parent_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for Device6Lowpan {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Device6Lowpan")
    }
}
