// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files
// DO NOT EDIT

use crate::Device;
use crate::Object;
#[cfg(any(feature = "v1_22", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_22")))]
use glib::object::ObjectType as ObjectType_;
#[cfg(any(feature = "v1_22", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_22")))]
use glib::signal::connect_raw;
#[cfg(any(feature = "v1_22", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_22")))]
use glib::signal::SignalHandlerId;
#[cfg(any(feature = "v1_14", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_14")))]
use glib::translate::*;
#[cfg(any(feature = "v1_22", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_22")))]
use std::boxed::Box as Box_;
use std::fmt;
#[cfg(any(feature = "v1_22", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_22")))]
use std::mem::transmute;

glib::wrapper! {
    #[doc(alias = "NMDeviceOvsPort")]
    pub struct DeviceOvsPort(Object<ffi::NMDeviceOvsPort, ffi::NMDeviceOvsPortClass>) @extends Device, Object;

    match fn {
        type_ => || ffi::nm_device_ovs_port_get_type(),
    }
}

impl DeviceOvsPort {
    /// Gets the interfaces currently enslaved to `self`.
    ///
    /// # Deprecated since 1.34
    ///
    /// Use [`DeviceExt::ports()`][crate::prelude::DeviceExt::ports()] instead.
    ///
    /// # Returns
    ///
    /// the [`glib::PtrArray`][crate::glib::PtrArray] containing
    /// `NMDevices` that are slaves of `self`. This is the internal
    /// copy used by the device, and must not be modified.
    #[cfg_attr(feature = "v1_34", deprecated = "Since 1.34")]
    #[cfg(any(feature = "v1_14", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_14")))]
    #[doc(alias = "nm_device_ovs_port_get_slaves")]
    #[doc(alias = "get_slaves")]
    pub fn slaves(&self) -> Vec<Device> {
        unsafe {
            FromGlibPtrContainer::from_glib_none(ffi::nm_device_ovs_port_get_slaves(
                self.to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v1_22", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_22")))]
    #[doc(alias = "slaves")]
    pub fn connect_slaves_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_slaves_trampoline<F: Fn(&DeviceOvsPort) + 'static>(
            this: *mut ffi::NMDeviceOvsPort,
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
                b"notify::slaves\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_slaves_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for DeviceOvsPort {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("DeviceOvsPort")
    }
}
