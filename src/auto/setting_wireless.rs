// This file was generated by gir (https://github.com/gtk-rs/gir @ 464833e)
// from gir-files (https://github.com/gtk-rs/gir-files @ ???)
// DO NOT EDIT

use _80211ApFlags;
use _80211ApSecurityFlags;
use _80211Mode;
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
use SettingWirelessSecurity;

glib_wrapper! {
    pub struct SettingWireless(Object<ffi::NMSettingWireless, ffi::NMSettingWirelessClass>): Setting;

    match fn {
        get_type => || ffi::nm_setting_wireless_get_type(),
    }
}

impl SettingWireless {
    pub fn new() -> SettingWireless {
        unsafe { Setting::from_glib_full(ffi::nm_setting_wireless_new()).downcast_unchecked() }
    }
}

impl Default for SettingWireless {
    fn default() -> Self {
        Self::new()
    }
}

pub trait SettingWirelessExt {
    fn add_mac_blacklist_item(&self, mac: &str) -> bool;

    fn add_seen_bssid(&self, bssid: &str) -> bool;

    fn ap_security_compatible(
        &self,
        s_wireless_sec: &SettingWirelessSecurity,
        ap_flags: _80211ApFlags,
        ap_wpa: _80211ApSecurityFlags,
        ap_rsn: _80211ApSecurityFlags,
        ap_mode: _80211Mode,
    ) -> bool;

    fn clear_mac_blacklist_items(&self);

    fn get_band(&self) -> Option<String>;

    fn get_bssid(&self) -> Option<String>;

    fn get_channel(&self) -> u32;

    fn get_cloned_mac_address(&self) -> Option<String>;

    #[cfg(any(feature = "v1_4", feature = "dox"))]
    fn get_generate_mac_address_mask(&self) -> Option<String>;

    fn get_hidden(&self) -> bool;

    fn get_mac_address(&self) -> Option<String>;

    fn get_mac_address_blacklist(&self) -> Vec<String>;

    //fn get_mac_address_randomization(&self) -> /*Ignored*/SettingMacRandomization;

    fn get_mac_blacklist_item(&self, idx: u32) -> Option<String>;

    fn get_mode(&self) -> Option<String>;

    fn get_mtu(&self) -> u32;

    fn get_num_mac_blacklist_items(&self) -> u32;

    fn get_num_seen_bssids(&self) -> u32;

    fn get_powersave(&self) -> u32;

    fn get_rate(&self) -> u32;

    fn get_seen_bssid(&self, i: u32) -> Option<String>;

    fn get_ssid(&self) -> Option<glib::Bytes>;

    fn get_tx_power(&self) -> u32;

    //#[cfg(any(feature = "v1_12", feature = "dox"))]
    //fn get_wake_on_wlan(&self) -> /*Ignored*/SettingWirelessWakeOnWLan;

    fn remove_mac_blacklist_item(&self, idx: u32);

    fn remove_mac_blacklist_item_by_value(&self, mac: &str) -> bool;

    fn set_property_band(&self, band: Option<&str>);

    fn set_property_bssid(&self, bssid: Option<&str>);

    fn set_property_channel(&self, channel: u32);

    fn set_property_cloned_mac_address(&self, cloned_mac_address: Option<&str>);

    fn get_property_generate_mac_address_mask(&self) -> Option<String>;

    fn set_property_generate_mac_address_mask(&self, generate_mac_address_mask: Option<&str>);

    fn set_property_hidden(&self, hidden: bool);

    fn set_property_mac_address(&self, mac_address: Option<&str>);

    fn set_property_mac_address_blacklist(&self, mac_address_blacklist: &[&str]);

    #[cfg_attr(feature = "v1_4", deprecated)]
    fn set_property_mac_address_randomization(&self, mac_address_randomization: u32);

    fn set_property_mode(&self, mode: Option<&str>);

    fn set_property_mtu(&self, mtu: u32);

    fn set_property_powersave(&self, powersave: u32);

    fn set_property_rate(&self, rate: u32);

    fn get_property_seen_bssids(&self) -> Vec<String>;

    fn set_property_seen_bssids(&self, seen_bssids: &[&str]);

    fn set_property_ssid(&self, ssid: Option<&glib::Bytes>);

