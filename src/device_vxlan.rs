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
#[cfg(any(feature = "v1_2", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_2")))]
use glib::translate::*;
#[cfg(any(feature = "v1_2", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_2")))]
use std::boxed::Box as Box_;
use std::fmt;
#[cfg(any(feature = "v1_2", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_2")))]
use std::mem::transmute;

glib::wrapper! {
    #[doc(alias = "NMDeviceVxlan")]
    pub struct DeviceVxlan(Object<ffi::NMDeviceVxlan, ffi::NMDeviceVxlanClass>) @extends Device, Object;

    match fn {
        type_ => || ffi::nm_device_vxlan_get_type(),
    }
}

impl DeviceVxlan {
    ///
    /// # Returns
    ///
    /// the lifetime in seconds of FDB entries learnt by the kernel
    #[cfg(any(feature = "v1_2", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_2")))]
    #[doc(alias = "nm_device_vxlan_get_ageing")]
    #[doc(alias = "get_ageing")]
    pub fn ageing(&self) -> u32 {
        unsafe { ffi::nm_device_vxlan_get_ageing(self.to_glib_none().0) }
    }

    /// Whether the device has carrier.
    ///
    /// # Returns
    ///
    /// [`true`] if the device has carrier.
    #[cfg(any(feature = "v1_2", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_2")))]
    #[doc(alias = "nm_device_vxlan_get_carrier")]
    #[doc(alias = "get_carrier")]
    pub fn is_carrier(&self) -> bool {
        unsafe { from_glib(ffi::nm_device_vxlan_get_carrier(self.to_glib_none().0)) }
    }

    ///
    /// # Returns
    ///
    /// the UDP destination port
    #[cfg(any(feature = "v1_2", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_2")))]
    #[doc(alias = "nm_device_vxlan_get_dst_port")]
    #[doc(alias = "get_dst_port")]
    pub fn dst_port(&self) -> u32 {
        unsafe { ffi::nm_device_vxlan_get_dst_port(self.to_glib_none().0) }
    }

    ///
    /// # Returns
    ///
    /// The unicast destination IP address or the multicast
    /// IP address joined
    #[cfg(any(feature = "v1_2", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_2")))]
    #[doc(alias = "nm_device_vxlan_get_group")]
    #[doc(alias = "get_group")]
    pub fn group(&self) -> Option<glib::GString> {
        unsafe { from_glib_none(ffi::nm_device_vxlan_get_group(self.to_glib_none().0)) }
    }

    /// Gets the hardware (MAC) address of the [`DeviceVxlan`][crate::DeviceVxlan]
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
    #[doc(alias = "nm_device_vxlan_get_hw_address")]
    #[doc(alias = "get_hw_address")]
    pub fn hw_address(&self) -> Option<glib::GString> {
        unsafe { from_glib_none(ffi::nm_device_vxlan_get_hw_address(self.to_glib_none().0)) }
    }

    ///
    /// # Returns
    ///
    /// the device's VXLAN ID.
    #[cfg(any(feature = "v1_2", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_2")))]
    #[doc(alias = "nm_device_vxlan_get_id")]
    #[doc(alias = "get_id")]
    pub fn id(&self) -> u32 {
        unsafe { ffi::nm_device_vxlan_get_id(self.to_glib_none().0) }
    }

    ///
    /// # Returns
    ///
    /// whether netlink LL ADDR miss notifications are generated
    #[cfg(any(feature = "v1_2", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_2")))]
    #[doc(alias = "nm_device_vxlan_get_l2miss")]
    #[doc(alias = "get_l2miss")]
    pub fn is_l2miss(&self) -> bool {
        unsafe { from_glib(ffi::nm_device_vxlan_get_l2miss(self.to_glib_none().0)) }
    }

    ///
    /// # Returns
    ///
    /// whether netlink IP ADDR miss notifications are generated
    #[cfg(any(feature = "v1_2", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_2")))]
    #[doc(alias = "nm_device_vxlan_get_l3miss")]
    #[doc(alias = "get_l3miss")]
    pub fn is_l3miss(&self) -> bool {
        unsafe { from_glib(ffi::nm_device_vxlan_get_l3miss(self.to_glib_none().0)) }
    }

    ///
    /// # Returns
    ///
    /// whether address learning is enabled
    #[cfg(any(feature = "v1_2", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_2")))]
    #[doc(alias = "nm_device_vxlan_get_learning")]
    #[doc(alias = "get_learning")]
    pub fn is_learning(&self) -> bool {
        unsafe { from_glib(ffi::nm_device_vxlan_get_learning(self.to_glib_none().0)) }
    }

