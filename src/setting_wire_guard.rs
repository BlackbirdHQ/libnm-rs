// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files
// DO NOT EDIT

use crate::Setting;
use crate::SettingSecretFlags;
#[cfg(any(feature = "v1_20", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_20")))]
use crate::Ternary;
use crate::WireGuardPeer;
use glib::object::Cast;
use glib::object::ObjectType as ObjectType_;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::ToValue;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem;
use std::mem::transmute;

glib::wrapper! {
    #[doc(alias = "NMSettingWireGuard")]
    pub struct SettingWireGuard(Object<ffi::NMSettingWireGuard, ffi::NMSettingWireGuardClass>) @extends Setting;

    match fn {
        type_ => || ffi::nm_setting_wireguard_get_type(),
    }
}

impl SettingWireGuard {
    /// Creates a new [`SettingWireGuard`][crate::SettingWireGuard] object with default values.
    ///
    /// # Returns
    ///
    /// the new empty [`SettingWireGuard`][crate::SettingWireGuard] object
    #[doc(alias = "nm_setting_wireguard_new")]
    pub fn new() -> SettingWireGuard {
        unsafe { Setting::from_glib_full(ffi::nm_setting_wireguard_new()).unsafe_cast() }
    }

    /// If a peer with the same public-key already exists, that
    /// one is replaced by `peer`. The new `peer` is always appended
    /// (or moved to) the end, so in case a peer is replaced, the
    /// indexes are shifted and the number of peers stays unchanged.
    /// ## `peer`
    /// the [`WireGuardPeer`][crate::WireGuardPeer] instance to append.
    ///  This seals `peer` and keeps a reference on the
    ///  instance.
    #[doc(alias = "nm_setting_wireguard_append_peer")]
    pub fn append_peer(&self, peer: &WireGuardPeer) {
        unsafe {
            ffi::nm_setting_wireguard_append_peer(self.to_glib_none().0, peer.to_glib_none().0);
        }
    }

    #[doc(alias = "nm_setting_wireguard_clear_peers")]
    pub fn clear_peers(&self) -> u32 {
        unsafe { ffi::nm_setting_wireguard_clear_peers(self.to_glib_none().0) }
    }

    ///
    /// # Returns
    ///
    /// the set firewall mark.
    #[doc(alias = "nm_setting_wireguard_get_fwmark")]
    #[doc(alias = "get_fwmark")]
    pub fn fwmark(&self) -> u32 {
        unsafe { ffi::nm_setting_wireguard_get_fwmark(self.to_glib_none().0) }
    }

    ///
    /// # Returns
    ///
    /// the "ip4-auto-default-route" property of the setting.
    #[cfg(any(feature = "v1_20", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_20")))]
    #[doc(alias = "nm_setting_wireguard_get_ip4_auto_default_route")]
    #[doc(alias = "get_ip4_auto_default_route")]
    pub fn ip4_auto_default_route(&self) -> Ternary {
        unsafe {
            from_glib(ffi::nm_setting_wireguard_get_ip4_auto_default_route(
                self.to_glib_none().0,
            ))
        }
    }

    ///
    /// # Returns
    ///
    /// the "ip6-auto-default-route" property of the setting.
    #[cfg(any(feature = "v1_20", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_20")))]
    #[doc(alias = "nm_setting_wireguard_get_ip6_auto_default_route")]
    #[doc(alias = "get_ip6_auto_default_route")]
    pub fn ip6_auto_default_route(&self) -> Ternary {
        unsafe {
            from_glib(ffi::nm_setting_wireguard_get_ip6_auto_default_route(
                self.to_glib_none().0,
            ))
        }
    }

    ///
    /// # Returns
    ///
    /// the set UDP listen port.
    #[doc(alias = "nm_setting_wireguard_get_listen_port")]
    #[doc(alias = "get_listen_port")]
    pub fn listen_port(&self) -> u16 {
        unsafe { ffi::nm_setting_wireguard_get_listen_port(self.to_glib_none().0) }
    }

