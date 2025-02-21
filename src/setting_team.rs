// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files
// DO NOT EDIT

use crate::Setting;
#[cfg(any(feature = "v1_12", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_12")))]
use crate::TeamLinkWatcher;
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
    #[doc(alias = "NMSettingTeam")]
    pub struct SettingTeam(Object<ffi::NMSettingTeam, ffi::NMSettingTeamClass>) @extends Setting;

    match fn {
        type_ => || ffi::nm_setting_team_get_type(),
    }
}

impl SettingTeam {
    /// Creates a new [`SettingTeam`][crate::SettingTeam] object with default values.
    ///
    /// # Returns
    ///
    /// the new empty [`SettingTeam`][crate::SettingTeam] object
    #[doc(alias = "nm_setting_team_new")]
    pub fn new() -> SettingTeam {
        unsafe { Setting::from_glib_full(ffi::nm_setting_team_new()).unsafe_cast() }
    }

    /// Appends a new link watcher to the setting.
    /// ## `link_watcher`
    /// the link watcher to add
    ///
    /// # Returns
    ///
    /// [`true`] if the link watcher is added; [`false`] if an identical link
    /// watcher was already there.
    #[cfg(any(feature = "v1_12", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_12")))]
    #[doc(alias = "nm_setting_team_add_link_watcher")]
    pub fn add_link_watcher(&self, link_watcher: &TeamLinkWatcher) -> bool {
        unsafe {
            from_glib(ffi::nm_setting_team_add_link_watcher(
                self.to_glib_none().0,
                link_watcher.to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v1_12", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_12")))]
    #[doc(alias = "nm_setting_team_add_runner_tx_hash")]
    pub fn add_runner_tx_hash(&self, txhash: &str) -> bool {
        unsafe {
            from_glib(ffi::nm_setting_team_add_runner_tx_hash(
                self.to_glib_none().0,
                txhash.to_glib_none().0,
            ))
        }
    }

    /// Removes all configured link watchers.
    #[cfg(any(feature = "v1_12", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_12")))]
    #[doc(alias = "nm_setting_team_clear_link_watchers")]
    pub fn clear_link_watchers(&self) {
        unsafe {
            ffi::nm_setting_team_clear_link_watchers(self.to_glib_none().0);
        }
    }

    ///
    /// # Returns
    ///
    /// the `property::SettingTeam::config` property of the setting
    #[doc(alias = "nm_setting_team_get_config")]
    #[doc(alias = "get_config")]
    pub fn config(&self) -> Option<glib::GString> {
        unsafe { from_glib_none(ffi::nm_setting_team_get_config(self.to_glib_none().0)) }
    }

    /// ## `idx`
    /// index number of the link watcher to return
    ///
    /// # Returns
    ///
    /// the link watcher at index `idx`.
    #[cfg(any(feature = "v1_12", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_12")))]
    #[doc(alias = "nm_setting_team_get_link_watcher")]
    #[doc(alias = "get_link_watcher")]
    pub fn link_watcher(&self, idx: u32) -> Option<TeamLinkWatcher> {
        unsafe {
            from_glib_none(ffi::nm_setting_team_get_link_watcher(
                self.to_glib_none().0,
                idx,
            ))
        }
    }

    ///
    /// # Returns
    ///
    /// the #`property::SettingTeam::mcast-rejoin-count` property of the setting
    #[cfg(any(feature = "v1_12", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_12")))]
    #[doc(alias = "nm_setting_team_get_mcast_rejoin_count")]
    #[doc(alias = "get_mcast_rejoin_count")]
    pub fn mcast_rejoin_count(&self) -> i32 {
        unsafe { ffi::nm_setting_team_get_mcast_rejoin_count(self.to_glib_none().0) }
    }