    fn set_property_tx_power(&self, tx_power: u32);

    #[cfg(any(feature = "v1_12", feature = "dox"))]
    fn set_property_wake_on_wlan(&self, wake_on_wlan: u32);

    fn connect_property_band_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_bssid_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_channel_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_cloned_mac_address_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_generate_mac_address_mask_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_hidden_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_mac_address_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_mac_address_blacklist_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    #[cfg_attr(feature = "v1_4", deprecated)]
    fn connect_property_mac_address_randomization_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_mode_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_mtu_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_powersave_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_rate_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_seen_bssids_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_ssid_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_tx_power_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v1_12", feature = "dox"))]
    fn connect_property_wake_on_wlan_notify<F: Fn(&Self) + 'static>(&self, f: F)
        -> SignalHandlerId;
}

impl<O: IsA<SettingWireless> + IsA<glib::object::Object>> SettingWirelessExt for O {
    fn add_mac_blacklist_item(&self, mac: &str) -> bool {
        unsafe {
            from_glib(ffi::nm_setting_wireless_add_mac_blacklist_item(
                self.to_glib_none().0,
                mac.to_glib_none().0,
            ))
        }
    }

    fn add_seen_bssid(&self, bssid: &str) -> bool {
        unsafe {
            from_glib(ffi::nm_setting_wireless_add_seen_bssid(
                self.to_glib_none().0,
                bssid.to_glib_none().0,
            ))
        }
    }

    fn ap_security_compatible(
        &self,
        s_wireless_sec: &SettingWirelessSecurity,
        ap_flags: _80211ApFlags,
        ap_wpa: _80211ApSecurityFlags,
        ap_rsn: _80211ApSecurityFlags,
        ap_mode: _80211Mode,
    ) -> bool {
        unsafe {
            from_glib(ffi::nm_setting_wireless_ap_security_compatible(
                self.to_glib_none().0,
                s_wireless_sec.to_glib_none().0,
                ap_flags.to_glib(),
                ap_wpa.to_glib(),
                ap_rsn.to_glib(),
                ap_mode.to_glib(),
            ))
        }
    }

    fn clear_mac_blacklist_items(&self) {
        unsafe {
            ffi::nm_setting_wireless_clear_mac_blacklist_items(self.to_glib_none().0);
        }
    }

    fn get_band(&self) -> Option<String> {
        unsafe { from_glib_none(ffi::nm_setting_wireless_get_band(self.to_glib_none().0)) }
    }

    fn get_bssid(&self) -> Option<String> {
        unsafe { from_glib_none(ffi::nm_setting_wireless_get_bssid(self.to_glib_none().0)) }
    }

    fn get_channel(&self) -> u32 {
        unsafe { ffi::nm_setting_wireless_get_channel(self.to_glib_none().0) }
    }

    fn get_cloned_mac_address(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::nm_setting_wireless_get_cloned_mac_address(
                self.to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v1_4", feature = "dox"))]
    fn get_generate_mac_address_mask(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::nm_setting_wireless_get_generate_mac_address_mask(
                self.to_glib_none().0,
            ))
        }
    }

    fn get_hidden(&self) -> bool {
        unsafe { from_glib(ffi::nm_setting_wireless_get_hidden(self.to_glib_none().0)) }
    }

