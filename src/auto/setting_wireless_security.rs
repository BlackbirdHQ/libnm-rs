// This file was generated by gir (https://github.com/gtk-rs/gir @ 464833e)
// from gir-files (https://github.com/gtk-rs/gir-files @ ???)
// DO NOT EDIT

use ffi;
use glib;
use glib::object::Downcast;
use glib::object::IsA;
use glib::signal::connect;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::StaticType;
use glib::Value;
use glib_ffi;
use gobject_ffi;
use std::boxed::Box as Box_;
use std::mem;
use std::mem::transmute;
use std::ptr;
use Setting;

glib_wrapper! {
    pub struct SettingWirelessSecurity(Object<ffi::NMSettingWirelessSecurity, ffi::NMSettingWirelessSecurityClass>): Setting;

    match fn {
        get_type => || ffi::nm_setting_wireless_security_get_type(),
    }
}

impl SettingWirelessSecurity {
    pub fn new() -> SettingWirelessSecurity {
        unsafe {
            Setting::from_glib_full(ffi::nm_setting_wireless_security_new()).downcast_unchecked()
        }
    }
}

impl Default for SettingWirelessSecurity {
    fn default() -> Self {
        Self::new()
    }
}

pub trait SettingWirelessSecurityExt {
    fn add_group(&self, group: &str) -> bool;

    fn add_pairwise(&self, pairwise: &str) -> bool;

    fn add_proto(&self, proto: &str) -> bool;

    fn clear_groups(&self);

    fn clear_pairwise(&self);

    fn clear_protos(&self);

    fn get_auth_alg(&self) -> Option<String>;

    //#[cfg(any(feature = "v1_12", feature = "dox"))]
    //fn get_fils(&self) -> /*Ignored*/SettingWirelessSecurityFils;

    fn get_group(&self, i: u32) -> Option<String>;

    fn get_key_mgmt(&self) -> Option<String>;

    fn get_leap_password(&self) -> Option<String>;

    //fn get_leap_password_flags(&self) -> /*Ignored*/SettingSecretFlags;

    fn get_leap_username(&self) -> Option<String>;

    fn get_num_groups(&self) -> u32;

    fn get_num_pairwise(&self) -> u32;

    fn get_num_protos(&self) -> u32;

    fn get_pairwise(&self, i: u32) -> Option<String>;

    //fn get_pmf(&self) -> /*Ignored*/SettingWirelessSecurityPmf;

    fn get_proto(&self, i: u32) -> Option<String>;

    fn get_psk(&self) -> Option<String>;

    //fn get_psk_flags(&self) -> /*Ignored*/SettingSecretFlags;

    fn get_wep_key(&self, idx: u32) -> Option<String>;

    //fn get_wep_key_flags(&self) -> /*Ignored*/SettingSecretFlags;

    //fn get_wep_key_type(&self) -> /*Ignored*/WepKeyType;

    fn get_wep_tx_keyidx(&self) -> u32;

    //#[cfg(any(feature = "v1_10", feature = "dox"))]
    //fn get_wps_method(&self) -> /*Ignored*/SettingWirelessSecurityWpsMethod;

    fn remove_group(&self, i: u32);

    fn remove_group_by_value(&self, group: &str) -> bool;

    fn remove_pairwise(&self, i: u32);

    fn remove_pairwise_by_value(&self, pairwise: &str) -> bool;

    fn remove_proto(&self, i: u32);

    fn remove_proto_by_value(&self, proto: &str) -> bool;

    fn set_wep_key(&self, idx: u32, key: &str);

    fn set_property_auth_alg(&self, auth_alg: Option<&str>);

    #[cfg(any(feature = "v1_12", feature = "dox"))]
    fn set_property_fils(&self, fils: i32);

    fn set_property_group(&self, group: &[&str]);

    fn set_property_key_mgmt(&self, key_mgmt: Option<&str>);

    fn set_property_leap_password(&self, leap_password: Option<&str>);