    ///
    /// # Returns
    ///
    /// the #`property::SettingTeam::mcast-rejoin-interval` property of the setting
    #[cfg(any(feature = "v1_12", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_12")))]
    #[doc(alias = "nm_setting_team_get_mcast_rejoin_interval")]
    #[doc(alias = "get_mcast_rejoin_interval")]
    pub fn mcast_rejoin_interval(&self) -> i32 {
        unsafe { ffi::nm_setting_team_get_mcast_rejoin_interval(self.to_glib_none().0) }
    }

    ///
    /// # Returns
    ///
    /// the #`property::SettingTeam::notify-peers-count` property of the setting
    #[cfg(any(feature = "v1_12", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_12")))]
    #[doc(alias = "nm_setting_team_get_notify_peers_count")]
    #[doc(alias = "get_notify_peers_count")]
    pub fn notify_peers_count(&self) -> i32 {
        unsafe { ffi::nm_setting_team_get_notify_peers_count(self.to_glib_none().0) }
    }

    ///
    /// # Returns
    ///
    /// the #`property::SettingTeam::notify-peers-interval` property of the setting
    #[cfg(any(feature = "v1_12", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_12")))]
    #[doc(alias = "nm_setting_team_get_notify_peers_interval")]
    #[doc(alias = "get_notify_peers_interval")]
    pub fn notify_peers_interval(&self) -> i32 {
        unsafe { ffi::nm_setting_team_get_notify_peers_interval(self.to_glib_none().0) }
    }

    ///
    /// # Returns
    ///
    /// the number of configured link watchers
    #[cfg(any(feature = "v1_12", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_12")))]
    #[doc(alias = "nm_setting_team_get_num_link_watchers")]
    #[doc(alias = "get_num_link_watchers")]
    pub fn num_link_watchers(&self) -> u32 {
        unsafe { ffi::nm_setting_team_get_num_link_watchers(self.to_glib_none().0) }
    }

    #[cfg(any(feature = "v1_12", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_12")))]
    #[doc(alias = "nm_setting_team_get_num_runner_tx_hash")]
    #[doc(alias = "get_num_runner_tx_hash")]
    pub fn num_runner_tx_hash(&self) -> u32 {
        unsafe { ffi::nm_setting_team_get_num_runner_tx_hash(self.to_glib_none().0) }
    }

    ///
    /// # Returns
    ///
    /// the #`property::SettingTeam::runner` property of the setting
    #[cfg(any(feature = "v1_12", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_12")))]
    #[doc(alias = "nm_setting_team_get_runner")]
    #[doc(alias = "get_runner")]
    pub fn runner(&self) -> Option<glib::GString> {
        unsafe { from_glib_none(ffi::nm_setting_team_get_runner(self.to_glib_none().0)) }
    }

    ///
    /// # Returns
    ///
    /// the #`property::SettingTeam::runner_active` property of the setting
    #[cfg(any(feature = "v1_12", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_12")))]
    #[doc(alias = "nm_setting_team_get_runner_active")]
    #[doc(alias = "get_runner_active")]
    pub fn is_runner_active(&self) -> bool {
        unsafe {
            from_glib(ffi::nm_setting_team_get_runner_active(
                self.to_glib_none().0,
            ))
        }
    }

