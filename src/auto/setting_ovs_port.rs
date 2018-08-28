// This file was generated by gir (https://github.com/gtk-rs/gir @ 464833e)
// from gir-files (https://github.com/gtk-rs/gir-files @ ???)
// DO NOT EDIT

use ffi;
use glib;
use glib::object::Downcast;
use glib::object::IsA;
#[cfg(any(feature = "v1_10", feature = "dox"))]
use glib::signal::connect;
#[cfg(any(feature = "v1_10", feature = "dox"))]
use glib::signal::SignalHandlerId;
use glib::translate::*;
#[cfg(any(feature = "v1_10", feature = "dox"))]
use glib::Value;
use glib_ffi;
use gobject_ffi;
#[cfg(any(feature = "v1_10", feature = "dox"))]
use std::boxed::Box as Box_;
use std::mem;
#[cfg(any(feature = "v1_10", feature = "dox"))]
use std::mem::transmute;
use std::ptr;
use Setting;

glib_wrapper! {
    pub struct SettingOvsPort(Object<ffi::NMSettingOvsPort, ffi::NMSettingOvsPortClass>): Setting;

    match fn {
        get_type => || ffi::nm_setting_ovs_port_get_type(),
    }
}

impl SettingOvsPort {
    #[cfg(any(feature = "v1_10", feature = "dox"))]
    pub fn new() -> SettingOvsPort {
        unsafe { Setting::from_glib_full(ffi::nm_setting_ovs_port_new()).downcast_unchecked() }
    }
}

#[cfg(any(feature = "v1_10", feature = "dox"))]
impl Default for SettingOvsPort {
    fn default() -> Self {
        Self::new()
    }
}

pub trait SettingOvsPortExt {
    #[cfg(any(feature = "v1_10", feature = "dox"))]
    fn get_bond_downdelay(&self) -> u32;

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    fn get_bond_mode(&self) -> Option<String>;

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    fn get_bond_updelay(&self) -> u32;

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    fn get_lacp(&self) -> Option<String>;

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    fn get_tag(&self) -> u32;

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    fn get_vlan_mode(&self) -> Option<String>;

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    fn set_property_bond_downdelay(&self, bond_downdelay: u32);

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    fn set_property_bond_mode(&self, bond_mode: Option<&str>);

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    fn set_property_bond_updelay(&self, bond_updelay: u32);

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    fn set_property_lacp(&self, lacp: Option<&str>);

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    fn set_property_tag(&self, tag: u32);

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    fn set_property_vlan_mode(&self, vlan_mode: Option<&str>);

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    fn connect_property_bond_downdelay_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    fn connect_property_bond_mode_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    fn connect_property_bond_updelay_notify<F: Fn(&Self) + 'static>(&self, f: F)
        -> SignalHandlerId;

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    fn connect_property_lacp_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    fn connect_property_tag_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    fn connect_property_vlan_mode_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<SettingOvsPort> + IsA<glib::object::Object>> SettingOvsPortExt for O {
    #[cfg(any(feature = "v1_10", feature = "dox"))]
    fn get_bond_downdelay(&self) -> u32 {
        unsafe { ffi::nm_setting_ovs_port_get_bond_downdelay(self.to_glib_none().0) }
    }

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    fn get_bond_mode(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::nm_setting_ovs_port_get_bond_mode(
                self.to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    fn get_bond_updelay(&self) -> u32 {
        unsafe { ffi::nm_setting_ovs_port_get_bond_updelay(self.to_glib_none().0) }
    }

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    fn get_lacp(&self) -> Option<String> {
        unsafe { from_glib_none(ffi::nm_setting_ovs_port_get_lacp(self.to_glib_none().0)) }
    }

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    fn get_tag(&self) -> u32 {
        unsafe { ffi::nm_setting_ovs_port_get_tag(self.to_glib_none().0) }
    }

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    fn get_vlan_mode(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::nm_setting_ovs_port_get_vlan_mode(
                self.to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    fn set_property_bond_downdelay(&self, bond_downdelay: u32) {
        unsafe {
            gobject_ffi::g_object_set_property(
                self.to_glib_none().0,
                "bond-downdelay".to_glib_none().0,
                Value::from(&bond_downdelay).to_glib_none().0,
            );
        }
    }

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    fn set_property_bond_mode(&self, bond_mode: Option<&str>) {
        unsafe {
            gobject_ffi::g_object_set_property(
                self.to_glib_none().0,
                "bond-mode".to_glib_none().0,
                Value::from(bond_mode).to_glib_none().0,
            );
        }
    }

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    fn set_property_bond_updelay(&self, bond_updelay: u32) {
        unsafe {
            gobject_ffi::g_object_set_property(
                self.to_glib_none().0,
                "bond-updelay".to_glib_none().0,
                Value::from(&bond_updelay).to_glib_none().0,
            );
        }
    }

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    fn set_property_lacp(&self, lacp: Option<&str>) {
        unsafe {
            gobject_ffi::g_object_set_property(
                self.to_glib_none().0,
                "lacp".to_glib_none().0,
                Value::from(lacp).to_glib_none().0,
            );
        }
    }

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    fn set_property_tag(&self, tag: u32) {
        unsafe {
            gobject_ffi::g_object_set_property(
                self.to_glib_none().0,
                "tag".to_glib_none().0,
                Value::from(&tag).to_glib_none().0,
            );
        }
    }

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    fn set_property_vlan_mode(&self, vlan_mode: Option<&str>) {
        unsafe {
            gobject_ffi::g_object_set_property(
                self.to_glib_none().0,
                "vlan-mode".to_glib_none().0,
                Value::from(vlan_mode).to_glib_none().0,
            );
        }
    }

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    fn connect_property_bond_downdelay_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(
                self.to_glib_none().0,
                "notify::bond-downdelay",
                transmute(notify_bond_downdelay_trampoline::<Self> as usize),
                Box_::into_raw(f) as *mut _,
            )
        }
    }

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    fn connect_property_bond_mode_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(
                self.to_glib_none().0,
                "notify::bond-mode",
                transmute(notify_bond_mode_trampoline::<Self> as usize),
                Box_::into_raw(f) as *mut _,
            )
        }
    }

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    fn connect_property_bond_updelay_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(
                self.to_glib_none().0,
                "notify::bond-updelay",
                transmute(notify_bond_updelay_trampoline::<Self> as usize),
                Box_::into_raw(f) as *mut _,
            )
        }
    }

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    fn connect_property_lacp_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(
                self.to_glib_none().0,
                "notify::lacp",
                transmute(notify_lacp_trampoline::<Self> as usize),
                Box_::into_raw(f) as *mut _,
            )
        }
    }

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    fn connect_property_tag_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(
                self.to_glib_none().0,
                "notify::tag",
                transmute(notify_tag_trampoline::<Self> as usize),
                Box_::into_raw(f) as *mut _,
            )
        }
    }

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    fn connect_property_vlan_mode_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(
                self.to_glib_none().0,
                "notify::vlan-mode",
                transmute(notify_vlan_mode_trampoline::<Self> as usize),
                Box_::into_raw(f) as *mut _,
            )
        }
    }
}