    ///
    /// # Returns
    ///
    /// the MTU of the setting.
    #[doc(alias = "nm_setting_wireguard_get_mtu")]
    #[doc(alias = "get_mtu")]
    pub fn mtu(&self) -> u32 {
        unsafe { ffi::nm_setting_wireguard_get_mtu(self.to_glib_none().0) }
    }

    /// ## `idx`
    /// the index to lookup.
    ///
    /// # Returns
    ///
    /// the [`WireGuardPeer`][crate::WireGuardPeer] entry at
    ///  index `idx`. If the index is out of range, [`None`] is returned.
    #[doc(alias = "nm_setting_wireguard_get_peer")]
    #[doc(alias = "get_peer")]
    pub fn peer(&self, idx: u32) -> Option<WireGuardPeer> {
        unsafe {
            from_glib_none(ffi::nm_setting_wireguard_get_peer(
                self.to_glib_none().0,
                idx,
            ))
        }
    }

    /// ## `public_key`
    /// the public key for looking up the
    ///  peer.
    ///
    /// # Returns
    ///
    /// the [`WireGuardPeer`][crate::WireGuardPeer] instance with a
    ///  matching public key. If no such peer exists, [`None`] is returned.
    ///
    /// ## `out_idx`
    /// optional output argument
    ///  for the index of the found peer. If no index is found,
    ///  this is set to the [`peers_len()`][Self::peers_len()].
    #[doc(alias = "nm_setting_wireguard_get_peer_by_public_key")]
    #[doc(alias = "get_peer_by_public_key")]
    pub fn peer_by_public_key(&self, public_key: &str) -> (WireGuardPeer, u32) {
        unsafe {
            let mut out_idx = mem::MaybeUninit::uninit();
            let ret = from_glib_none(ffi::nm_setting_wireguard_get_peer_by_public_key(
                self.to_glib_none().0,
                public_key.to_glib_none().0,
                out_idx.as_mut_ptr(),
            ));
            (ret, out_idx.assume_init())
        }
    }

    ///
    /// # Returns
    ///
    /// whether automatically add peer routes.
    #[doc(alias = "nm_setting_wireguard_get_peer_routes")]
    #[doc(alias = "get_peer_routes")]
    pub fn is_peer_routes(&self) -> bool {
        unsafe {
            from_glib(ffi::nm_setting_wireguard_get_peer_routes(
                self.to_glib_none().0,
            ))
        }
    }

    ///
    /// # Returns
    ///
    /// the number of registered peers.
    #[doc(alias = "nm_setting_wireguard_get_peers_len")]
    #[doc(alias = "get_peers_len")]
    pub fn peers_len(&self) -> u32 {
        unsafe { ffi::nm_setting_wireguard_get_peers_len(self.to_glib_none().0) }
    }

