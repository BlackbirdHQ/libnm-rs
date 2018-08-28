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
    pub struct SettingPpp(Object<ffi::NMSettingPpp, ffi::NMSettingPppClass>): Setting;

    match fn {
        get_type => || ffi::nm_setting_ppp_get_type(),
    }
}

impl SettingPpp {
    pub fn new() -> SettingPpp {
        unsafe { Setting::from_glib_full(ffi::nm_setting_ppp_new()).downcast_unchecked() }
    }
}

impl Default for SettingPpp {
    fn default() -> Self {
        Self::new()
    }
}

pub trait SettingPppExt {
    fn get_baud(&self) -> u32;

    fn get_crtscts(&self) -> bool;

    fn get_lcp_echo_failure(&self) -> u32;

    fn get_lcp_echo_interval(&self) -> u32;

    fn get_mppe_stateful(&self) -> bool;

    fn get_mru(&self) -> u32;

    fn get_mtu(&self) -> u32;

    fn get_no_vj_comp(&self) -> bool;

    fn get_noauth(&self) -> bool;

    fn get_nobsdcomp(&self) -> bool;

    fn get_nodeflate(&self) -> bool;

    fn get_refuse_chap(&self) -> bool;

    fn get_refuse_eap(&self) -> bool;

    fn get_refuse_mschap(&self) -> bool;

    fn get_refuse_mschapv2(&self) -> bool;

    fn get_refuse_pap(&self) -> bool;

    fn get_require_mppe(&self) -> bool;

    fn get_require_mppe_128(&self) -> bool;

    fn set_property_baud(&self, baud: u32);

    fn set_property_crtscts(&self, crtscts: bool);

    fn set_property_lcp_echo_failure(&self, lcp_echo_failure: u32);

    fn set_property_lcp_echo_interval(&self, lcp_echo_interval: u32);

    fn set_property_mppe_stateful(&self, mppe_stateful: bool);

    fn set_property_mru(&self, mru: u32);

    fn set_property_mtu(&self, mtu: u32);

    fn set_property_no_vj_comp(&self, no_vj_comp: bool);

    fn set_property_noauth(&self, noauth: bool);

    fn set_property_nobsdcomp(&self, nobsdcomp: bool);

    fn set_property_nodeflate(&self, nodeflate: bool);

    fn set_property_refuse_chap(&self, refuse_chap: bool);

    fn set_property_refuse_eap(&self, refuse_eap: bool);

    fn set_property_refuse_mschap(&self, refuse_mschap: bool);

    fn set_property_refuse_mschapv2(&self, refuse_mschapv2: bool);

    fn set_property_refuse_pap(&self, refuse_pap: bool);

    fn set_property_require_mppe(&self, require_mppe: bool);

    fn set_property_require_mppe_128(&self, require_mppe_128: bool);

    fn connect_property_baud_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_crtscts_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_lcp_echo_failure_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_lcp_echo_interval_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_mppe_stateful_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_mru_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_mtu_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_no_vj_comp_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_noauth_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_nobsdcomp_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_nodeflate_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_refuse_chap_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_refuse_eap_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_refuse_mschap_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_refuse_mschapv2_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_refuse_pap_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_require_mppe_notify<F: Fn(&Self) + 'static>(&self, f: F)
        -> SignalHandlerId;

    fn connect_property_require_mppe_128_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;
}

impl<O: IsA<SettingPpp> + IsA<glib::object::Object>> SettingPppExt for O {
    fn get_baud(&self) -> u32 {
        unsafe { ffi::nm_setting_ppp_get_baud(self.to_glib_none().0) }
    }

    fn get_crtscts(&self) -> bool {
        unsafe { from_glib(ffi::nm_setting_ppp_get_crtscts(self.to_glib_none().0)) }
    }

    fn get_lcp_echo_failure(&self) -> u32 {
        unsafe { ffi::nm_setting_ppp_get_lcp_echo_failure(self.to_glib_none().0) }
    }

    fn get_lcp_echo_interval(&self) -> u32 {
        unsafe { ffi::nm_setting_ppp_get_lcp_echo_interval(self.to_glib_none().0) }
    }