    //fn set_property_leap_password_flags(&self, leap_password_flags: /*Ignored*/SettingSecretFlags);

    fn set_property_leap_username(&self, leap_username: Option<&str>);

    fn set_property_pairwise(&self, pairwise: &[&str]);

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    fn set_property_pmf(&self, pmf: i32);

    fn set_property_proto(&self, proto: &[&str]);

    fn set_property_psk(&self, psk: Option<&str>);

    //fn set_property_psk_flags(&self, psk_flags: /*Ignored*/SettingSecretFlags);

    //fn set_property_wep_key_flags(&self, wep_key_flags: /*Ignored*/SettingSecretFlags);

    //fn set_property_wep_key_type(&self, wep_key_type: /*Ignored*/WepKeyType);

    fn get_property_wep_key0(&self) -> Option<String>;

    fn set_property_wep_key0(&self, wep_key0: Option<&str>);

    fn get_property_wep_key1(&self) -> Option<String>;

    fn set_property_wep_key1(&self, wep_key1: Option<&str>);

    fn get_property_wep_key2(&self) -> Option<String>;

    fn set_property_wep_key2(&self, wep_key2: Option<&str>);

    fn get_property_wep_key3(&self) -> Option<String>;

    fn set_property_wep_key3(&self, wep_key3: Option<&str>);

    fn set_property_wep_tx_keyidx(&self, wep_tx_keyidx: u32);

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    fn set_property_wps_method(&self, wps_method: u32);

    fn connect_property_auth_alg_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v1_12", feature = "dox"))]
    fn connect_property_fils_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_group_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_key_mgmt_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_leap_password_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_leap_password_flags_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_leap_username_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_pairwise_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    fn connect_property_pmf_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_proto_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_psk_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_psk_flags_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_wep_key_flags_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_wep_key_type_notify<F: Fn(&Self) + 'static>(&self, f: F)
        -> SignalHandlerId;