#[cfg(any(feature = "v1_10", feature = "dox"))]
unsafe extern "C" fn notify_bond_downdelay_trampoline<P>(
    this: *mut ffi::NMSettingOvsPort,
    _param_spec: glib_ffi::gpointer,
    f: glib_ffi::gpointer,
) where
    P: IsA<SettingOvsPort>,
{
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&SettingOvsPort::from_glib_borrow(this).downcast_unchecked())
}

#[cfg(any(feature = "v1_10", feature = "dox"))]
unsafe extern "C" fn notify_bond_mode_trampoline<P>(
    this: *mut ffi::NMSettingOvsPort,
    _param_spec: glib_ffi::gpointer,
    f: glib_ffi::gpointer,
) where
    P: IsA<SettingOvsPort>,
{
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&SettingOvsPort::from_glib_borrow(this).downcast_unchecked())
}

#[cfg(any(feature = "v1_10", feature = "dox"))]
unsafe extern "C" fn notify_bond_updelay_trampoline<P>(
    this: *mut ffi::NMSettingOvsPort,
    _param_spec: glib_ffi::gpointer,
    f: glib_ffi::gpointer,
) where
    P: IsA<SettingOvsPort>,
{
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&SettingOvsPort::from_glib_borrow(this).downcast_unchecked())
}

#[cfg(any(feature = "v1_10", feature = "dox"))]
unsafe extern "C" fn notify_lacp_trampoline<P>(
    this: *mut ffi::NMSettingOvsPort,
    _param_spec: glib_ffi::gpointer,
    f: glib_ffi::gpointer,
) where
    P: IsA<SettingOvsPort>,
{
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&SettingOvsPort::from_glib_borrow(this).downcast_unchecked())
}

#[cfg(any(feature = "v1_10", feature = "dox"))]
unsafe extern "C" fn notify_tag_trampoline<P>(
    this: *mut ffi::NMSettingOvsPort,
    _param_spec: glib_ffi::gpointer,
    f: glib_ffi::gpointer,
) where
    P: IsA<SettingOvsPort>,
{
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&SettingOvsPort::from_glib_borrow(this).downcast_unchecked())
}

#[cfg(any(feature = "v1_10", feature = "dox"))]
unsafe extern "C" fn notify_vlan_mode_trampoline<P>(
    this: *mut ffi::NMSettingOvsPort,
    _param_spec: glib_ffi::gpointer,
    f: glib_ffi::gpointer,
) where
    P: IsA<SettingOvsPort>,
{
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&SettingOvsPort::from_glib_borrow(this).downcast_unchecked())
}