    fn get_mppe_stateful(&self) -> bool {
        unsafe { from_glib(ffi::nm_setting_ppp_get_mppe_stateful(self.to_glib_none().0)) }
    }

    fn get_mru(&self) -> u32 {
        unsafe { ffi::nm_setting_ppp_get_mru(self.to_glib_none().0) }
    }

    fn get_mtu(&self) -> u32 {
        unsafe { ffi::nm_setting_ppp_get_mtu(self.to_glib_none().0) }
    }

    fn get_no_vj_comp(&self) -> bool {
        unsafe { from_glib(ffi::nm_setting_ppp_get_no_vj_comp(self.to_glib_none().0)) }
    }

    fn get_noauth(&self) -> bool {
        unsafe { from_glib(ffi::nm_setting_ppp_get_noauth(self.to_glib_none().0)) }
    }

    fn get_nobsdcomp(&self) -> bool {
        unsafe { from_glib(ffi::nm_setting_ppp_get_nobsdcomp(self.to_glib_none().0)) }
    }

    fn get_nodeflate(&self) -> bool {
        unsafe { from_glib(ffi::nm_setting_ppp_get_nodeflate(self.to_glib_none().0)) }
    }

    fn get_refuse_chap(&self) -> bool {
        unsafe { from_glib(ffi::nm_setting_ppp_get_refuse_chap(self.to_glib_none().0)) }
    }

    fn get_refuse_eap(&self) -> bool {
        unsafe { from_glib(ffi::nm_setting_ppp_get_refuse_eap(self.to_glib_none().0)) }
    }

    fn get_refuse_mschap(&self) -> bool {
        unsafe { from_glib(ffi::nm_setting_ppp_get_refuse_mschap(self.to_glib_none().0)) }
    }

    fn get_refuse_mschapv2(&self) -> bool {
        unsafe {
            from_glib(ffi::nm_setting_ppp_get_refuse_mschapv2(
                self.to_glib_none().0,
            ))
        }
    }

    fn get_refuse_pap(&self) -> bool {
        unsafe { from_glib(ffi::nm_setting_ppp_get_refuse_pap(self.to_glib_none().0)) }
    }

    fn get_require_mppe(&self) -> bool {
        unsafe { from_glib(ffi::nm_setting_ppp_get_require_mppe(self.to_glib_none().0)) }
    }

    fn get_require_mppe_128(&self) -> bool {
        unsafe {
            from_glib(ffi::nm_setting_ppp_get_require_mppe_128(
                self.to_glib_none().0,
            ))
        }
    }

    fn set_property_baud(&self, baud: u32) {
        unsafe {
            gobject_ffi::g_object_set_property(
                self.to_glib_none().0,
                "baud".to_glib_none().0,
                Value::from(&baud).to_glib_none().0,
            );
        }
    }

    fn set_property_crtscts(&self, crtscts: bool) {
        unsafe {
            gobject_ffi::g_object_set_property(
                self.to_glib_none().0,
                "crtscts".to_glib_none().0,
                Value::from(&crtscts).to_glib_none().0,
            );
        }
    }

    fn set_property_lcp_echo_failure(&self, lcp_echo_failure: u32) {
        unsafe {
            gobject_ffi::g_object_set_property(
                self.to_glib_none().0,
                "lcp-echo-failure".to_glib_none().0,
                Value::from(&lcp_echo_failure).to_glib_none().0,
            );
        }
    }

    fn set_property_lcp_echo_interval(&self, lcp_echo_interval: u32) {
        unsafe {
            gobject_ffi::g_object_set_property(
                self.to_glib_none().0,
                "lcp-echo-interval".to_glib_none().0,
                Value::from(&lcp_echo_interval).to_glib_none().0,
            );
        }
    }

    fn set_property_mppe_stateful(&self, mppe_stateful: bool) {
        unsafe {
            gobject_ffi::g_object_set_property(
                self.to_glib_none().0,
                "mppe-stateful".to_glib_none().0,
                Value::from(&mppe_stateful).to_glib_none().0,
            );
        }
    }

