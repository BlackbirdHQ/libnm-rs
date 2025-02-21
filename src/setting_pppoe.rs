// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files
// DO NOT EDIT

use crate::Setting;
use crate::SettingSecretFlags;
use glib::object::Cast;
use glib::object::ObjectType as ObjectType_;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::ToValue;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib::wrapper! {
    #[doc(alias = "NMSettingPppoe")]
    pub struct SettingPppoe(Object<ffi::NMSettingPppoe, ffi::NMSettingPppoeClass>) @extends Setting;

    match fn {
        type_ => || ffi::nm_setting_pppoe_get_type(),
    }
}

impl SettingPppoe {
    /// Creates a new [`SettingPppoe`][crate::SettingPppoe] object with default values.
    ///
    /// # Returns
    ///
    /// the new empty [`SettingPppoe`][crate::SettingPppoe] object
    #[doc(alias = "nm_setting_pppoe_new")]
    pub fn new() -> SettingPppoe {
        unsafe { Setting::from_glib_full(ffi::nm_setting_pppoe_new()).unsafe_cast() }
    }

    ///
    /// # Returns
    ///
    /// the `property::SettingPppoe::parent` property of the setting
    #[cfg(any(feature = "v1_10", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_10")))]
    #[doc(alias = "nm_setting_pppoe_get_parent")]
    #[doc(alias = "get_parent")]
    pub fn parent(&self) -> Option<glib::GString> {
        unsafe { from_glib_none(ffi::nm_setting_pppoe_get_parent(self.to_glib_none().0)) }
    }

    ///
    /// # Returns
    ///
    /// the `property::SettingPppoe::password` property of the setting
    #[doc(alias = "nm_setting_pppoe_get_password")]
    #[doc(alias = "get_password")]
    pub fn password(&self) -> Option<glib::GString> {
        unsafe { from_glib_none(ffi::nm_setting_pppoe_get_password(self.to_glib_none().0)) }
    }

    ///
    /// # Returns
    ///
    /// the [`SettingSecretFlags`][crate::SettingSecretFlags] pertaining to the `property::SettingPppoe::password`
    #[doc(alias = "nm_setting_pppoe_get_password_flags")]
    #[doc(alias = "get_password_flags")]
    pub fn password_flags(&self) -> SettingSecretFlags {
        unsafe {
            from_glib(ffi::nm_setting_pppoe_get_password_flags(
                self.to_glib_none().0,
            ))
        }
    }

    ///
    /// # Returns
    ///
    /// the `property::SettingPppoe::service` property of the setting
    #[doc(alias = "nm_setting_pppoe_get_service")]
    #[doc(alias = "get_service")]
    pub fn service(&self) -> Option<glib::GString> {
        unsafe { from_glib_none(ffi::nm_setting_pppoe_get_service(self.to_glib_none().0)) }
    }

    ///
    /// # Returns
    ///
    /// the `property::SettingPppoe::username` property of the setting
    #[doc(alias = "nm_setting_pppoe_get_username")]
    #[doc(alias = "get_username")]
    pub fn username(&self) -> Option<glib::GString> {
        unsafe { from_glib_none(ffi::nm_setting_pppoe_get_username(self.to_glib_none().0)) }
    }

    /// If given, specifies the parent interface name on which this PPPoE
    /// connection should be created. If this property is not specified,
    /// the connection is activated on the interface specified in
    /// `property::SettingConnection::interface-name` of [`SettingConnection`][crate::SettingConnection].
    #[cfg(any(feature = "v1_10", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_10")))]
    pub fn set_parent(&self, parent: Option<&str>) {
        glib::ObjectExt::set_property(self, "parent", &parent)
    }

    /// Password used to authenticate with the PPPoE service.
    pub fn set_password(&self, password: Option<&str>) {
        glib::ObjectExt::set_property(self, "password", &password)
    }

    /// Flags indicating how to handle the `property::SettingPppoe::password` property.
    #[doc(alias = "password-flags")]
    pub fn set_password_flags(&self, password_flags: SettingSecretFlags) {
        glib::ObjectExt::set_property(self, "password-flags", &password_flags)
    }

    /// If specified, instruct PPPoE to only initiate sessions with access
    /// concentrators that provide the specified service. For most providers,
    /// this should be left blank. It is only required if there are multiple
    /// access concentrators or a specific service is known to be required.
    pub fn set_service(&self, service: Option<&str>) {
        glib::ObjectExt::set_property(self, "service", &service)
    }

    /// Username used to authenticate with the PPPoE service.
    pub fn set_username(&self, username: Option<&str>) {
        glib::ObjectExt::set_property(self, "username", &username)
    }

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_10")))]
    #[doc(alias = "parent")]
    pub fn connect_parent_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_parent_trampoline<F: Fn(&SettingPppoe) + 'static>(
            this: *mut ffi::NMSettingPppoe,
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

    #[doc(alias = "password")]
    pub fn connect_password_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_password_trampoline<F: Fn(&SettingPppoe) + 'static>(
            this: *mut ffi::NMSettingPppoe,
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
                b"notify::password\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_password_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "password-flags")]
    pub fn connect_password_flags_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_password_flags_trampoline<F: Fn(&SettingPppoe) + 'static>(
            this: *mut ffi::NMSettingPppoe,
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
                b"notify::password-flags\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_password_flags_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "service")]
    pub fn connect_service_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_service_trampoline<F: Fn(&SettingPppoe) + 'static>(
            this: *mut ffi::NMSettingPppoe,
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
                b"notify::service\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_service_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "username")]
    pub fn connect_username_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_username_trampoline<F: Fn(&SettingPppoe) + 'static>(
            this: *mut ffi::NMSettingPppoe,
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
                b"notify::username\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_username_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl Default for SettingPppoe {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for SettingPppoe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("SettingPppoe")
    }
}