    ///
    /// # Returns
    ///
    /// the maximum number of entries that can be added to the
    /// forwarding table
    #[cfg(any(feature = "v1_2", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_2")))]
    #[doc(alias = "nm_device_vxlan_get_limit")]
    #[doc(alias = "get_limit")]
    pub fn limit(&self) -> u32 {
        unsafe { ffi::nm_device_vxlan_get_limit(self.to_glib_none().0) }
    }

    ///
    /// # Returns
    ///
    /// the source IP address to use in outgoing packets
    #[cfg(any(feature = "v1_2", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_2")))]
    #[doc(alias = "nm_device_vxlan_get_local")]
    #[doc(alias = "get_local")]
    pub fn local(&self) -> Option<glib::GString> {
        unsafe { from_glib_none(ffi::nm_device_vxlan_get_local(self.to_glib_none().0)) }
    }

    ///
    /// # Returns
    ///
    /// the device's parent device
    #[cfg(any(feature = "v1_2", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_2")))]
    #[doc(alias = "nm_device_vxlan_get_parent")]
    #[doc(alias = "get_parent")]
    pub fn parent(&self) -> Option<Device> {
        unsafe { from_glib_none(ffi::nm_device_vxlan_get_parent(self.to_glib_none().0)) }
    }

    ///
    /// # Returns
    ///
    /// whether ARP proxy is turned on
    #[cfg(any(feature = "v1_2", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_2")))]
    #[doc(alias = "nm_device_vxlan_get_proxy")]
    #[doc(alias = "get_proxy")]
    pub fn is_proxy(&self) -> bool {
        unsafe { from_glib(ffi::nm_device_vxlan_get_proxy(self.to_glib_none().0)) }
    }

    ///
    /// # Returns
    ///
    /// whether route short circuit is turned on
    #[cfg(any(feature = "v1_2", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_2")))]
    #[doc(alias = "nm_device_vxlan_get_rsc")]
    #[doc(alias = "get_rsc")]
    pub fn is_rsc(&self) -> bool {
        unsafe { from_glib(ffi::nm_device_vxlan_get_rsc(self.to_glib_none().0)) }
    }

    ///
    /// # Returns
    ///
    /// the maximum UDP source port
    #[cfg(any(feature = "v1_2", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_2")))]
    #[doc(alias = "nm_device_vxlan_get_src_port_max")]
    #[doc(alias = "get_src_port_max")]
    pub fn src_port_max(&self) -> u32 {
        unsafe { ffi::nm_device_vxlan_get_src_port_max(self.to_glib_none().0) }
    }

    ///
    /// # Returns
    ///
    /// the minimum UDP source port
    #[cfg(any(feature = "v1_2", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_2")))]
    #[doc(alias = "nm_device_vxlan_get_src_port_min")]
    #[doc(alias = "get_src_port_min")]
    pub fn src_port_min(&self) -> u32 {
        unsafe { ffi::nm_device_vxlan_get_src_port_min(self.to_glib_none().0) }
    }

    ///
    /// # Returns
    ///
    /// the TOS value to use in outgoing packets
    #[cfg(any(feature = "v1_2", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_2")))]
    #[doc(alias = "nm_device_vxlan_get_tos")]
    #[doc(alias = "get_tos")]
    pub fn tos(&self) -> u32 {
        unsafe { ffi::nm_device_vxlan_get_tos(self.to_glib_none().0) }
    }

    ///
    /// # Returns
    ///
    /// the time-to-live value to use in outgoing packets
    #[cfg(any(feature = "v1_2", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_2")))]
    #[doc(alias = "nm_device_vxlan_get_ttl")]
    #[doc(alias = "get_ttl")]
    pub fn ttl(&self) -> u32 {
        unsafe { ffi::nm_device_vxlan_get_ttl(self.to_glib_none().0) }
    }