    fn set_property_mru(&self, mru: u32) {
        unsafe {
            gobject_ffi::g_object_set_property(
                self.to_glib_none().0,
                "mru".to_glib_none().0,
                Value::from(&mru).to_glib_none().0,
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

    fn set_property_no_vj_comp(&self, no_vj_comp: bool) {
        unsafe {
            gobject_ffi::g_object_set_property(
                self.to_glib_none().0,
                "no-vj-comp".to_glib_none().0,
                Value::from(&no_vj_comp).to_glib_none().0,
            );
        }
    }

    fn set_property_noauth(&self, noauth: bool) {
        unsafe {
            gobject_ffi::g_object_set_property(
                self.to_glib_none().0,
                "noauth".to_glib_none().0,
                Value::from(&noauth).to_glib_none().0,
            );
        }
    }

    fn set_property_nobsdcomp(&self, nobsdcomp: bool) {
        unsafe {
            gobject_ffi::g_object_set_property(
                self.to_glib_none().0,
                "nobsdcomp".to_glib_none().0,
                Value::from(&nobsdcomp).to_glib_none().0,
            );
        }
    }

    fn set_property_nodeflate(&self, nodeflate: bool) {
        unsafe {
            gobject_ffi::g_object_set_property(
                self.to_glib_none().0,
                "nodeflate".to_glib_none().0,
                Value::from(&nodeflate).to_glib_none().0,
            );
        }
    }

    fn set_property_refuse_chap(&self, refuse_chap: bool) {
        unsafe {
            gobject_ffi::g_object_set_property(
                self.to_glib_none().0,
                "refuse-chap".to_glib_none().0,
                Value::from(&refuse_chap).to_glib_none().0,
            );
        }
    }

    fn set_property_refuse_eap(&self, refuse_eap: bool) {
        unsafe {
            gobject_ffi::g_object_set_property(
                self.to_glib_none().0,
                "refuse-eap".to_glib_none().0,
                Value::from(&refuse_eap).to_glib_none().0,
            );
        }
    }

    fn set_property_refuse_mschap(&self, refuse_mschap: bool) {
        unsafe {
            gobject_ffi::g_object_set_property(
                self.to_glib_none().0,
                "refuse-mschap".to_glib_none().0,
                Value::from(&refuse_mschap).to_glib_none().0,
            );
        }
    }

    fn set_property_refuse_mschapv2(&self, refuse_mschapv2: bool) {
        unsafe {
            gobject_ffi::g_object_set_property(
                self.to_glib_none().0,
                "refuse-mschapv2".to_glib_none().0,
                Value::from(&refuse_mschapv2).to_glib_none().0,
            );
        }
    }

    fn set_property_refuse_pap(&self, refuse_pap: bool) {
        unsafe {
            gobject_ffi::g_object_set_property(
                self.to_glib_none().0,
                "refuse-pap".to_glib_none().0,
                Value::from(&refuse_pap).to_glib_none().0,
            );
        }
    }

    fn set_property_require_mppe(&self, require_mppe: bool) {
        unsafe {
            gobject_ffi::g_object_set_property(
                self.to_glib_none().0,
                "require-mppe".to_glib_none().0,
                Value::from(&require_mppe).to_glib_none().0,
            );
        }
    }

    fn set_property_require_mppe_128(&self, require_mppe_128: bool) {
        unsafe {
            gobject_ffi::g_object_set_property(
                self.to_glib_none().0,
                "require-mppe-128".to_glib_none().0,
                Value::from(&require_mppe_128).to_glib_none().0,
            );
        }
    }

    fn connect_property_baud_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(
                self.to_glib_none().0,
                "notify::baud",
                transmute(notify_baud_trampoline::<Self> as usize),
                Box_::into_raw(f) as *mut _,
            )
        }
    }

    fn connect_property_crtscts_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(
                self.to_glib_none().0,
                "notify::crtscts",
                transmute(notify_crtscts_trampoline::<Self> as usize),
                Box_::into_raw(f) as *mut _,
            )
        }
    }

    fn connect_property_lcp_echo_failure_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(
                self.to_glib_none().0,
                "notify::lcp-echo-failure",
                transmute(notify_lcp_echo_failure_trampoline::<Self> as usize),
                Box_::into_raw(f) as *mut _,
            )
        }
    }

    fn connect_property_lcp_echo_interval_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(
                self.to_glib_none().0,
                "notify::lcp-echo-interval",
                transmute(notify_lcp_echo_interval_trampoline::<Self> as usize),
                Box_::into_raw(f) as *mut _,
            )
        }
    }

    fn connect_property_mppe_stateful_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(
                self.to_glib_none().0,
                "notify::mppe-stateful",
                transmute(notify_mppe_stateful_trampoline::<Self> as usize),
                Box_::into_raw(f) as *mut _,
            )
        }
    }

    fn connect_property_mru_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(
                self.to_glib_none().0,
                "notify::mru",
                transmute(notify_mru_trampoline::<Self> as usize),
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

    fn connect_property_no_vj_comp_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(
                self.to_glib_none().0,
                "notify::no-vj-comp",
                transmute(notify_no_vj_comp_trampoline::<Self> as usize),
                Box_::into_raw(f) as *mut _,
            )
        }
    }

    fn connect_property_noauth_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(
                self.to_glib_none().0,
                "notify::noauth",
                transmute(notify_noauth_trampoline::<Self> as usize),
                Box_::into_raw(f) as *mut _,
            )
        }
    }

    fn connect_property_nobsdcomp_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(
                self.to_glib_none().0,
                "notify::nobsdcomp",
                transmute(notify_nobsdcomp_trampoline::<Self> as usize),
                Box_::into_raw(f) as *mut _,
            )
        }
    }

    fn connect_property_nodeflate_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(
                self.to_glib_none().0,
                "notify::nodeflate",
                transmute(notify_nodeflate_trampoline::<Self> as usize),
                Box_::into_raw(f) as *mut _,
            )
        }
    }

    fn connect_property_refuse_chap_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(
                self.to_glib_none().0,
                "notify::refuse-chap",
                transmute(notify_refuse_chap_trampoline::<Self> as usize),
                Box_::into_raw(f) as *mut _,
            )
        }
    }

    fn connect_property_refuse_eap_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(
                self.to_glib_none().0,
                "notify::refuse-eap",
                transmute(notify_refuse_eap_trampoline::<Self> as usize),
                Box_::into_raw(f) as *mut _,
            )
        }
    }

    fn connect_property_refuse_mschap_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(
                self.to_glib_none().0,
                "notify::refuse-mschap",
                transmute(notify_refuse_mschap_trampoline::<Self> as usize),
                Box_::into_raw(f) as *mut _,
            )
        }
    }

    fn connect_property_refuse_mschapv2_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(
                self.to_glib_none().0,
                "notify::refuse-mschapv2",
                transmute(notify_refuse_mschapv2_trampoline::<Self> as usize),
                Box_::into_raw(f) as *mut _,
            )
        }
    }

    fn connect_property_refuse_pap_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(
                self.to_glib_none().0,
                "notify::refuse-pap",
                transmute(notify_refuse_pap_trampoline::<Self> as usize),
                Box_::into_raw(f) as *mut _,
            )
        }
    }

    fn connect_property_require_mppe_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(
                self.to_glib_none().0,
                "notify::require-mppe",
                transmute(notify_require_mppe_trampoline::<Self> as usize),
                Box_::into_raw(f) as *mut _,
            )
        }
    }

    fn connect_property_require_mppe_128_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(
                self.to_glib_none().0,
                "notify::require-mppe-128",
                transmute(notify_require_mppe_128_trampoline::<Self> as usize),
                Box_::into_raw(f) as *mut _,
            )
        }
    }
}

