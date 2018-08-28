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
use glib::StaticType;
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
    pub struct SettingOvsInterface(Object<ffi::NMSettingOvsInterface, ffi::NMSettingOvsInterfaceClass>): Setting;

    match fn {
        get_type => || ffi::nm_setting_ovs_interface_get_type(),
    }
}

impl SettingOvsInterface {
    #[cfg(any(feature = "v1_10", feature = "dox"))]
    pub fn new() -> SettingOvsInterface {
        unsafe { Setting::from_glib_full(ffi::nm_setting_ovs_interface_new()).downcast_unchecked() }
    }
}

#[cfg(any(feature = "v1_10", feature = "dox"))]
impl Default for SettingOvsInterface {
    fn default() -> Self {
        Self::new()
    }
}

pub trait SettingOvsInterfaceExt {
    #[cfg(any(feature = "v1_10", feature = "dox"))]
    fn get_interface_type(&self) -> Option<String>;

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    fn get_property_type(&self) -> Option<String>;

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    fn set_property_type(&self, type_: Option<&str>);

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    fn connect_property_type_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<SettingOvsInterface> + IsA<glib::object::Object>> SettingOvsInterfaceExt for O {
    #[cfg(any(feature = "v1_10", feature = "dox"))]
    fn get_interface_type(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::nm_setting_ovs_interface_get_interface_type(
                self.to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    fn get_property_type(&self) -> Option<String> {
        unsafe {
            let mut value = Value::from_type(<String as StaticType>::static_type());
            gobject_ffi::g_object_get_property(
                self.to_glib_none().0,
                "type".to_glib_none().0,
                value.to_glib_none_mut().0,
            );
            value.get()
        }
    }

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    fn set_property_type(&self, type_: Option<&str>) {
        unsafe {
            gobject_ffi::g_object_set_property(
                self.to_glib_none().0,
                "type".to_glib_none().0,
                Value::from(type_).to_glib_none().0,
            );
        }
    }

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    fn connect_property_type_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(
                self.to_glib_none().0,
                "notify::type",
                transmute(notify_type_trampoline::<Self> as usize),
                Box_::into_raw(f) as *mut _,
            )
        }
    }
}

#[cfg(any(feature = "v1_10", feature = "dox"))]
unsafe extern "C" fn notify_type_trampoline<P>(
    this: *mut ffi::NMSettingOvsInterface,
    _param_spec: glib_ffi::gpointer,
    f: glib_ffi::gpointer,
) where
    P: IsA<SettingOvsInterface>,
{
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&SettingOvsInterface::from_glib_borrow(this).downcast_unchecked())
}