    fn get_mac_address(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::nm_setting_wireless_get_mac_address(
                self.to_glib_none().0,
            ))
        }
    }

    fn get_mac_address_blacklist(&self) -> Vec<String> {
        unsafe {
            FromGlibPtrContainer::from_glib_none(
                ffi::nm_setting_wireless_get_mac_address_blacklist(self.to_glib_none().0),
            )
        }
    }

    //fn get_mac_address_randomization(&self) -> /*Ignored*/SettingMacRandomization {
    //    unsafe { TODO: call ffi::nm_setting_wireless_get_mac_address_randomization() }
    //}

    fn get_mac_blacklist_item(&self, idx: u32) -> Option<String> {
        unsafe {
            from_glib_none(ffi::nm_setting_wireless_get_mac_blacklist_item(
                self.to_glib_none().0,
                idx,
            ))
        }
    }

    fn get_mode(&self) -> Option<String> {
        unsafe { from_glib_none(ffi::nm_setting_wireless_get_mode(self.to_glib_none().0)) }
    }

    fn get_mtu(&self) -> u32 {
        unsafe { ffi::nm_setting_wireless_get_mtu(self.to_glib_none().0) }
    }

    fn get_num_mac_blacklist_items(&self) -> u32 {
        unsafe { ffi::nm_setting_wireless_get_num_mac_blacklist_items(self.to_glib_none().0) }
    }

    fn get_num_seen_bssids(&self) -> u32 {
        unsafe { ffi::nm_setting_wireless_get_num_seen_bssids(self.to_glib_none().0) }
    }

    fn get_powersave(&self) -> u32 {
        unsafe { ffi::nm_setting_wireless_get_powersave(self.to_glib_none().0) }
    }

    fn get_rate(&self) -> u32 {
        unsafe { ffi::nm_setting_wireless_get_rate(self.to_glib_none().0) }
    }

    fn get_seen_bssid(&self, i: u32) -> Option<String> {
        unsafe {
            from_glib_none(ffi::nm_setting_wireless_get_seen_bssid(
                self.to_glib_none().0,
                i,
            ))
        }
    }

    fn get_ssid(&self) -> Option<glib::Bytes> {
        unsafe { from_glib_none(ffi::nm_setting_wireless_get_ssid(self.to_glib_none().0)) }
    }

    fn get_tx_power(&self) -> u32 {
        unsafe { ffi::nm_setting_wireless_get_tx_power(self.to_glib_none().0) }
    }

    //#[cfg(any(feature = "v1_12", feature = "dox"))]
    //fn get_wake_on_wlan(&self) -> /*Ignored*/SettingWirelessWakeOnWLan {
    //    unsafe { TODO: call ffi::nm_setting_wireless_get_wake_on_wlan() }
    //}

    fn remove_mac_blacklist_item(&self, idx: u32) {
        unsafe {
            ffi::nm_setting_wireless_remove_mac_blacklist_item(self.to_glib_none().0, idx);
        }
    }

    fn remove_mac_blacklist_item_by_value(&self, mac: &str) -> bool {
        unsafe {
            from_glib(ffi::nm_setting_wireless_remove_mac_blacklist_item_by_value(
                self.to_glib_none().0,
                mac.to_glib_none().0,
            ))
        }
    }

    fn set_property_band(&self, band: Option<&str>) {
        unsafe {
            gobject_ffi::g_object_set_property(
                self.to_glib_none().0,
                "band".to_glib_none().0,
                Value::from(band).to_glib_none().0,
            );
        }
    }

    fn set_property_bssid(&self, bssid: Option<&str>) {
        unsafe {
            gobject_ffi::g_object_set_property(
                self.to_glib_none().0,
                "bssid".to_glib_none().0,
                Value::from(bssid).to_glib_none().0,
            );
        }
    }

    fn set_property_channel(&self, channel: u32) {
        unsafe {
            gobject_ffi::g_object_set_property(
                self.to_glib_none().0,
                "channel".to_glib_none().0,
                Value::from(&channel).to_glib_none().0,
            );
        }
    }

    fn set_property_cloned_mac_address(&self, cloned_mac_address: Option<&str>) {
        unsafe {
            gobject_ffi::g_object_set_property(
                self.to_glib_none().0,
                "cloned-mac-address".to_glib_none().0,
                Value::from(cloned_mac_address).to_glib_none().0,
            );
        }
    }

    fn get_property_generate_mac_address_mask(&self) -> Option<String> {
        unsafe {
            let mut value = Value::from_type(<String as StaticType>::static_type());
            gobject_ffi::g_object_get_property(
                self.to_glib_none().0,
                "generate-mac-address-mask".to_glib_none().0,
                value.to_glib_none_mut().0,
            );
            value.get()
        }
    }

    fn set_property_generate_mac_address_mask(&self, generate_mac_address_mask: Option<&str>) {
        unsafe {
            gobject_ffi::g_object_set_property(
                self.to_glib_none().0,
                "generate-mac-address-mask".to_glib_none().0,
                Value::from(generate_mac_address_mask).to_glib_none().0,
            );
        }
    }

    fn set_property_hidden(&self, hidden: bool) {
        unsafe {
            gobject_ffi::g_object_set_property(
                self.to_glib_none().0,
                "hidden".to_glib_none().0,
                Value::from(&hidden).to_glib_none().0,
            );
        }
    }

    fn set_property_mac_address(&self, mac_address: Option<&str>) {
        unsafe {
            gobject_ffi::g_object_set_property(
                self.to_glib_none().0,
                "mac-address".to_glib_none().0,
                Value::from(mac_address).to_glib_none().0,
            );
        }
    }

    fn set_property_mac_address_blacklist(&self, mac_address_blacklist: &[&str]) {
        unsafe {
            gobject_ffi::g_object_set_property(
                self.to_glib_none().0,
                "mac-address-blacklist".to_glib_none().0,
                Value::from(mac_address_blacklist).to_glib_none().0,
            );
        }
    }

    fn set_property_mac_address_randomization(&self, mac_address_randomization: u32) {
        unsafe {
            gobject_ffi::g_object_set_property(
                self.to_glib_none().0,
                "mac-address-randomization".to_glib_none().0,
                Value::from(&mac_address_randomization).to_glib_none().0,
            );
        }
    }

    fn set_property_mode(&self, mode: Option<&str>) {
        unsafe {
            gobject_ffi::g_object_set_property(
                self.to_glib_none().0,
                "mode".to_glib_none().0,
                Value::from(mode).to_glib_none().0,
            );
        }
    }

    fn set_property_mtu(&self, mtu: u32) {
        unsafe {
            gobject_ffi::g_object_set_property(
                self.to_glib_none().0,
                "mtu".to_glib_none().0,
                Value::from(&mtu).to_glib_none().0,
            );
        }
    }

    fn set_property_powersave(&self, powersave: u32) {
        unsafe {
            gobject_ffi::g_object_set_property(
                self.to_glib_none().0,
                "powersave".to_glib_none().0,
                Value::from(&powersave).to_glib_none().0,
            );
        }
    }

    fn set_property_rate(&self, rate: u32) {
        unsafe {
            gobject_ffi::g_object_set_property(
                self.to_glib_none().0,
                "rate".to_glib_none().0,
                Value::from(&rate).to_glib_none().0,
            );
        }
    }

    fn get_property_seen_bssids(&self) -> Vec<String> {
        unsafe {
            let mut value = Value::from_type(<Vec<String> as StaticType>::static_type());
            gobject_ffi::g_object_get_property(
                self.to_glib_none().0,
                "seen-bssids".to_glib_none().0,
                value.to_glib_none_mut().0,
            );
            value.get().unwrap()
        }
    }

    fn set_property_seen_bssids(&self, seen_bssids: &[&str]) {
        unsafe {
            gobject_ffi::g_object_set_property(
                self.to_glib_none().0,
                "seen-bssids".to_glib_none().0,
                Value::from(seen_bssids).to_glib_none().0,
            );
        }
    }

    fn set_property_ssid(&self, ssid: Option<&glib::Bytes>) {
        unsafe {
            gobject_ffi::g_object_set_property(
                self.to_glib_none().0,
                "ssid".to_glib_none().0,
                Value::from(ssid).to_glib_none().0,
            );
        }
    }

    fn set_property_tx_power(&self, tx_power: u32) {
        unsafe {
            gobject_ffi::g_object_set_property(
                self.to_glib_none().0,
                "tx-power".to_glib_none().0,
                Value::from(&tx_power).to_glib_none().0,
            );
        }
    }

    #[cfg(any(feature = "v1_12", feature = "dox"))]
    fn set_property_wake_on_wlan(&self, wake_on_wlan: u32) {
        unsafe {
            gobject_ffi::g_object_set_property(
                self.to_glib_none().0,
                "wake-on-wlan".to_glib_none().0,
                Value::from(&wake_on_wlan).to_glib_none().0,
            );
        }
    }

    fn connect_property_band_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(
                self.to_glib_none().0,
                "notify::band",
                transmute(notify_band_trampoline::<Self> as usize),
                Box_::into_raw(f) as *mut _,
            )
        }
    }

    fn connect_property_bssid_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(
                self.to_glib_none().0,
                "notify::bssid",
                transmute(notify_bssid_trampoline::<Self> as usize),
                Box_::into_raw(f) as *mut _,
            )
        }
    }

    fn connect_property_channel_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(
                self.to_glib_none().0,
                "notify::channel",
                transmute(notify_channel_trampoline::<Self> as usize),
                Box_::into_raw(f) as *mut _,
            )
        }
    }

    fn connect_property_cloned_mac_address_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(
                self.to_glib_none().0,
                "notify::cloned-mac-address",
                transmute(notify_cloned_mac_address_trampoline::<Self> as usize),
                Box_::into_raw(f) as *mut _,
            )
        }
    }

    fn connect_property_generate_mac_address_mask_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(
                self.to_glib_none().0,
                "notify::generate-mac-address-mask",
                transmute(notify_generate_mac_address_mask_trampoline::<Self> as usize),
                Box_::into_raw(f) as *mut _,
            )
        }
    }

    fn connect_property_hidden_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(
                self.to_glib_none().0,
                "notify::hidden",
                transmute(notify_hidden_trampoline::<Self> as usize),
                Box_::into_raw(f) as *mut _,
            )
        }
    }

    fn connect_property_mac_address_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(
                self.to_glib_none().0,
                "notify::mac-address",
                transmute(notify_mac_address_trampoline::<Self> as usize),
                Box_::into_raw(f) as *mut _,
            )
        }
    }

    fn connect_property_mac_address_blacklist_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(
                self.to_glib_none().0,
                "notify::mac-address-blacklist",
                transmute(notify_mac_address_blacklist_trampoline::<Self> as usize),
                Box_::into_raw(f) as *mut _,
            )
        }
    }

    fn connect_property_mac_address_randomization_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(
                self.to_glib_none().0,
                "notify::mac-address-randomization",
                transmute(notify_mac_address_randomization_trampoline::<Self> as usize),
                Box_::into_raw(f) as *mut _,
            )
        }
    }

    fn connect_property_mode_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(
                self.to_glib_none().0,
                "notify::mode",
                transmute(notify_mode_trampoline::<Self> as usize),
                Box_::into_raw(f) as *mut _,
            )
        }
    }

    fn connect_property_mtu_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(
                self.to_glib_none().0,
                "notify::mtu",
                transmute(notify_mtu_trampoline::<Self> as usize),
                Box_::into_raw(f) as *mut _,
            )
        }
    }

    fn connect_property_powersave_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(
                self.to_glib_none().0,
                "notify::powersave",
                transmute(notify_powersave_trampoline::<Self> as usize),
                Box_::into_raw(f) as *mut _,
            )
        }
    }

    fn connect_property_rate_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(
                self.to_glib_none().0,
                "notify::rate",
                transmute(notify_rate_trampoline::<Self> as usize),
                Box_::into_raw(f) as *mut _,
            )
        }
    }

    fn connect_property_seen_bssids_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(
                self.to_glib_none().0,
                "notify::seen-bssids",
                transmute(notify_seen_bssids_trampoline::<Self> as usize),
                Box_::into_raw(f) as *mut _,
            )
        }
    }

    fn connect_property_ssid_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(
                self.to_glib_none().0,
                "notify::ssid",
                transmute(notify_ssid_trampoline::<Self> as usize),
                Box_::into_raw(f) as *mut _,
            )
        }
    }

    fn connect_property_tx_power_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(
                self.to_glib_none().0,
                "notify::tx-power",
                transmute(notify_tx_power_trampoline::<Self> as usize),
                Box_::into_raw(f) as *mut _,
            )
        }
    }

    #[cfg(any(feature = "v1_12", feature = "dox"))]
    fn connect_property_wake_on_wlan_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(
                self.to_glib_none().0,
                "notify::wake-on-wlan",
                transmute(notify_wake_on_wlan_trampoline::<Self> as usize),
                Box_::into_raw(f) as *mut _,
            )
        }
    }
}

