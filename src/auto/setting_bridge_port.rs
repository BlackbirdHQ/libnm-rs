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
use glib::Value;
use glib_ffi;
use gobject_ffi;
use std::boxed::Box as Box_;
use std::mem;
use std::mem::transmute;
use std::ptr;
use Setting;

glib_wrapper! {
    pub struct SettingBridgePort(Object<ffi::NMSettingBridgePort, ffi::NMSettingBridgePortClass>): Setting;

    match fn {
        get_type => || ffi::nm_setting_bridge_port_get_type(),
    }
}

impl SettingBridgePort {
    pub fn new() -> SettingBridgePort {
        unsafe { Setting::from_glib_full(ffi::nm_setting_bridge_port_new()).downcast_unchecked() }
    }
}

impl Default for SettingBridgePort {
    fn default() -> Self {
        Self::new()
    }
}

pub trait SettingBridgePortExt {
    fn get_hairpin_mode(&self) -> bool;

    fn get_path_cost(&self) -> u16;

    fn get_priority(&self) -> u16;

    fn set_property_hairpin_mode(&self, hairpin_mode: bool);

    fn set_property_path_cost(&self, path_cost: u32);

    fn set_property_priority(&self, priority: u32);

    fn connect_property_hairpin_mode_notify<F: Fn(&Self) + 'static>(&self, f: F)
        -> SignalHandlerId;

    fn connect_property_path_cost_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_priority_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<SettingBridgePort> + IsA<glib::object::Object>> SettingBridgePortExt for O {
    fn get_hairpin_mode(&self) -> bool {
        unsafe {
            from_glib(ffi::nm_setting_bridge_port_get_hairpin_mode(
                self.to_glib_none().0,
            ))
        }
    }

    fn get_path_cost(&self) -> u16 {
        unsafe { ffi::nm_setting_bridge_port_get_path_cost(self.to_glib_none().0) }
    }

    fn get_priority(&self) -> u16 {
        unsafe { ffi::nm_setting_bridge_port_get_priority(self.to_glib_none().0) }
    }

    fn set_property_hairpin_mode(&self, hairpin_mode: bool) {
        unsafe {
            gobject_ffi::g_object_set_property(
                self.to_glib_none().0,
                "hairpin-mode".to_glib_none().0,
                Value::from(&hairpin_mode).to_glib_none().0,
            );
        }
    }

    fn set_property_path_cost(&self, path_cost: u32) {
        unsafe {
            gobject_ffi::g_object_set_property(
                self.to_glib_none().0,
                "path-cost".to_glib_none().0,
                Value::from(&path_cost).to_glib_none().0,
            );
        }
    }

    fn set_property_priority(&self, priority: u32) {
        unsafe {
            gobject_ffi::g_object_set_property(
                self.to_glib_none().0,
                "priority".to_glib_none().0,
                Value::from(&priority).to_glib_none().0,
            );
        }
    }

    fn connect_property_hairpin_mode_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(
                self.to_glib_none().0,
                "notify::hairpin-mode",
                transmute(notify_hairpin_mode_trampoline::<Self> as usize),
                Box_::into_raw(f) as *mut _,
            )
        }
    }

    fn connect_property_path_cost_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(
                self.to_glib_none().0,
                "notify::path-cost",
                transmute(notify_path_cost_trampoline::<Self> as usize),
                Box_::into_raw(f) as *mut _,
            )
        }
    }

    fn connect_property_priority_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(
                self.to_glib_none().0,
                "notify::priority",
                transmute(notify_priority_trampoline::<Self> as usize),
                Box_::into_raw(f) as *mut _,
            )
        }
    }
}

unsafe extern "C" fn notify_hairpin_mode_trampoline<P>(
    this: *mut ffi::NMSettingBridgePort,
    _param_spec: glib_ffi::gpointer,
    f: glib_ffi::gpointer,
) where
    P: IsA<SettingBridgePort>,
{
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&SettingBridgePort::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_path_cost_trampoline<P>(
    this: *mut ffi::NMSettingBridgePort,
    _param_spec: glib_ffi::gpointer,
    f: glib_ffi::gpointer,
) where
    P: IsA<SettingBridgePort>,
{
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&SettingBridgePort::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_priority_trampoline<P>(
    this: *mut ffi::NMSettingBridgePort,
    _param_spec: glib_ffi::gpointer,
    f: glib_ffi::gpointer,
) where
    P: IsA<SettingBridgePort>,
{
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&SettingBridgePort::from_glib_borrow(this).downcast_unchecked())
}