    ///
    /// # Returns
    ///
    /// the #`property::SettingTeam::runner-agg-select-policy` property of the setting
    #[cfg(any(feature = "v1_12", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_12")))]
    #[doc(alias = "nm_setting_team_get_runner_agg_select_policy")]
    #[doc(alias = "get_runner_agg_select_policy")]
    pub fn runner_agg_select_policy(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::nm_setting_team_get_runner_agg_select_policy(
                self.to_glib_none().0,
            ))
        }
    }

    ///
    /// # Returns
    ///
    /// the #`property::SettingTeam::runner-fast-rate` property of the setting
    #[cfg(any(feature = "v1_12", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_12")))]
    #[doc(alias = "nm_setting_team_get_runner_fast_rate")]
    #[doc(alias = "get_runner_fast_rate")]
    pub fn is_runner_fast_rate(&self) -> bool {
        unsafe {
            from_glib(ffi::nm_setting_team_get_runner_fast_rate(
                self.to_glib_none().0,
            ))
        }
    }

    ///
    /// # Returns
    ///
    /// the #`property::SettingTeam::runner-hwaddr-policy` property of the setting
    #[cfg(any(feature = "v1_12", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_12")))]
    #[doc(alias = "nm_setting_team_get_runner_hwaddr_policy")]
    #[doc(alias = "get_runner_hwaddr_policy")]
    pub fn runner_hwaddr_policy(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::nm_setting_team_get_runner_hwaddr_policy(
                self.to_glib_none().0,
            ))
        }
    }

    ///
    /// # Returns
    ///
    /// the #`property::SettingTeam::runner-min-ports` property of the setting
    #[cfg(any(feature = "v1_12", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_12")))]
    #[doc(alias = "nm_setting_team_get_runner_min_ports")]
    #[doc(alias = "get_runner_min_ports")]
    pub fn runner_min_ports(&self) -> i32 {
        unsafe { ffi::nm_setting_team_get_runner_min_ports(self.to_glib_none().0) }
    }

    ///
    /// # Returns
    ///
    /// the #`property::SettingTeam::runner-sys-prio` property of the setting
    #[cfg(any(feature = "v1_12", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_12")))]
    #[doc(alias = "nm_setting_team_get_runner_sys_prio")]
    #[doc(alias = "get_runner_sys_prio")]
    pub fn runner_sys_prio(&self) -> i32 {
        unsafe { ffi::nm_setting_team_get_runner_sys_prio(self.to_glib_none().0) }
    }

    ///
    /// # Returns
    ///
    /// the #`property::SettingTeam::runner-tx-balancer` property of the setting
    #[cfg(any(feature = "v1_12", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_12")))]
    #[doc(alias = "nm_setting_team_get_runner_tx_balancer")]
    #[doc(alias = "get_runner_tx_balancer")]
    pub fn runner_tx_balancer(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::nm_setting_team_get_runner_tx_balancer(
                self.to_glib_none().0,
            ))
        }
    }

    ///
    /// # Returns
    ///
    /// the #`property::SettingTeam::runner-tx-balancer_interval` property of the setting
    #[cfg(any(feature = "v1_12", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_12")))]
    #[doc(alias = "nm_setting_team_get_runner_tx_balancer_interval")]
    #[doc(alias = "get_runner_tx_balancer_interval")]
    pub fn runner_tx_balancer_interval(&self) -> i32 {
        unsafe { ffi::nm_setting_team_get_runner_tx_balancer_interval(self.to_glib_none().0) }
    }

    #[cfg(any(feature = "v1_12", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_12")))]
    #[doc(alias = "nm_setting_team_get_runner_tx_hash")]
    #[doc(alias = "get_runner_tx_hash")]
    pub fn runner_tx_hash(&self, idx: u32) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::nm_setting_team_get_runner_tx_hash(
                self.to_glib_none().0,
                idx,
            ))
        }
    }

    /// Removes the link watcher at index `idx`.
    /// ## `idx`
    /// index number of the link watcher to remove
    #[cfg(any(feature = "v1_12", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_12")))]
    #[doc(alias = "nm_setting_team_remove_link_watcher")]
    pub fn remove_link_watcher(&self, idx: u32) {
        unsafe {
            ffi::nm_setting_team_remove_link_watcher(self.to_glib_none().0, idx);
        }
    }

    /// Removes the link watcher entry matching link_watcher.
    /// ## `link_watcher`
    /// the link watcher to remove
    ///
    /// # Returns
    ///
    /// [`true`] if the link watcher was found and removed, [`false`] otherwise.
    #[cfg(any(feature = "v1_12", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_12")))]
    #[doc(alias = "nm_setting_team_remove_link_watcher_by_value")]
    pub fn remove_link_watcher_by_value(&self, link_watcher: &TeamLinkWatcher) -> bool {
        unsafe {
            from_glib(ffi::nm_setting_team_remove_link_watcher_by_value(
                self.to_glib_none().0,
                link_watcher.to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v1_12", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_12")))]
    #[doc(alias = "nm_setting_team_remove_runner_tx_hash")]
    pub fn remove_runner_tx_hash(&self, idx: u32) {
        unsafe {
            ffi::nm_setting_team_remove_runner_tx_hash(self.to_glib_none().0, idx);
        }
    }

    /// Removes the txhash element `txhash`
    /// ## `txhash`
    /// the txhash element to remove
    ///
    /// # Returns
    ///
    /// [`true`] if the txhash element was found and removed; [`false`] if it was not.
    #[cfg(any(feature = "v1_12", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_12")))]
    #[doc(alias = "nm_setting_team_remove_runner_tx_hash_by_value")]
    pub fn remove_runner_tx_hash_by_value(&self, txhash: &str) -> bool {
        unsafe {
            from_glib(ffi::nm_setting_team_remove_runner_tx_hash_by_value(
                self.to_glib_none().0,
                txhash.to_glib_none().0,
            ))
        }
    }

    /// The JSON configuration for the team network interface. The property
    /// should contain raw JSON configuration data suitable for teamd, because
    /// the value is passed directly to teamd. If not specified, the default
    /// configuration is used. See man teamd.conf for the format details.
    pub fn set_config(&self, config: Option<&str>) {
        glib::ObjectExt::set_property(self, "config", &config)
    }

    /// Corresponds to the teamd mcast_rejoin.count.
    #[cfg(any(feature = "v1_12", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_12")))]
    #[doc(alias = "mcast-rejoin-count")]
    pub fn set_mcast_rejoin_count(&self, mcast_rejoin_count: i32) {
        glib::ObjectExt::set_property(self, "mcast-rejoin-count", &mcast_rejoin_count)
    }

    /// Corresponds to the teamd mcast_rejoin.interval.
    #[cfg(any(feature = "v1_12", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_12")))]
    #[doc(alias = "mcast-rejoin-interval")]
    pub fn set_mcast_rejoin_interval(&self, mcast_rejoin_interval: i32) {
        glib::ObjectExt::set_property(self, "mcast-rejoin-interval", &mcast_rejoin_interval)
    }

    /// Corresponds to the teamd notify_peers.count.
    #[cfg(any(feature = "v1_12", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_12")))]
    #[doc(alias = "notify-peers-count")]
    pub fn set_notify_peers_count(&self, notify_peers_count: i32) {
        glib::ObjectExt::set_property(self, "notify-peers-count", &notify_peers_count)
    }

    /// Corresponds to the teamd notify_peers.interval.
    #[cfg(any(feature = "v1_12", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_12")))]
    #[doc(alias = "notify-peers-interval")]
    pub fn set_notify_peers_interval(&self, notify_peers_interval: i32) {
        glib::ObjectExt::set_property(self, "notify-peers-interval", &notify_peers_interval)
    }

    /// Corresponds to the teamd runner.name.
    /// Permitted values are: "roundrobin", "broadcast", "activebackup",
    /// "loadbalance", "lacp", "random".
    #[cfg(any(feature = "v1_12", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_12")))]
    pub fn set_runner(&self, runner: Option<&str>) {
        glib::ObjectExt::set_property(self, "runner", &runner)
    }

    /// Corresponds to the teamd runner.active.
    #[cfg(any(feature = "v1_12", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_12")))]
    #[doc(alias = "runner-active")]
    pub fn set_runner_active(&self, runner_active: bool) {
        glib::ObjectExt::set_property(self, "runner-active", &runner_active)
    }

    /// Corresponds to the teamd runner.agg_select_policy.
    #[cfg(any(feature = "v1_12", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_12")))]
    #[doc(alias = "runner-agg-select-policy")]
    pub fn set_runner_agg_select_policy(&self, runner_agg_select_policy: Option<&str>) {
        glib::ObjectExt::set_property(self, "runner-agg-select-policy", &runner_agg_select_policy)
    }

    /// Corresponds to the teamd runner.fast_rate.
    #[cfg(any(feature = "v1_12", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_12")))]
    #[doc(alias = "runner-fast-rate")]
    pub fn set_runner_fast_rate(&self, runner_fast_rate: bool) {
        glib::ObjectExt::set_property(self, "runner-fast-rate", &runner_fast_rate)
    }

    /// Corresponds to the teamd runner.hwaddr_policy.
    #[cfg(any(feature = "v1_12", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_12")))]
    #[doc(alias = "runner-hwaddr-policy")]
    pub fn set_runner_hwaddr_policy(&self, runner_hwaddr_policy: Option<&str>) {
        glib::ObjectExt::set_property(self, "runner-hwaddr-policy", &runner_hwaddr_policy)
    }

    /// Corresponds to the teamd runner.min_ports.
    #[cfg(any(feature = "v1_12", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_12")))]
    #[doc(alias = "runner-min-ports")]
    pub fn set_runner_min_ports(&self, runner_min_ports: i32) {
        glib::ObjectExt::set_property(self, "runner-min-ports", &runner_min_ports)
    }

    /// Corresponds to the teamd runner.sys_prio.
    #[cfg(any(feature = "v1_12", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_12")))]
    #[doc(alias = "runner-sys-prio")]
    pub fn set_runner_sys_prio(&self, runner_sys_prio: i32) {
        glib::ObjectExt::set_property(self, "runner-sys-prio", &runner_sys_prio)
    }

    /// Corresponds to the teamd runner.tx_balancer.name.
    #[cfg(any(feature = "v1_12", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_12")))]
    #[doc(alias = "runner-tx-balancer")]
    pub fn set_runner_tx_balancer(&self, runner_tx_balancer: Option<&str>) {
        glib::ObjectExt::set_property(self, "runner-tx-balancer", &runner_tx_balancer)
    }

    /// Corresponds to the teamd runner.tx_balancer.interval.
    #[cfg(any(feature = "v1_12", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_12")))]
    #[doc(alias = "runner-tx-balancer-interval")]
    pub fn set_runner_tx_balancer_interval(&self, runner_tx_balancer_interval: i32) {
        glib::ObjectExt::set_property(
            self,
            "runner-tx-balancer-interval",
            &runner_tx_balancer_interval,
        )
    }

    /// Corresponds to the teamd runner.tx_hash.
    #[cfg(any(feature = "v1_12", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_12")))]
    #[doc(alias = "runner-tx-hash")]
    pub fn set_runner_tx_hash(&self, runner_tx_hash: &[&str]) {
        glib::ObjectExt::set_property(self, "runner-tx-hash", &runner_tx_hash)
    }

    #[doc(alias = "config")]
    pub fn connect_config_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_config_trampoline<F: Fn(&SettingTeam) + 'static>(
            this: *mut ffi::NMSettingTeam,
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
                b"notify::config\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_config_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v1_12", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_12")))]
    #[doc(alias = "mcast-rejoin-count")]
    pub fn connect_mcast_rejoin_count_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_mcast_rejoin_count_trampoline<F: Fn(&SettingTeam) + 'static>(
            this: *mut ffi::NMSettingTeam,
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
                b"notify::mcast-rejoin-count\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_mcast_rejoin_count_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v1_12", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_12")))]
    #[doc(alias = "mcast-rejoin-interval")]
    pub fn connect_mcast_rejoin_interval_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_mcast_rejoin_interval_trampoline<
            F: Fn(&SettingTeam) + 'static,
        >(
            this: *mut ffi::NMSettingTeam,
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
                b"notify::mcast-rejoin-interval\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_mcast_rejoin_interval_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v1_12", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_12")))]
    #[doc(alias = "notify-peers-count")]
    pub fn connect_notify_peers_count_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_notify_peers_count_trampoline<F: Fn(&SettingTeam) + 'static>(
            this: *mut ffi::NMSettingTeam,
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
                b"notify::notify-peers-count\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_notify_peers_count_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v1_12", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_12")))]
    #[doc(alias = "notify-peers-interval")]
    pub fn connect_notify_peers_interval_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_notify_peers_interval_trampoline<
            F: Fn(&SettingTeam) + 'static,
        >(
            this: *mut ffi::NMSettingTeam,
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
                b"notify::notify-peers-interval\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_notify_peers_interval_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v1_12", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_12")))]
    #[doc(alias = "runner")]
    pub fn connect_runner_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_runner_trampoline<F: Fn(&SettingTeam) + 'static>(
            this: *mut ffi::NMSettingTeam,
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
                b"notify::runner\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_runner_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v1_12", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_12")))]
    #[doc(alias = "runner-active")]
    pub fn connect_runner_active_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_runner_active_trampoline<F: Fn(&SettingTeam) + 'static>(
            this: *mut ffi::NMSettingTeam,
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
                b"notify::runner-active\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_runner_active_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v1_12", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_12")))]
    #[doc(alias = "runner-agg-select-policy")]
    pub fn connect_runner_agg_select_policy_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_runner_agg_select_policy_trampoline<
            F: Fn(&SettingTeam) + 'static,
        >(
            this: *mut ffi::NMSettingTeam,
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
                b"notify::runner-agg-select-policy\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_runner_agg_select_policy_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v1_12", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_12")))]
    #[doc(alias = "runner-fast-rate")]
    pub fn connect_runner_fast_rate_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_runner_fast_rate_trampoline<F: Fn(&SettingTeam) + 'static>(
            this: *mut ffi::NMSettingTeam,
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
                b"notify::runner-fast-rate\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_runner_fast_rate_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v1_12", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_12")))]
    #[doc(alias = "runner-hwaddr-policy")]
    pub fn connect_runner_hwaddr_policy_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_runner_hwaddr_policy_trampoline<
            F: Fn(&SettingTeam) + 'static,
        >(
            this: *mut ffi::NMSettingTeam,
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
                b"notify::runner-hwaddr-policy\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_runner_hwaddr_policy_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v1_12", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_12")))]
    #[doc(alias = "runner-min-ports")]
    pub fn connect_runner_min_ports_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_runner_min_ports_trampoline<F: Fn(&SettingTeam) + 'static>(
            this: *mut ffi::NMSettingTeam,
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
                b"notify::runner-min-ports\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_runner_min_ports_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v1_12", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_12")))]
    #[doc(alias = "runner-sys-prio")]
    pub fn connect_runner_sys_prio_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_runner_sys_prio_trampoline<F: Fn(&SettingTeam) + 'static>(
            this: *mut ffi::NMSettingTeam,
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
                b"notify::runner-sys-prio\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_runner_sys_prio_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v1_12", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_12")))]
    #[doc(alias = "runner-tx-balancer")]
    pub fn connect_runner_tx_balancer_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_runner_tx_balancer_trampoline<F: Fn(&SettingTeam) + 'static>(
            this: *mut ffi::NMSettingTeam,
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
                b"notify::runner-tx-balancer\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_runner_tx_balancer_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v1_12", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_12")))]
    #[doc(alias = "runner-tx-balancer-interval")]
    pub fn connect_runner_tx_balancer_interval_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_runner_tx_balancer_interval_trampoline<
            F: Fn(&SettingTeam) + 'static,
        >(
            this: *mut ffi::NMSettingTeam,
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
                b"notify::runner-tx-balancer-interval\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_runner_tx_balancer_interval_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v1_12", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_12")))]
    #[doc(alias = "runner-tx-hash")]
    pub fn connect_runner_tx_hash_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_runner_tx_hash_trampoline<F: Fn(&SettingTeam) + 'static>(
            this: *mut ffi::NMSettingTeam,
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
                b"notify::runner-tx-hash\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_runner_tx_hash_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl Default for SettingTeam {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for SettingTeam {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("SettingTeam")
    }
}
