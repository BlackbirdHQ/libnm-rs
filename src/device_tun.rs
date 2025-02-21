// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files
// DO NOT EDIT

use crate::Device;
use crate::Object;
#[cfg(any(feature = "v1_2", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_2")))]
use glib::object::ObjectType as ObjectType_;
#[cfg(any(feature = "v1_2", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_2")))]
use glib::signal::connect_raw;
#[cfg(any(feature = "v1_2", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_2")))]
use glib::signal::SignalHandlerId;
use glib::translate::*;
#[cfg(any(feature = "v1_2", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_2")))]
use std::boxed::Box as Box_;
use std::fmt;
#[cfg(any(feature = "v1_2", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_2")))]
use std::mem::transmute;

glib::wrapper! {
    #[doc(alias = "NMDeviceTun")]
    pub struct DeviceTun(Object<ffi::NMDeviceTun, ffi::NMDeviceTunClass>) @extends Device, Object;

    match fn {
        type_ => || ffi::nm_device_tun_get_type(),
    }
}

impl DeviceTun {
    /// Gets the tunnel group.
    ///
    /// # Returns
    ///
    /// the gid of the tunnel group, or -1 if it has no owner.
    #[cfg(any(feature = "v1_2", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_2")))]
    #[doc(alias = "nm_device_tun_get_group")]
    #[doc(alias = "get_group")]
    pub fn group(&self) -> i64 {
        unsafe { ffi::nm_device_tun_get_group(self.to_glib_none().0) }
    }

    /// Gets the hardware (MAC) address of the [`DeviceTun`][crate::DeviceTun]
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
    #[cfg(any(feature = "v1_2", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_2")))]
    #[doc(alias = "nm_device_tun_get_hw_address")]
    #[doc(alias = "get_hw_address")]
    pub fn hw_address(&self) -> Option<glib::GString> {
        unsafe { from_glib_none(ffi::nm_device_tun_get_hw_address(self.to_glib_none().0)) }
    }

    /// Returns the TUN/TAP mode for the device.
    ///
    /// # Returns
    ///
    /// 'tun' or 'tap'
    #[cfg(any(feature = "v1_2", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_2")))]
    #[doc(alias = "nm_device_tun_get_mode")]
    #[doc(alias = "get_mode")]
    pub fn mode(&self) -> Option<glib::GString> {
        unsafe { from_glib_none(ffi::nm_device_tun_get_mode(self.to_glib_none().0)) }
    }

    /// Returns whether the [`DeviceTun`][crate::DeviceTun] has the IFF_MULTI_QUEUE flag.
    ///
    /// # Returns
    ///
    /// [`true`] if the device doesn't have the flag, [`false`] otherwise
    #[cfg(any(feature = "v1_2", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_2")))]
    #[doc(alias = "nm_device_tun_get_multi_queue")]
    #[doc(alias = "get_multi_queue")]
    pub fn is_multi_queue(&self) -> bool {
        unsafe { from_glib(ffi::nm_device_tun_get_multi_queue(self.to_glib_none().0)) }
    }

    #[doc(alias = "nm_device_tun_get_no_pi")]
    #[doc(alias = "get_no_pi")]
    pub fn is_no_pi(&self) -> bool {
        unsafe { from_glib(ffi::nm_device_tun_get_no_pi(self.to_glib_none().0)) }
    }

    /// Gets the tunnel owner.
    ///
    /// # Returns
    ///
    /// the uid of the tunnel owner, or -1 if it has no owner.
    #[cfg(any(feature = "v1_2", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_2")))]
    #[doc(alias = "nm_device_tun_get_owner")]
    #[doc(alias = "get_owner")]
    pub fn owner(&self) -> i64 {
        unsafe { ffi::nm_device_tun_get_owner(self.to_glib_none().0) }
    }

    /// Returns whether the [`DeviceTun`][crate::DeviceTun] has the IFF_VNET_HDR flag.
    ///
    /// # Returns
    ///
    /// [`true`] if the device has the flag, [`false`] otherwise
    #[cfg(any(feature = "v1_2", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_2")))]
    #[doc(alias = "nm_device_tun_get_vnet_hdr")]
    #[doc(alias = "get_vnet_hdr")]
    pub fn is_vnet_hdr(&self) -> bool {
        unsafe { from_glib(ffi::nm_device_tun_get_vnet_hdr(self.to_glib_none().0)) }
    }

    #[cfg(any(feature = "v1_2", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_2")))]
    #[doc(alias = "group")]
    pub fn connect_group_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_group_trampoline<F: Fn(&DeviceTun) + 'static>(
            this: *mut ffi::NMDeviceTun,
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
                b"notify::group\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_group_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v1_2", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_2")))]
    #[doc(alias = "mode")]
    pub fn connect_mode_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_mode_trampoline<F: Fn(&DeviceTun) + 'static>(
            this: *mut ffi::NMDeviceTun,
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
                b"notify::mode\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_mode_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v1_2", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_2")))]
    #[doc(alias = "multi-queue")]
    pub fn connect_multi_queue_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_multi_queue_trampoline<F: Fn(&DeviceTun) + 'static>(
            this: *mut ffi::NMDeviceTun,
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
                b"notify::multi-queue\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_multi_queue_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v1_2", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_2")))]
    #[doc(alias = "no-pi")]
    pub fn connect_no_pi_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_no_pi_trampoline<F: Fn(&DeviceTun) + 'static>(
            this: *mut ffi::NMDeviceTun,
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
                b"notify::no-pi\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_no_pi_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v1_2", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_2")))]
    #[doc(alias = "owner")]
    pub fn connect_owner_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_owner_trampoline<F: Fn(&DeviceTun) + 'static>(
            this: *mut ffi::NMDeviceTun,
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
                b"notify::owner\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_owner_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v1_2", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_2")))]
    #[doc(alias = "vnet-hdr")]
    pub fn connect_vnet_hdr_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_vnet_hdr_trampoline<F: Fn(&DeviceTun) + 'static>(
            this: *mut ffi::NMDeviceTun,
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
                b"notify::vnet-hdr\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_vnet_hdr_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for DeviceTun {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("DeviceTun")
    }
}