    fn connect_property_wep_key0_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_wep_key1_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_wep_key2_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_wep_key3_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_wep_tx_keyidx_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    fn connect_property_wps_method_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<SettingWirelessSecurity> + IsA<glib::object::Object>> SettingWirelessSecurityExt for O {
    fn add_group(&self, group: &str) -> bool {
        unsafe {
            from_glib(ffi::nm_setting_wireless_security_add_group(
                self.to_glib_none().0,
                group.to_glib_none().0,
            ))
        }
    }

    fn add_pairwise(&self, pairwise: &str) -> bool {
        unsafe {
            from_glib(ffi::nm_setting_wireless_security_add_pairwise(
                self.to_glib_none().0,
                pairwise.to_glib_none().0,
            ))
        }
    }

    fn add_proto(&self, proto: &str) -> bool {
        unsafe {
            from_glib(ffi::nm_setting_wireless_security_add_proto(
                self.to_glib_none().0,
                proto.to_glib_none().0,
            ))
        }
    }

    fn clear_groups(&self) {
        unsafe {
            ffi::nm_setting_wireless_security_clear_groups(self.to_glib_none().0);
        }
    }

    fn clear_pairwise(&self) {
        unsafe {
            ffi::nm_setting_wireless_security_clear_pairwise(self.to_glib_none().0);
        }
    }

    fn clear_protos(&self) {
        unsafe {
            ffi::nm_setting_wireless_security_clear_protos(self.to_glib_none().0);
        }
    }

    fn get_auth_alg(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::nm_setting_wireless_security_get_auth_alg(
                self.to_glib_none().0,
            ))
        }
    }

    //#[cfg(any(feature = "v1_12", feature = "dox"))]
    //fn get_fils(&self) -> /*Ignored*/SettingWirelessSecurityFils {
    //    unsafe { TODO: call ffi::nm_setting_wireless_security_get_fils() }
    //}

    fn get_group(&self, i: u32) -> Option<String> {
        unsafe {
            from_glib_none(ffi::nm_setting_wireless_security_get_group(
                self.to_glib_none().0,
                i,
            ))
        }
    }

    fn get_key_mgmt(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::nm_setting_wireless_security_get_key_mgmt(
                self.to_glib_none().0,
            ))
        }
    }

    fn get_leap_password(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::nm_setting_wireless_security_get_leap_password(
                self.to_glib_none().0,
            ))
        }
    }

    //fn get_leap_password_flags(&self) -> /*Ignored*/SettingSecretFlags {
    //    unsafe { TODO: call ffi::nm_setting_wireless_security_get_leap_password_flags() }
    //}

    fn get_leap_username(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::nm_setting_wireless_security_get_leap_username(
                self.to_glib_none().0,
            ))
        }
    }

    fn get_num_groups(&self) -> u32 {
        unsafe { ffi::nm_setting_wireless_security_get_num_groups(self.to_glib_none().0) }
    }

    fn get_num_pairwise(&self) -> u32 {
        unsafe { ffi::nm_setting_wireless_security_get_num_pairwise(self.to_glib_none().0) }
    }

    fn get_num_protos(&self) -> u32 {
        unsafe { ffi::nm_setting_wireless_security_get_num_protos(self.to_glib_none().0) }
    }

    fn get_pairwise(&self, i: u32) -> Option<String> {
        unsafe {
            from_glib_none(ffi::nm_setting_wireless_security_get_pairwise(
                self.to_glib_none().0,
                i,
            ))
        }
    }

    //fn get_pmf(&self) -> /*Ignored*/SettingWirelessSecurityPmf {
    //    unsafe { TODO: call ffi::nm_setting_wireless_security_get_pmf() }
    //}

    fn get_proto(&self, i: u32) -> Option<String> {
        unsafe {
            from_glib_none(ffi::nm_setting_wireless_security_get_proto(
                self.to_glib_none().0,
                i,
            ))
        }
    }

    fn get_psk(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::nm_setting_wireless_security_get_psk(
                self.to_glib_none().0,
            ))
        }
    }

    //fn get_psk_flags(&self) -> /*Ignored*/SettingSecretFlags {
    //    unsafe { TODO: call ffi::nm_setting_wireless_security_get_psk_flags() }
    //}

    fn get_wep_key(&self, idx: u32) -> Option<String> {
        unsafe {
            from_glib_none(ffi::nm_setting_wireless_security_get_wep_key(
                self.to_glib_none().0,
                idx,
            ))
        }
    }

    //fn get_wep_key_flags(&self) -> /*Ignored*/SettingSecretFlags {
    //    unsafe { TODO: call ffi::nm_setting_wireless_security_get_wep_key_flags() }
    //}

    //fn get_wep_key_type(&self) -> /*Ignored*/WepKeyType {
    //    unsafe { TODO: call ffi::nm_setting_wireless_security_get_wep_key_type() }
    //}

    fn get_wep_tx_keyidx(&self) -> u32 {
        unsafe { ffi::nm_setting_wireless_security_get_wep_tx_keyidx(self.to_glib_none().0) }
    }

    //#[cfg(any(feature = "v1_10", feature = "dox"))]
    //fn get_wps_method(&self) -> /*Ignored*/SettingWirelessSecurityWpsMethod {
    //    unsafe { TODO: call ffi::nm_setting_wireless_security_get_wps_method() }
    //}

    fn remove_group(&self, i: u32) {
        unsafe {
            ffi::nm_setting_wireless_security_remove_group(self.to_glib_none().0, i);
        }
    }

    fn remove_group_by_value(&self, group: &str) -> bool {
        unsafe {
            from_glib(ffi::nm_setting_wireless_security_remove_group_by_value(
                self.to_glib_none().0,
                group.to_glib_none().0,
            ))
        }
    }

    fn remove_pairwise(&self, i: u32) {
        unsafe {
            ffi::nm_setting_wireless_security_remove_pairwise(self.to_glib_none().0, i);
        }
    }

    fn remove_pairwise_by_value(&self, pairwise: &str) -> bool {
        unsafe {
            from_glib(ffi::nm_setting_wireless_security_remove_pairwise_by_value(
                self.to_glib_none().0,
                pairwise.to_glib_none().0,
            ))
        }
    }

    fn remove_proto(&self, i: u32) {
        unsafe {
            ffi::nm_setting_wireless_security_remove_proto(self.to_glib_none().0, i);
        }
    }

    fn remove_proto_by_value(&self, proto: &str) -> bool {
        unsafe {
            from_glib(ffi::nm_setting_wireless_security_remove_proto_by_value(
                self.to_glib_none().0,
                proto.to_glib_none().0,
            ))
        }
    }

    fn set_wep_key(&self, idx: u32, key: &str) {
        unsafe {
            ffi::nm_setting_wireless_security_set_wep_key(
                self.to_glib_none().0,
                idx,
                key.to_glib_none().0,
            );
        }
    }

    fn set_property_auth_alg(&self, auth_alg: Option<&str>) {
        unsafe {
            gobject_ffi::g_object_set_property(
                self.to_glib_none().0,
                "auth-alg".to_glib_none().0,
                Value::from(auth_alg).to_glib_none().0,
            );
        }
    }

    #[cfg(any(feature = "v1_12", feature = "dox"))]
    fn set_property_fils(&self, fils: i32) {
        unsafe {
            gobject_ffi::g_object_set_property(
                self.to_glib_none().0,
                "fils".to_glib_none().0,
                Value::from(&fils).to_glib_none().0,
            );
        }
    }

    fn set_property_group(&self, group: &[&str]) {
        unsafe {
            gobject_ffi::g_object_set_property(
                self.to_glib_none().0,
                "group".to_glib_none().0,
                Value::from(group).to_glib_none().0,
            );
        }
    }

    fn set_property_key_mgmt(&self, key_mgmt: Option<&str>) {
        unsafe {
            gobject_ffi::g_object_set_property(
                self.to_glib_none().0,
                "key-mgmt".to_glib_none().0,
                Value::from(key_mgmt).to_glib_none().0,
            );
        }
    }

    fn set_property_leap_password(&self, leap_password: Option<&str>) {
        unsafe {
            gobject_ffi::g_object_set_property(
                self.to_glib_none().0,
                "leap-password".to_glib_none().0,
                Value::from(leap_password).to_glib_none().0,
            );
        }
    }

    //fn set_property_leap_password_flags(&self, leap_password_flags: /*Ignored*/SettingSecretFlags) {
    //    unsafe {
    //        gobject_ffi::g_object_set_property(self.to_glib_none().0, "leap-password-flags".to_glib_none().0, Value::from(&leap_password_flags).to_glib_none().0);
    //    }
    //}

    fn set_property_leap_username(&self, leap_username: Option<&str>) {
        unsafe {
            gobject_ffi::g_object_set_property(
                self.to_glib_none().0,
                "leap-username".to_glib_none().0,
                Value::from(leap_username).to_glib_none().0,
            );
        }
    }

    fn set_property_pairwise(&self, pairwise: &[&str]) {
        unsafe {
            gobject_ffi::g_object_set_property(
                self.to_glib_none().0,
                "pairwise".to_glib_none().0,
                Value::from(pairwise).to_glib_none().0,
            );
        }
    }

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    fn set_property_pmf(&self, pmf: i32) {
        unsafe {
            gobject_ffi::g_object_set_property(
                self.to_glib_none().0,
                "pmf".to_glib_none().0,
                Value::from(&pmf).to_glib_none().0,
            );
        }
    }

    fn set_property_proto(&self, proto: &[&str]) {
        unsafe {
            gobject_ffi::g_object_set_property(
                self.to_glib_none().0,
                "proto".to_glib_none().0,
                Value::from(proto).to_glib_none().0,
            );
        }
    }

    fn set_property_psk(&self, psk: Option<&str>) {
        unsafe {
            gobject_ffi::g_object_set_property(
                self.to_glib_none().0,
                "psk".to_glib_none().0,
                Value::from(psk).to_glib_none().0,
            );
        }
    }

    //fn set_property_psk_flags(&self, psk_flags: /*Ignored*/SettingSecretFlags) {
    //    unsafe {
    //        gobject_ffi::g_object_set_property(self.to_glib_none().0, "psk-flags".to_glib_none().0, Value::from(&psk_flags).to_glib_none().0);
    //    }
    //}

    //fn set_property_wep_key_flags(&self, wep_key_flags: /*Ignored*/SettingSecretFlags) {
    //    unsafe {
    //        gobject_ffi::g_object_set_property(self.to_glib_none().0, "wep-key-flags".to_glib_none().0, Value::from(&wep_key_flags).to_glib_none().0);
    //    }
    //}

    //fn set_property_wep_key_type(&self, wep_key_type: /*Ignored*/WepKeyType) {
    //    unsafe {
    //        gobject_ffi::g_object_set_property(self.to_glib_none().0, "wep-key-type".to_glib_none().0, Value::from(&wep_key_type).to_glib_none().0);
    //    }
    //}

    fn get_property_wep_key0(&self) -> Option<String> {
        unsafe {
            let mut value = Value::from_type(<String as StaticType>::static_type());
            gobject_ffi::g_object_get_property(
                self.to_glib_none().0,
                "wep-key0".to_glib_none().0,
                value.to_glib_none_mut().0,
            );
            value.get()
        }
    }

    fn set_property_wep_key0(&self, wep_key0: Option<&str>) {
        unsafe {
            gobject_ffi::g_object_set_property(
                self.to_glib_none().0,
                "wep-key0".to_glib_none().0,
                Value::from(wep_key0).to_glib_none().0,
            );
        }
    }

    fn get_property_wep_key1(&self) -> Option<String> {
        unsafe {
            let mut value = Value::from_type(<String as StaticType>::static_type());
            gobject_ffi::g_object_get_property(
                self.to_glib_none().0,
                "wep-key1".to_glib_none().0,
                value.to_glib_none_mut().0,
            );
            value.get()
        }
    }

    fn set_property_wep_key1(&self, wep_key1: Option<&str>) {
        unsafe {
            gobject_ffi::g_object_set_property(
                self.to_glib_none().0,
                "wep-key1".to_glib_none().0,
                Value::from(wep_key1).to_glib_none().0,
            );
        }
    }

    fn get_property_wep_key2(&self) -> Option<String> {
        unsafe {
            let mut value = Value::from_type(<String as StaticType>::static_type());
            gobject_ffi::g_object_get_property(
                self.to_glib_none().0,
                "wep-key2".to_glib_none().0,
                value.to_glib_none_mut().0,
            );
            value.get()
        }
    }

    fn set_property_wep_key2(&self, wep_key2: Option<&str>) {
        unsafe {
            gobject_ffi::g_object_set_property(
                self.to_glib_none().0,
                "wep-key2".to_glib_none().0,
                Value::from(wep_key2).to_glib_none().0,
            );
        }
    }

    fn get_property_wep_key3(&self) -> Option<String> {
        unsafe {
            let mut value = Value::from_type(<String as StaticType>::static_type());
            gobject_ffi::g_object_get_property(
                self.to_glib_none().0,
                "wep-key3".to_glib_none().0,
                value.to_glib_none_mut().0,
            );
            value.get()
        }
    }

    fn set_property_wep_key3(&self, wep_key3: Option<&str>) {
        unsafe {
            gobject_ffi::g_object_set_property(
                self.to_glib_none().0,
                "wep-key3".to_glib_none().0,
                Value::from(wep_key3).to_glib_none().0,
            );
        }
    }

    fn set_property_wep_tx_keyidx(&self, wep_tx_keyidx: u32) {
        unsafe {
            gobject_ffi::g_object_set_property(
                self.to_glib_none().0,
                "wep-tx-keyidx".to_glib_none().0,
                Value::from(&wep_tx_keyidx).to_glib_none().0,
            );
        }
    }

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    fn set_property_wps_method(&self, wps_method: u32) {
        unsafe {
            gobject_ffi::g_object_set_property(
                self.to_glib_none().0,
                "wps-method".to_glib_none().0,
                Value::from(&wps_method).to_glib_none().0,
            );
        }
    }

    fn connect_property_auth_alg_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(
                self.to_glib_none().0,
                "notify::auth-alg",
                transmute(notify_auth_alg_trampoline::<Self> as usize),
                Box_::into_raw(f) as *mut _,
            )
        }
    }

    #[cfg(any(feature = "v1_12", feature = "dox"))]
    fn connect_property_fils_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(
                self.to_glib_none().0,
                "notify::fils",
                transmute(notify_fils_trampoline::<Self> as usize),
                Box_::into_raw(f) as *mut _,
            )
        }
    }

    fn connect_property_group_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(
                self.to_glib_none().0,
                "notify::group",
                transmute(notify_group_trampoline::<Self> as usize),
                Box_::into_raw(f) as *mut _,
            )
        }
    }

    fn connect_property_key_mgmt_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(
                self.to_glib_none().0,
                "notify::key-mgmt",
                transmute(notify_key_mgmt_trampoline::<Self> as usize),
                Box_::into_raw(f) as *mut _,
            )
        }
    }

    fn connect_property_leap_password_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(
                self.to_glib_none().0,
                "notify::leap-password",
                transmute(notify_leap_password_trampoline::<Self> as usize),
                Box_::into_raw(f) as *mut _,
            )
        }
    }

    fn connect_property_leap_password_flags_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(
                self.to_glib_none().0,
                "notify::leap-password-flags",
                transmute(notify_leap_password_flags_trampoline::<Self> as usize),
                Box_::into_raw(f) as *mut _,
            )
        }
    }

    fn connect_property_leap_username_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(
                self.to_glib_none().0,
                "notify::leap-username",
                transmute(notify_leap_username_trampoline::<Self> as usize),
                Box_::into_raw(f) as *mut _,
            )
        }
    }

    fn connect_property_pairwise_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(
                self.to_glib_none().0,
                "notify::pairwise",
                transmute(notify_pairwise_trampoline::<Self> as usize),
                Box_::into_raw(f) as *mut _,
            )
        }
    }

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    fn connect_property_pmf_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(
                self.to_glib_none().0,
                "notify::pmf",
                transmute(notify_pmf_trampoline::<Self> as usize),
                Box_::into_raw(f) as *mut _,
            )
        }
    }

    fn connect_property_proto_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(
                self.to_glib_none().0,
                "notify::proto",
                transmute(notify_proto_trampoline::<Self> as usize),
                Box_::into_raw(f) as *mut _,
            )
        }
    }

    fn connect_property_psk_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(
                self.to_glib_none().0,
                "notify::psk",
                transmute(notify_psk_trampoline::<Self> as usize),
                Box_::into_raw(f) as *mut _,
            )
        }
    }

    fn connect_property_psk_flags_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(
                self.to_glib_none().0,
                "notify::psk-flags",
                transmute(notify_psk_flags_trampoline::<Self> as usize),
                Box_::into_raw(f) as *mut _,
            )
        }
    }

    fn connect_property_wep_key_flags_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(
                self.to_glib_none().0,
                "notify::wep-key-flags",
                transmute(notify_wep_key_flags_trampoline::<Self> as usize),
                Box_::into_raw(f) as *mut _,
            )
        }
    }

    fn connect_property_wep_key_type_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(
                self.to_glib_none().0,
                "notify::wep-key-type",
                transmute(notify_wep_key_type_trampoline::<Self> as usize),
                Box_::into_raw(f) as *mut _,
            )
        }
    }

    fn connect_property_wep_key0_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(
                self.to_glib_none().0,
                "notify::wep-key0",
                transmute(notify_wep_key0_trampoline::<Self> as usize),
                Box_::into_raw(f) as *mut _,
            )
        }
    }

    fn connect_property_wep_key1_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(
                self.to_glib_none().0,
                "notify::wep-key1",
                transmute(notify_wep_key1_trampoline::<Self> as usize),
                Box_::into_raw(f) as *mut _,
            )
        }
    }

    fn connect_property_wep_key2_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(
                self.to_glib_none().0,
                "notify::wep-key2",
                transmute(notify_wep_key2_trampoline::<Self> as usize),
                Box_::into_raw(f) as *mut _,
            )
        }
    }

    fn connect_property_wep_key3_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(
                self.to_glib_none().0,
                "notify::wep-key3",
                transmute(notify_wep_key3_trampoline::<Self> as usize),
                Box_::into_raw(f) as *mut _,
            )
        }
    }

    fn connect_property_wep_tx_keyidx_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(
                self.to_glib_none().0,
                "notify::wep-tx-keyidx",
                transmute(notify_wep_tx_keyidx_trampoline::<Self> as usize),
                Box_::into_raw(f) as *mut _,
            )
        }
    }

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    fn connect_property_wps_method_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(
                self.to_glib_none().0,
                "notify::wps-method",
                transmute(notify_wps_method_trampoline::<Self> as usize),
                Box_::into_raw(f) as *mut _,
            )
        }
    }
}

