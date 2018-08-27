// This file was generated by gir (https://github.com/gtk-rs/gir @ 464833e)
// from gir-files (https://github.com/gtk-rs/gir-files @ ???)
// DO NOT EDIT

use ffi;
use glib::StaticType;
use glib::Type;
use glib::translate::*;
use glib::value::FromValue;
use glib::value::FromValueOptional;
use glib::value::SetValue;
use glib::value::Value;
use gobject_ffi;

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[derive(Clone, Copy)]
pub enum _80211Mode {
    Unknown,
    Adhoc,
    Infra,
    Ap,
    #[doc(hidden)]
    __Unknown(i32),
}

#[doc(hidden)]
impl ToGlib for _80211Mode {
    type GlibType = ffi::NM80211Mode;

    fn to_glib(&self) -> ffi::NM80211Mode {
        match *self {
            _80211Mode::Unknown => ffi::NM_802_11_MODE_UNKNOWN,
            _80211Mode::Adhoc => ffi::NM_802_11_MODE_ADHOC,
            _80211Mode::Infra => ffi::NM_802_11_MODE_INFRA,
            _80211Mode::Ap => ffi::NM_802_11_MODE_AP,
            _80211Mode::__Unknown(value) => value
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::NM80211Mode> for _80211Mode {
    fn from_glib(value: ffi::NM80211Mode) -> Self {
        match value {
            0 => _80211Mode::Unknown,
            1 => _80211Mode::Adhoc,
            2 => _80211Mode::Infra,
            3 => _80211Mode::Ap,
            value => _80211Mode::__Unknown(value),
        }
    }
}

impl StaticType for _80211Mode {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::nm_802_11_mode_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for _80211Mode {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for _80211Mode {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_ffi::g_value_get_enum(value.to_glib_none().0))
    }
}

impl SetValue for _80211Mode {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_ffi::g_value_set_enum(value.to_glib_none_mut().0, this.to_glib())
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[derive(Clone, Copy)]
pub enum ConnectivityState {
    Unknown,
    None,
    Portal,
    Limited,
    Full,
    #[doc(hidden)]
    __Unknown(i32),
}

#[doc(hidden)]
impl ToGlib for ConnectivityState {
    type GlibType = ffi::NMConnectivityState;

    fn to_glib(&self) -> ffi::NMConnectivityState {
        match *self {
            ConnectivityState::Unknown => ffi::NM_CONNECTIVITY_UNKNOWN,
            ConnectivityState::None => ffi::NM_CONNECTIVITY_NONE,
            ConnectivityState::Portal => ffi::NM_CONNECTIVITY_PORTAL,
            ConnectivityState::Limited => ffi::NM_CONNECTIVITY_LIMITED,
            ConnectivityState::Full => ffi::NM_CONNECTIVITY_FULL,
            ConnectivityState::__Unknown(value) => value
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::NMConnectivityState> for ConnectivityState {
    fn from_glib(value: ffi::NMConnectivityState) -> Self {
        match value {
            0 => ConnectivityState::Unknown,
            1 => ConnectivityState::None,
            2 => ConnectivityState::Portal,
            3 => ConnectivityState::Limited,
            4 => ConnectivityState::Full,
            value => ConnectivityState::__Unknown(value),
        }
    }
}

impl StaticType for ConnectivityState {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::nm_connectivity_state_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for ConnectivityState {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for ConnectivityState {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_ffi::g_value_get_enum(value.to_glib_none().0))
    }
}

impl SetValue for ConnectivityState {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_ffi::g_value_set_enum(value.to_glib_none_mut().0, this.to_glib())
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[derive(Clone, Copy)]
pub enum State {
    Unknown,
    Asleep,
    Disconnected,
    Disconnecting,
    Connecting,
    ConnectedLocal,
    ConnectedSite,
    ConnectedGlobal,
    #[doc(hidden)]
    __Unknown(i32),
}

#[doc(hidden)]
impl ToGlib for State {
    type GlibType = ffi::NMState;

    fn to_glib(&self) -> ffi::NMState {
        match *self {
            State::Unknown => ffi::NM_STATE_UNKNOWN,
            State::Asleep => ffi::NM_STATE_ASLEEP,
            State::Disconnected => ffi::NM_STATE_DISCONNECTED,
            State::Disconnecting => ffi::NM_STATE_DISCONNECTING,
            State::Connecting => ffi::NM_STATE_CONNECTING,
            State::ConnectedLocal => ffi::NM_STATE_CONNECTED_LOCAL,
            State::ConnectedSite => ffi::NM_STATE_CONNECTED_SITE,
            State::ConnectedGlobal => ffi::NM_STATE_CONNECTED_GLOBAL,
            State::__Unknown(value) => value
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::NMState> for State {
    fn from_glib(value: ffi::NMState) -> Self {
        match value {
            0 => State::Unknown,
            10 => State::Asleep,
            20 => State::Disconnected,
            30 => State::Disconnecting,
            40 => State::Connecting,
            50 => State::ConnectedLocal,
            60 => State::ConnectedSite,
            70 => State::ConnectedGlobal,
            value => State::__Unknown(value),
        }
    }
}

impl StaticType for State {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::nm_state_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for State {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for State {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_ffi::g_value_get_enum(value.to_glib_none().0))
    }
}

impl SetValue for State {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_ffi::g_value_set_enum(value.to_glib_none_mut().0, this.to_glib())
    }
}

