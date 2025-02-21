// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files
// DO NOT EDIT

use crate::Setting;
use glib::object::Cast;
use glib::object::ObjectType as ObjectType_;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::StaticType;
use glib::ToValue;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib::wrapper! {
    #[doc(alias = "NMSettingBluetooth")]
    pub struct SettingBluetooth(Object<ffi::NMSettingBluetooth, ffi::NMSettingBluetoothClass>) @extends Setting;

    match fn {
        type_ => || ffi::nm_setting_bluetooth_get_type(),
    }
}

impl SettingBluetooth {
    /// Creates a new [`SettingBluetooth`][crate::SettingBluetooth] object with default values.
    ///
    /// # Returns
    ///
    /// the new empty [`SettingBluetooth`][crate::SettingBluetooth] object
    #[doc(alias = "nm_setting_bluetooth_new")]
    pub fn new() -> SettingBluetooth {
        unsafe { Setting::from_glib_full(ffi::nm_setting_bluetooth_new()).unsafe_cast() }
    }

    /// Gets the Bluetooth address of the remote device which this setting
    /// describes a connection to.
    ///
    /// # Returns
    ///
    /// the Bluetooth address
    #[doc(alias = "nm_setting_bluetooth_get_bdaddr")]
    #[doc(alias = "get_bdaddr")]
    pub fn bdaddr(&self) -> Option<glib::GString> {
        unsafe { from_glib_none(ffi::nm_setting_bluetooth_get_bdaddr(self.to_glib_none().0)) }
    }

    /// Returns the connection method for communicating with the remote device (i.e.
    /// either DUN to a DUN-capable device or PANU to a NAP-capable device).
    ///
    /// # Returns
    ///
    /// the type, either [`SETTING_BLUETOOTH_TYPE_PANU`][crate::SETTING_BLUETOOTH_TYPE_PANU],
    /// [`SETTING_BLUETOOTH_TYPE_NAP`][crate::SETTING_BLUETOOTH_TYPE_NAP] or [`SETTING_BLUETOOTH_TYPE_DUN`][crate::SETTING_BLUETOOTH_TYPE_DUN]
    #[doc(alias = "nm_setting_bluetooth_get_connection_type")]
    #[doc(alias = "get_connection_type")]
    pub fn connection_type(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::nm_setting_bluetooth_get_connection_type(
                self.to_glib_none().0,
            ))
        }
    }

    /// The Bluetooth address of the device.
    pub fn set_bdaddr(&self, bdaddr: Option<&str>) {
        glib::ObjectExt::set_property(self, "bdaddr", &bdaddr)
    }

    /// Either "dun" for Dial-Up Networking connections or "panu" for Personal
    /// Area Networking connections to devices supporting the NAP profile.
    #[doc(alias = "type")]
    pub fn type_(&self) -> Option<glib::GString> {
        glib::ObjectExt::property(self, "type")
    }

    /// Either "dun" for Dial-Up Networking connections or "panu" for Personal
    /// Area Networking connections to devices supporting the NAP profile.
    #[doc(alias = "type")]
    pub fn set_type(&self, type_: Option<&str>) {
        glib::ObjectExt::set_property(self, "type", &type_)
    }

    #[doc(alias = "bdaddr")]
    pub fn connect_bdaddr_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_bdaddr_trampoline<F: Fn(&SettingBluetooth) + 'static>(
            this: *mut ffi::NMSettingBluetooth,
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
                b"notify::bdaddr\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_bdaddr_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "type")]
    pub fn connect_type_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_type_trampoline<F: Fn(&SettingBluetooth) + 'static>(
            this: *mut ffi::NMSettingBluetooth,
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
                b"notify::type\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_type_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl Default for SettingBluetooth {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for SettingBluetooth {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("SettingBluetooth")
    }
}