unsafe extern "C" fn notify_baud_trampoline<P>(
    this: *mut ffi::NMSettingPpp,
    _param_spec: glib_ffi::gpointer,
    f: glib_ffi::gpointer,
) where
    P: IsA<SettingPpp>,
{
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&SettingPpp::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_crtscts_trampoline<P>(
    this: *mut ffi::NMSettingPpp,
    _param_spec: glib_ffi::gpointer,
    f: glib_ffi::gpointer,
) where
    P: IsA<SettingPpp>,
{
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&SettingPpp::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_lcp_echo_failure_trampoline<P>(
    this: *mut ffi::NMSettingPpp,
    _param_spec: glib_ffi::gpointer,
    f: glib_ffi::gpointer,
) where
    P: IsA<SettingPpp>,
{
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&SettingPpp::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_lcp_echo_interval_trampoline<P>(
    this: *mut ffi::NMSettingPpp,
    _param_spec: glib_ffi::gpointer,
    f: glib_ffi::gpointer,
) where
    P: IsA<SettingPpp>,
{
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&SettingPpp::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_mppe_stateful_trampoline<P>(
    this: *mut ffi::NMSettingPpp,
    _param_spec: glib_ffi::gpointer,
    f: glib_ffi::gpointer,
) where
    P: IsA<SettingPpp>,
{
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&SettingPpp::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_mru_trampoline<P>(
    this: *mut ffi::NMSettingPpp,
    _param_spec: glib_ffi::gpointer,
    f: glib_ffi::gpointer,
) where
    P: IsA<SettingPpp>,
{
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&SettingPpp::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_mtu_trampoline<P>(
    this: *mut ffi::NMSettingPpp,
    _param_spec: glib_ffi::gpointer,
    f: glib_ffi::gpointer,
) where
    P: IsA<SettingPpp>,
{
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&SettingPpp::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_no_vj_comp_trampoline<P>(
    this: *mut ffi::NMSettingPpp,
    _param_spec: glib_ffi::gpointer,
    f: glib_ffi::gpointer,
) where
    P: IsA<SettingPpp>,
{
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&SettingPpp::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_noauth_trampoline<P>(
    this: *mut ffi::NMSettingPpp,
    _param_spec: glib_ffi::gpointer,
    f: glib_ffi::gpointer,
) where
    P: IsA<SettingPpp>,
{
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&SettingPpp::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_nobsdcomp_trampoline<P>(
    this: *mut ffi::NMSettingPpp,
    _param_spec: glib_ffi::gpointer,
    f: glib_ffi::gpointer,
) where
    P: IsA<SettingPpp>,
{
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&SettingPpp::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_nodeflate_trampoline<P>(
    this: *mut ffi::NMSettingPpp,
    _param_spec: glib_ffi::gpointer,
    f: glib_ffi::gpointer,
) where
    P: IsA<SettingPpp>,
{
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&SettingPpp::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_refuse_chap_trampoline<P>(
    this: *mut ffi::NMSettingPpp,
    _param_spec: glib_ffi::gpointer,
    f: glib_ffi::gpointer,
) where
    P: IsA<SettingPpp>,
{
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&SettingPpp::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_refuse_eap_trampoline<P>(
    this: *mut ffi::NMSettingPpp,
    _param_spec: glib_ffi::gpointer,
    f: glib_ffi::gpointer,
) where
    P: IsA<SettingPpp>,
{
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&SettingPpp::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_refuse_mschap_trampoline<P>(
    this: *mut ffi::NMSettingPpp,
    _param_spec: glib_ffi::gpointer,
    f: glib_ffi::gpointer,
) where
    P: IsA<SettingPpp>,
{
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&SettingPpp::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_refuse_mschapv2_trampoline<P>(
    this: *mut ffi::NMSettingPpp,
    _param_spec: glib_ffi::gpointer,
    f: glib_ffi::gpointer,
) where
    P: IsA<SettingPpp>,
{
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&SettingPpp::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_refuse_pap_trampoline<P>(
    this: *mut ffi::NMSettingPpp,
    _param_spec: glib_ffi::gpointer,
    f: glib_ffi::gpointer,
) where
    P: IsA<SettingPpp>,
{
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&SettingPpp::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_require_mppe_trampoline<P>(
    this: *mut ffi::NMSettingPpp,
    _param_spec: glib_ffi::gpointer,
    f: glib_ffi::gpointer,
) where
    P: IsA<SettingPpp>,
{
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&SettingPpp::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_require_mppe_128_trampoline<P>(
    this: *mut ffi::NMSettingPpp,
    _param_spec: glib_ffi::gpointer,
    f: glib_ffi::gpointer,
) where
    P: IsA<SettingPpp>,
{
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&SettingPpp::from_glib_borrow(this).downcast_unchecked())
}