unsafe extern "C" fn notify_band_trampoline<P>(
    this: *mut ffi::NMSettingWireless,
    _param_spec: glib_ffi::gpointer,
    f: glib_ffi::gpointer,
) where
    P: IsA<SettingWireless>,
{
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&SettingWireless::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_bssid_trampoline<P>(
    this: *mut ffi::NMSettingWireless,
    _param_spec: glib_ffi::gpointer,
    f: glib_ffi::gpointer,
) where
    P: IsA<SettingWireless>,
{
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&SettingWireless::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_channel_trampoline<P>(
    this: *mut ffi::NMSettingWireless,
    _param_spec: glib_ffi::gpointer,
    f: glib_ffi::gpointer,
) where
    P: IsA<SettingWireless>,
{
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&SettingWireless::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_cloned_mac_address_trampoline<P>(
    this: *mut ffi::NMSettingWireless,
    _param_spec: glib_ffi::gpointer,
    f: glib_ffi::gpointer,
) where
    P: IsA<SettingWireless>,
{
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&SettingWireless::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_generate_mac_address_mask_trampoline<P>(
    this: *mut ffi::NMSettingWireless,
    _param_spec: glib_ffi::gpointer,
    f: glib_ffi::gpointer,
) where
    P: IsA<SettingWireless>,
{
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&SettingWireless::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_hidden_trampoline<P>(
    this: *mut ffi::NMSettingWireless,
    _param_spec: glib_ffi::gpointer,
    f: glib_ffi::gpointer,
) where
    P: IsA<SettingWireless>,
{
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&SettingWireless::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_mac_address_trampoline<P>(
    this: *mut ffi::NMSettingWireless,
    _param_spec: glib_ffi::gpointer,
    f: glib_ffi::gpointer,
) where
    P: IsA<SettingWireless>,
{
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&SettingWireless::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_mac_address_blacklist_trampoline<P>(
    this: *mut ffi::NMSettingWireless,
    _param_spec: glib_ffi::gpointer,
    f: glib_ffi::gpointer,
) where
    P: IsA<SettingWireless>,
{
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&SettingWireless::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_mac_address_randomization_trampoline<P>(
    this: *mut ffi::NMSettingWireless,
    _param_spec: glib_ffi::gpointer,
    f: glib_ffi::gpointer,
) where
    P: IsA<SettingWireless>,
{
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&SettingWireless::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_mode_trampoline<P>(
    this: *mut ffi::NMSettingWireless,
    _param_spec: glib_ffi::gpointer,
    f: glib_ffi::gpointer,
) where
    P: IsA<SettingWireless>,
{
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&SettingWireless::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_mtu_trampoline<P>(
    this: *mut ffi::NMSettingWireless,
    _param_spec: glib_ffi::gpointer,
    f: glib_ffi::gpointer,
) where
    P: IsA<SettingWireless>,
{
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&SettingWireless::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_powersave_trampoline<P>(
    this: *mut ffi::NMSettingWireless,
    _param_spec: glib_ffi::gpointer,
    f: glib_ffi::gpointer,
) where
    P: IsA<SettingWireless>,
{
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&SettingWireless::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_rate_trampoline<P>(
    this: *mut ffi::NMSettingWireless,
    _param_spec: glib_ffi::gpointer,
    f: glib_ffi::gpointer,
) where
    P: IsA<SettingWireless>,
{
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&SettingWireless::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_seen_bssids_trampoline<P>(
    this: *mut ffi::NMSettingWireless,
    _param_spec: glib_ffi::gpointer,
    f: glib_ffi::gpointer,
) where
    P: IsA<SettingWireless>,
{
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&SettingWireless::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_ssid_trampoline<P>(
    this: *mut ffi::NMSettingWireless,
    _param_spec: glib_ffi::gpointer,
    f: glib_ffi::gpointer,
) where
    P: IsA<SettingWireless>,
{
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&SettingWireless::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_tx_power_trampoline<P>(
    this: *mut ffi::NMSettingWireless,
    _param_spec: glib_ffi::gpointer,
    f: glib_ffi::gpointer,
) where
    P: IsA<SettingWireless>,
{
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&SettingWireless::from_glib_borrow(this).downcast_unchecked())
}

#[cfg(any(feature = "v1_12", feature = "dox"))]
unsafe extern "C" fn notify_wake_on_wlan_trampoline<P>(
    this: *mut ffi::NMSettingWireless,
    _param_spec: glib_ffi::gpointer,
    f: glib_ffi::gpointer,
) where
    P: IsA<SettingWireless>,
{
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&SettingWireless::from_glib_borrow(this).downcast_unchecked())
}