unsafe extern "C" fn notify_auth_alg_trampoline<P>(
    this: *mut ffi::NMSettingWirelessSecurity,
    _param_spec: glib_ffi::gpointer,
    f: glib_ffi::gpointer,
) where
    P: IsA<SettingWirelessSecurity>,
{
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&SettingWirelessSecurity::from_glib_borrow(this).downcast_unchecked())
}

#[cfg(any(feature = "v1_12", feature = "dox"))]
unsafe extern "C" fn notify_fils_trampoline<P>(
    this: *mut ffi::NMSettingWirelessSecurity,
    _param_spec: glib_ffi::gpointer,
    f: glib_ffi::gpointer,
) where
    P: IsA<SettingWirelessSecurity>,
{
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&SettingWirelessSecurity::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_group_trampoline<P>(
    this: *mut ffi::NMSettingWirelessSecurity,
    _param_spec: glib_ffi::gpointer,
    f: glib_ffi::gpointer,
) where
    P: IsA<SettingWirelessSecurity>,
{
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&SettingWirelessSecurity::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_key_mgmt_trampoline<P>(
    this: *mut ffi::NMSettingWirelessSecurity,
    _param_spec: glib_ffi::gpointer,
    f: glib_ffi::gpointer,
) where
    P: IsA<SettingWirelessSecurity>,
{
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&SettingWirelessSecurity::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_leap_password_trampoline<P>(
    this: *mut ffi::NMSettingWirelessSecurity,
    _param_spec: glib_ffi::gpointer,
    f: glib_ffi::gpointer,
) where
    P: IsA<SettingWirelessSecurity>,
{
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&SettingWirelessSecurity::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_leap_password_flags_trampoline<P>(
    this: *mut ffi::NMSettingWirelessSecurity,
    _param_spec: glib_ffi::gpointer,
    f: glib_ffi::gpointer,
) where
    P: IsA<SettingWirelessSecurity>,
{
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&SettingWirelessSecurity::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_leap_username_trampoline<P>(
    this: *mut ffi::NMSettingWirelessSecurity,
    _param_spec: glib_ffi::gpointer,
    f: glib_ffi::gpointer,
) where
    P: IsA<SettingWirelessSecurity>,
{
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&SettingWirelessSecurity::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_pairwise_trampoline<P>(
    this: *mut ffi::NMSettingWirelessSecurity,
    _param_spec: glib_ffi::gpointer,
    f: glib_ffi::gpointer,
) where
    P: IsA<SettingWirelessSecurity>,
{
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&SettingWirelessSecurity::from_glib_borrow(this).downcast_unchecked())
}

#[cfg(any(feature = "v1_10", feature = "dox"))]
unsafe extern "C" fn notify_pmf_trampoline<P>(
    this: *mut ffi::NMSettingWirelessSecurity,
    _param_spec: glib_ffi::gpointer,
    f: glib_ffi::gpointer,
) where
    P: IsA<SettingWirelessSecurity>,
{
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&SettingWirelessSecurity::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_proto_trampoline<P>(
    this: *mut ffi::NMSettingWirelessSecurity,
    _param_spec: glib_ffi::gpointer,
    f: glib_ffi::gpointer,
) where
    P: IsA<SettingWirelessSecurity>,
{
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&SettingWirelessSecurity::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_psk_trampoline<P>(
    this: *mut ffi::NMSettingWirelessSecurity,
    _param_spec: glib_ffi::gpointer,
    f: glib_ffi::gpointer,
) where
    P: IsA<SettingWirelessSecurity>,
{
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&SettingWirelessSecurity::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_psk_flags_trampoline<P>(
    this: *mut ffi::NMSettingWirelessSecurity,
    _param_spec: glib_ffi::gpointer,
    f: glib_ffi::gpointer,
) where
    P: IsA<SettingWirelessSecurity>,
{
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&SettingWirelessSecurity::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_wep_key_flags_trampoline<P>(
    this: *mut ffi::NMSettingWirelessSecurity,
    _param_spec: glib_ffi::gpointer,
    f: glib_ffi::gpointer,
) where
    P: IsA<SettingWirelessSecurity>,
{
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&SettingWirelessSecurity::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_wep_key_type_trampoline<P>(
    this: *mut ffi::NMSettingWirelessSecurity,
    _param_spec: glib_ffi::gpointer,
    f: glib_ffi::gpointer,
) where
    P: IsA<SettingWirelessSecurity>,
{
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&SettingWirelessSecurity::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_wep_key0_trampoline<P>(
    this: *mut ffi::NMSettingWirelessSecurity,
    _param_spec: glib_ffi::gpointer,
    f: glib_ffi::gpointer,
) where
    P: IsA<SettingWirelessSecurity>,
{
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&SettingWirelessSecurity::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_wep_key1_trampoline<P>(
    this: *mut ffi::NMSettingWirelessSecurity,
    _param_spec: glib_ffi::gpointer,
    f: glib_ffi::gpointer,
) where
    P: IsA<SettingWirelessSecurity>,
{
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&SettingWirelessSecurity::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_wep_key2_trampoline<P>(
    this: *mut ffi::NMSettingWirelessSecurity,
    _param_spec: glib_ffi::gpointer,
    f: glib_ffi::gpointer,
) where
    P: IsA<SettingWirelessSecurity>,
{
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&SettingWirelessSecurity::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_wep_key3_trampoline<P>(
    this: *mut ffi::NMSettingWirelessSecurity,
    _param_spec: glib_ffi::gpointer,
    f: glib_ffi::gpointer,
) where
    P: IsA<SettingWirelessSecurity>,
{
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&SettingWirelessSecurity::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_wep_tx_keyidx_trampoline<P>(
    this: *mut ffi::NMSettingWirelessSecurity,
    _param_spec: glib_ffi::gpointer,
    f: glib_ffi::gpointer,
) where
    P: IsA<SettingWirelessSecurity>,
{
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&SettingWirelessSecurity::from_glib_borrow(this).downcast_unchecked())
}

#[cfg(any(feature = "v1_10", feature = "dox"))]
unsafe extern "C" fn notify_wps_method_trampoline<P>(
    this: *mut ffi::NMSettingWirelessSecurity,
    _param_spec: glib_ffi::gpointer,
    f: glib_ffi::gpointer,
) where
    P: IsA<SettingWirelessSecurity>,
{
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&SettingWirelessSecurity::from_glib_borrow(this).downcast_unchecked())
}