    #[cfg(any(feature = "v1_2", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_2")))]
    #[doc(alias = "ageing")]
    pub fn connect_ageing_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_ageing_trampoline<F: Fn(&DeviceVxlan) + 'static>(
            this: *mut ffi::NMDeviceVxlan,
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
                b"notify::ageing\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_ageing_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v1_2", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_2")))]
    #[doc(alias = "carrier")]
    pub fn connect_carrier_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_carrier_trampoline<F: Fn(&DeviceVxlan) + 'static>(
            this: *mut ffi::NMDeviceVxlan,
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
                b"notify::carrier\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_carrier_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v1_2", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_2")))]
    #[doc(alias = "dst-port")]
    pub fn connect_dst_port_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_dst_port_trampoline<F: Fn(&DeviceVxlan) + 'static>(
            this: *mut ffi::NMDeviceVxlan,
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
                b"notify::dst-port\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_dst_port_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v1_2", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_2")))]
    #[doc(alias = "group")]
    pub fn connect_group_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_group_trampoline<F: Fn(&DeviceVxlan) + 'static>(
            this: *mut ffi::NMDeviceVxlan,
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
    #[doc(alias = "id")]
    pub fn connect_id_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_id_trampoline<F: Fn(&DeviceVxlan) + 'static>(
            this: *mut ffi::NMDeviceVxlan,
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
                b"notify::id\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_id_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v1_2", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_2")))]
    #[doc(alias = "l2miss")]
    pub fn connect_l2miss_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_l2miss_trampoline<F: Fn(&DeviceVxlan) + 'static>(
            this: *mut ffi::NMDeviceVxlan,
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
                b"notify::l2miss\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_l2miss_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v1_2", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_2")))]
    #[doc(alias = "l3miss")]
    pub fn connect_l3miss_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_l3miss_trampoline<F: Fn(&DeviceVxlan) + 'static>(
            this: *mut ffi::NMDeviceVxlan,
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
                b"notify::l3miss\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_l3miss_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v1_2", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_2")))]
    #[doc(alias = "learning")]
    pub fn connect_learning_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_learning_trampoline<F: Fn(&DeviceVxlan) + 'static>(
            this: *mut ffi::NMDeviceVxlan,
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
                b"notify::learning\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_learning_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v1_2", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_2")))]
    #[doc(alias = "limit")]
    pub fn connect_limit_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_limit_trampoline<F: Fn(&DeviceVxlan) + 'static>(
            this: *mut ffi::NMDeviceVxlan,
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
                b"notify::limit\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_limit_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v1_2", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_2")))]
    #[doc(alias = "local")]
    pub fn connect_local_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_local_trampoline<F: Fn(&DeviceVxlan) + 'static>(
            this: *mut ffi::NMDeviceVxlan,
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
                b"notify::local\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_local_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v1_2", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_2")))]
    #[doc(alias = "parent")]
    pub fn connect_parent_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_parent_trampoline<F: Fn(&DeviceVxlan) + 'static>(
            this: *mut ffi::NMDeviceVxlan,
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

    #[cfg(any(feature = "v1_2", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_2")))]
    #[doc(alias = "proxy")]
    pub fn connect_proxy_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_proxy_trampoline<F: Fn(&DeviceVxlan) + 'static>(
            this: *mut ffi::NMDeviceVxlan,
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
                b"notify::proxy\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_proxy_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v1_2", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_2")))]
    #[doc(alias = "rsc")]
    pub fn connect_rsc_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_rsc_trampoline<F: Fn(&DeviceVxlan) + 'static>(
            this: *mut ffi::NMDeviceVxlan,
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
                b"notify::rsc\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_rsc_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v1_2", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_2")))]
    #[doc(alias = "src-port-max")]
    pub fn connect_src_port_max_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_src_port_max_trampoline<F: Fn(&DeviceVxlan) + 'static>(
            this: *mut ffi::NMDeviceVxlan,
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
                b"notify::src-port-max\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_src_port_max_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v1_2", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_2")))]
    #[doc(alias = "src-port-min")]
    pub fn connect_src_port_min_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_src_port_min_trampoline<F: Fn(&DeviceVxlan) + 'static>(
            this: *mut ffi::NMDeviceVxlan,
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
                b"notify::src-port-min\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_src_port_min_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v1_2", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_2")))]
    #[doc(alias = "tos")]
    pub fn connect_tos_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_tos_trampoline<F: Fn(&DeviceVxlan) + 'static>(
            this: *mut ffi::NMDeviceVxlan,
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
                b"notify::tos\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_tos_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v1_2", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_2")))]
    #[doc(alias = "ttl")]
    pub fn connect_ttl_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_ttl_trampoline<F: Fn(&DeviceVxlan) + 'static>(
            this: *mut ffi::NMDeviceVxlan,
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
                b"notify::ttl\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_ttl_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for DeviceVxlan {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("DeviceVxlan")
    }
}