    ///
    /// # Returns
    ///
    /// the set private-key or [`None`].
    #[doc(alias = "nm_setting_wireguard_get_private_key")]
    #[doc(alias = "get_private_key")]
    pub fn private_key(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::nm_setting_wireguard_get_private_key(
                self.to_glib_none().0,
            ))
        }
    }

    ///
    /// # Returns
    ///
    /// the secret-flags for `property::SettingWireGuard::private-key`.
    #[doc(alias = "nm_setting_wireguard_get_private_key_flags")]
    #[doc(alias = "get_private_key_flags")]
    pub fn private_key_flags(&self) -> SettingSecretFlags {
        unsafe {
            from_glib(ffi::nm_setting_wireguard_get_private_key_flags(
                self.to_glib_none().0,
            ))
        }
    }

    /// ## `idx`
    /// the index to remove.
    ///
    /// # Returns
    ///
    /// [`true`] if `idx` was in range and a peer
    ///  was removed. Otherwise, `self` is unchanged.
    #[doc(alias = "nm_setting_wireguard_remove_peer")]
    pub fn remove_peer(&self, idx: u32) -> bool {
        unsafe {
            from_glib(ffi::nm_setting_wireguard_remove_peer(
                self.to_glib_none().0,
                idx,
            ))
        }
    }

    /// If `idx` is one past the last peer, the behavior is the same
    /// as [`append_peer()`][Self::append_peer()].
    /// Otherwise, the peer will be at `idx` and replace the peer
    /// instance at that index. Note that if a peer with the same
    /// public-key exists on another index, then that peer will also
    /// be replaced. In that case, the number of peers will shrink
    /// by one (because the one at `idx` got replace and then one
    /// with the same public-key got removed). This also means,
    /// that the resulting index afterwards may be one less than
    /// `idx` (if another peer with a lower index was dropped).
    /// ## `peer`
    /// the [`WireGuardPeer`][crate::WireGuardPeer] instance to set.
    ///  This seals `peer` and keeps a reference on the
    ///  instance.
    /// ## `idx`
    /// the index, in the range of 0 to the number of
    ///  peers (including). That means, if `idx` is one past
    ///  the end of the number of peers, this is the same as
    ///  [`append_peer()`][Self::append_peer()]. Otherwise, the
    ///  peer at this index is replaced.
    #[doc(alias = "nm_setting_wireguard_set_peer")]
    pub fn set_peer(&self, peer: &WireGuardPeer, idx: u32) {
        unsafe {
            ffi::nm_setting_wireguard_set_peer(self.to_glib_none().0, peer.to_glib_none().0, idx);
        }
    }

    /// The use of fwmark is optional and is by default off. Setting it to 0
    /// disables it. Otherwise, it is a 32-bit fwmark for outgoing packets.
    ///
    /// Note that "ip4-auto-default-route" or "ip6-auto-default-route" enabled,
    /// implies to automatically choose a fwmark.
    #[cfg(any(feature = "v1_16", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_16")))]
    pub fn set_fwmark(&self, fwmark: u32) {
        glib::ObjectExt::set_property(self, "fwmark", &fwmark)
    }

    /// Whether to enable special handling of the IPv4 default route.
    /// If enabled, the IPv4 default route from wireguard.peer-routes
    /// will be placed to a dedicated routing-table and two policy routing rules
    /// will be added. The fwmark number is also used as routing-table for the default-route,
    /// and if fwmark is zero, an unused fwmark/table is chosen automatically.
    /// This corresponds to what wg-quick does with Table=auto and what WireGuard
    /// calls "Improved Rule-based Routing".
    ///
    /// Note that for this automatism to work, you usually don't want to set
    /// ipv4.gateway, because that will result in a conflicting default route.
    ///
    /// Leaving this at the default will enable this option automatically
    /// if ipv4.never-default is not set and there are any peers that use
    /// a default-route as allowed-ips.
    #[cfg(any(feature = "v1_20", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_20")))]
    #[doc(alias = "ip4-auto-default-route")]
    pub fn set_ip4_auto_default_route(&self, ip4_auto_default_route: Ternary) {
        glib::ObjectExt::set_property(self, "ip4-auto-default-route", &ip4_auto_default_route)
    }

    /// Like ip4-auto-default-route, but for the IPv6 default route.
    #[cfg(any(feature = "v1_20", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_20")))]
    #[doc(alias = "ip6-auto-default-route")]
    pub fn set_ip6_auto_default_route(&self, ip6_auto_default_route: Ternary) {
        glib::ObjectExt::set_property(self, "ip6-auto-default-route", &ip6_auto_default_route)
    }

    /// The listen-port. If listen-port is not specified, the port will be chosen
    /// randomly when the interface comes up.
    #[cfg(any(feature = "v1_16", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_16")))]
    #[doc(alias = "listen-port")]
    pub fn set_listen_port(&self, listen_port: u32) {
        glib::ObjectExt::set_property(self, "listen-port", &listen_port)
    }

    /// If non-zero, only transmit packets of the specified size or smaller,
    /// breaking larger packets up into multiple fragments.
    ///
    /// If zero a default MTU is used. Note that contrary to wg-quick's MTU
    /// setting, this does not take into account the current routes at the
    /// time of activation.
    #[cfg(any(feature = "v1_16", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_16")))]
    pub fn set_mtu(&self, mtu: u32) {
        glib::ObjectExt::set_property(self, "mtu", &mtu)
    }

    /// Whether to automatically add routes for the AllowedIPs ranges
    /// of the peers. If [`true`] (the default), NetworkManager will automatically
    /// add routes in the routing tables according to ipv4.route-table and
    /// ipv6.route-table. Usually you want this automatism enabled.
    /// If [`false`], no such routes are added automatically. In this case, the
    /// user may want to configure static routes in ipv4.routes and ipv6.routes,
    /// respectively.
    ///
    /// Note that if the peer's AllowedIPs is "0.0.0.0/0" or "::/0" and the profile's
    /// ipv4.never-default or ipv6.never-default setting is enabled, the peer route for
    /// this peer won't be added automatically.
    #[cfg(any(feature = "v1_16", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_16")))]
    #[doc(alias = "peer-routes")]
    pub fn set_peer_routes(&self, peer_routes: bool) {
        glib::ObjectExt::set_property(self, "peer-routes", &peer_routes)
    }

    /// The 256 bit private-key in base64 encoding.
    #[cfg(any(feature = "v1_16", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_16")))]
    #[doc(alias = "private-key")]
    pub fn set_private_key(&self, private_key: Option<&str>) {
        glib::ObjectExt::set_property(self, "private-key", &private_key)
    }

    /// Flags indicating how to handle the `property::SettingWirelessSecurity::private-key`
    /// property.
    #[cfg(any(feature = "v1_16", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_16")))]
    #[doc(alias = "private-key-flags")]
    pub fn set_private_key_flags(&self, private_key_flags: SettingSecretFlags) {
        glib::ObjectExt::set_property(self, "private-key-flags", &private_key_flags)
    }

    #[cfg(any(feature = "v1_16", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_16")))]
    #[doc(alias = "fwmark")]
    pub fn connect_fwmark_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_fwmark_trampoline<F: Fn(&SettingWireGuard) + 'static>(
            this: *mut ffi::NMSettingWireGuard,
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
                b"notify::fwmark\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_fwmark_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v1_20", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_20")))]
    #[doc(alias = "ip4-auto-default-route")]
    pub fn connect_ip4_auto_default_route_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_ip4_auto_default_route_trampoline<
            F: Fn(&SettingWireGuard) + 'static,
        >(
            this: *mut ffi::NMSettingWireGuard,
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
                b"notify::ip4-auto-default-route\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_ip4_auto_default_route_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v1_20", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_20")))]
    #[doc(alias = "ip6-auto-default-route")]
    pub fn connect_ip6_auto_default_route_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_ip6_auto_default_route_trampoline<
            F: Fn(&SettingWireGuard) + 'static,
        >(
            this: *mut ffi::NMSettingWireGuard,
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
                b"notify::ip6-auto-default-route\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_ip6_auto_default_route_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v1_16", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_16")))]
    #[doc(alias = "listen-port")]
    pub fn connect_listen_port_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_listen_port_trampoline<F: Fn(&SettingWireGuard) + 'static>(
            this: *mut ffi::NMSettingWireGuard,
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
                b"notify::listen-port\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_listen_port_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v1_16", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_16")))]
    #[doc(alias = "mtu")]
    pub fn connect_mtu_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_mtu_trampoline<F: Fn(&SettingWireGuard) + 'static>(
            this: *mut ffi::NMSettingWireGuard,
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
                b"notify::mtu\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_mtu_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v1_16", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_16")))]
    #[doc(alias = "peer-routes")]
    pub fn connect_peer_routes_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_peer_routes_trampoline<F: Fn(&SettingWireGuard) + 'static>(
            this: *mut ffi::NMSettingWireGuard,
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
                b"notify::peer-routes\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_peer_routes_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v1_16", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_16")))]
    #[doc(alias = "private-key")]
    pub fn connect_private_key_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_private_key_trampoline<F: Fn(&SettingWireGuard) + 'static>(
            this: *mut ffi::NMSettingWireGuard,
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
                b"notify::private-key\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_private_key_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v1_16", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_16")))]
    #[doc(alias = "private-key-flags")]
    pub fn connect_private_key_flags_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_private_key_flags_trampoline<
            F: Fn(&SettingWireGuard) + 'static,
        >(
            this: *mut ffi::NMSettingWireGuard,
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
                b"notify::private-key-flags\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_private_key_flags_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

#[cfg(any(feature = "v1_16", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_16")))]
impl Default for SettingWireGuard {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for SettingWireGuard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("SettingWireGuard")
    }
}
