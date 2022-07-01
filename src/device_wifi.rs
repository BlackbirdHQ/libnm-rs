// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files
// DO NOT EDIT

use crate::AccessPoint;
use crate::Device;
use crate::DeviceWifiCapabilities;
use crate::Object;
use crate::_80211Mode;
use glib::object::IsA;
use glib::object::ObjectType as ObjectType_;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::StaticType;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;
use std::pin::Pin;
use std::ptr;

glib::wrapper! {
    #[doc(alias = "NMDeviceWifi")]
    pub struct DeviceWifi(Object<ffi::NMDeviceWifi, ffi::NMDeviceWifiClass>) @extends Device, Object;

    match fn {
        type_ => || ffi::nm_device_wifi_get_type(),
    }
}

impl DeviceWifi {
    /// Gets a [`AccessPoint`][crate::AccessPoint] by path.
    /// ## `path`
    /// the object path of the access point
    ///
    /// # Returns
    ///
    /// the access point or [`None`] if none is found.
    #[doc(alias = "nm_device_wifi_get_access_point_by_path")]
    #[doc(alias = "get_access_point_by_path")]
    pub fn access_point_by_path(&self, path: &str) -> Option<AccessPoint> {
        unsafe {
            from_glib_none(ffi::nm_device_wifi_get_access_point_by_path(
                self.to_glib_none().0,
                path.to_glib_none().0,
            ))
        }
    }

    /// Gets all the scanned access points of the [`DeviceWifi`][crate::DeviceWifi].
    ///
    /// # Returns
    ///
    /// a [`glib::PtrArray`][crate::glib::PtrArray] containing all the
    /// scanned `NMAccessPoints`.
    /// The returned array is owned by the client and should not be modified.
    #[doc(alias = "nm_device_wifi_get_access_points")]
    #[doc(alias = "get_access_points")]
    pub fn access_points(&self) -> Vec<AccessPoint> {
        unsafe {
            FromGlibPtrContainer::from_glib_none(ffi::nm_device_wifi_get_access_points(
                self.to_glib_none().0,
            ))
        }
    }

    /// Gets the active [`AccessPoint`][crate::AccessPoint].
    ///
    /// # Returns
    ///
    /// the access point or [`None`] if none is active
    #[doc(alias = "nm_device_wifi_get_active_access_point")]
    #[doc(alias = "get_active_access_point")]
    pub fn active_access_point(&self) -> Option<AccessPoint> {
        unsafe {
            from_glib_none(ffi::nm_device_wifi_get_active_access_point(
                self.to_glib_none().0,
            ))
        }
    }

    /// Gets the bit rate of the [`DeviceWifi`][crate::DeviceWifi] in kbit/s.
    ///
    /// # Returns
    ///
    /// the bit rate (kbit/s)
    #[doc(alias = "nm_device_wifi_get_bitrate")]
    #[doc(alias = "get_bitrate")]
    pub fn bitrate(&self) -> u32 {
        unsafe { ffi::nm_device_wifi_get_bitrate(self.to_glib_none().0) }
    }

    /// Gets the Wi-Fi capabilities of the [`DeviceWifi`][crate::DeviceWifi].
    ///
    /// # Returns
    ///
    /// the capabilities
    #[doc(alias = "nm_device_wifi_get_capabilities")]
    #[doc(alias = "get_capabilities")]
    pub fn capabilities(&self) -> DeviceWifiCapabilities {
        unsafe { from_glib(ffi::nm_device_wifi_get_capabilities(self.to_glib_none().0)) }
    }

    /// Returns the timestamp (in CLOCK_BOOTTIME milliseconds) for the last finished
    /// network scan. A value of -1 means the device never scanned for access points.
    ///
    /// Use [`utils_get_timestamp_msec()`][crate::utils_get_timestamp_msec()] to obtain current time value suitable for
    /// comparing to this value.
    ///
    /// # Returns
    ///
    /// the last scan time in milliseconds (in clock_gettime(CLOCK_BOOTTIME) scale).
    #[cfg(any(feature = "v1_12", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_12")))]
    #[doc(alias = "nm_device_wifi_get_last_scan")]
    #[doc(alias = "get_last_scan")]
    pub fn last_scan(&self) -> i64 {
        unsafe { ffi::nm_device_wifi_get_last_scan(self.to_glib_none().0) }
    }

    /// Gets the [`DeviceWifi`][crate::DeviceWifi] mode.
    ///
    /// # Returns
    ///
    /// the mode
    #[doc(alias = "nm_device_wifi_get_mode")]
    #[doc(alias = "get_mode")]
    pub fn mode(&self) -> _80211Mode {
        unsafe { from_glib(ffi::nm_device_wifi_get_mode(self.to_glib_none().0)) }
    }

    /// Gets the permanent hardware (MAC) address of the [`DeviceWifi`][crate::DeviceWifi]
    ///
    /// # Returns
    ///
    /// the permanent hardware address. This is the internal string used by the
    /// device, and must not be modified.
    #[doc(alias = "nm_device_wifi_get_permanent_hw_address")]
    #[doc(alias = "get_permanent_hw_address")]
    pub fn permanent_hw_address(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::nm_device_wifi_get_permanent_hw_address(
                self.to_glib_none().0,
            ))
        }
    }

    /// Request NM to scan for access points on `self`. Note that the function
    /// returns immediately after requesting the scan, and it may take some time
    /// after that for the scan to complete.
    ///
    /// # Deprecated since 1.22
    ///
    /// Use [`request_scan_async()`][Self::request_scan_async()] or GDBusConnection.
    /// ## `cancellable`
    /// a [`gio::Cancellable`][crate::gio::Cancellable], or [`None`]
    ///
    /// # Returns
    ///
    /// [`true`] on success, [`false`] on error, in which case `error` will be
    /// set.
    #[cfg_attr(feature = "v1_22", deprecated = "Since 1.22")]
    #[doc(alias = "nm_device_wifi_request_scan")]
    pub fn request_scan(
        &self,
        cancellable: Option<&impl IsA<gio::Cancellable>>,
    ) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let is_ok = ffi::nm_device_wifi_request_scan(
                self.to_glib_none().0,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                &mut error,
            );
            assert_eq!(is_ok == glib::ffi::GFALSE, !error.is_null());
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    /// Request NM to scan for access points on `self`. Note that `callback` will be
    /// called immediately after requesting the scan, and it may take some time after
    /// that for the scan to complete.
    /// ## `cancellable`
    /// a [`gio::Cancellable`][crate::gio::Cancellable], or [`None`]
    /// ## `callback`
    /// callback to be called when the scan has been requested
    #[doc(alias = "nm_device_wifi_request_scan_async")]
    pub fn request_scan_async<P: FnOnce(Result<(), glib::Error>) + 'static>(
        &self,
        cancellable: Option<&impl IsA<gio::Cancellable>>,
        callback: P,
    ) {
        let main_context = glib::MainContext::ref_thread_default();
        let is_main_context_owner = main_context.is_owner();
        let has_acquired_main_context = (!is_main_context_owner)
            .then(|| main_context.acquire().ok())
            .flatten();
        assert!(
            is_main_context_owner || has_acquired_main_context.is_some(),
            "Async operations only allowed if the thread is owning the MainContext"
        );

        let user_data: Box_<glib::thread_guard::ThreadGuard<P>> =
            Box_::new(glib::thread_guard::ThreadGuard::new(callback));
        unsafe extern "C" fn request_scan_async_trampoline<
            P: FnOnce(Result<(), glib::Error>) + 'static,
        >(
            _source_object: *mut glib::gobject_ffi::GObject,
            res: *mut gio::ffi::GAsyncResult,
            user_data: glib::ffi::gpointer,
        ) {
            let mut error = ptr::null_mut();
            let _ =
                ffi::nm_device_wifi_request_scan_finish(_source_object as *mut _, res, &mut error);
            let result = if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            };
            let callback: Box_<glib::thread_guard::ThreadGuard<P>> =
                Box_::from_raw(user_data as *mut _);
            let callback: P = callback.into_inner();
            callback(result);
        }
        let callback = request_scan_async_trampoline::<P>;
        unsafe {
            ffi::nm_device_wifi_request_scan_async(
                self.to_glib_none().0,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                Some(callback),
                Box_::into_raw(user_data) as *mut _,
            );
        }
    }

    pub fn request_scan_future(
        &self,
    ) -> Pin<Box_<dyn std::future::Future<Output = Result<(), glib::Error>> + 'static>> {
        Box_::pin(gio::GioFuture::new(self, move |obj, cancellable, send| {
            obj.request_scan_async(Some(cancellable), move |res| {
                send.resolve(res);
            });
        }))
    }

    /// Request NM to scan for access points on `self`. Note that the function
    /// returns immediately after requesting the scan, and it may take some time
    /// after that for the scan to complete.
    /// This is the same as [`request_scan()`][Self::request_scan()] except it accepts `options`
    /// for the scanning. The argument is the dictionary passed to RequestScan()
    /// D-Bus call. Valid options inside the dictionary are:
    /// 'ssids' => array of SSIDs (saay)
    ///
    /// # Deprecated since 1.22
    ///
    /// Use `nm_device_wifi_request_scan_options_async()` or GDBusConnection.
    /// ## `options`
    /// dictionary with options for RequestScan(), or [`None`]
    /// ## `cancellable`
    /// a [`gio::Cancellable`][crate::gio::Cancellable], or [`None`]
    ///
    /// # Returns
    ///
    /// [`true`] on success, [`false`] on error, in which case `error` will be
    /// set.
    #[cfg_attr(feature = "v1_22", deprecated = "Since 1.22")]
    #[cfg(any(feature = "v1_2", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_2")))]
    #[doc(alias = "nm_device_wifi_request_scan_options")]
    pub fn request_scan_options(
        &self,
        options: &glib::Variant,
        cancellable: Option<&impl IsA<gio::Cancellable>>,
    ) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let is_ok = ffi::nm_device_wifi_request_scan_options(
                self.to_glib_none().0,
                options.to_glib_none().0,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                &mut error,
            );
            assert_eq!(is_ok == glib::ffi::GFALSE, !error.is_null());
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    //#[cfg(any(feature = "v1_2", feature = "dox"))]
    //#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_2")))]
    //#[doc(alias = "nm_device_wifi_request_scan_options_async")]
    //pub fn request_scan_options_async<P: FnOnce(Result<(), glib::Error>) + 'static>(&self, options: &glib::Variant, cancellable: Option<&impl IsA<gio::Cancellable>>, callback: P) {
    //    unsafe { TODO: call ffi:nm_device_wifi_request_scan_options_async() }
    //}

    /// The hardware (MAC) address of the device.
    #[doc(alias = "perm-hw-address")]
    pub fn perm_hw_address(&self) -> Option<glib::GString> {
        glib::ObjectExt::property(self, "perm-hw-address")
    }

    /// The wireless capabilities of the device.
    #[doc(alias = "wireless-capabilities")]
    pub fn wireless_capabilities(&self) -> DeviceWifiCapabilities {
        glib::ObjectExt::property(self, "wireless-capabilities")
    }

    /// Notifies that a [`AccessPoint`][crate::AccessPoint] is added to the Wi-Fi device.
    /// ## `ap`
    /// the new access point
    #[doc(alias = "access-point-added")]
    pub fn connect_access_point_added<F: Fn(&Self, &glib::Object) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn access_point_added_trampoline<
            F: Fn(&DeviceWifi, &glib::Object) + 'static,
        >(
            this: *mut ffi::NMDeviceWifi,
            ap: *mut glib::gobject_ffi::GObject,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), &from_glib_borrow(ap))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"access-point-added\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    access_point_added_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    /// Notifies that a [`AccessPoint`][crate::AccessPoint] is removed from the Wi-Fi device.
    /// ## `ap`
    /// the removed access point
    #[doc(alias = "access-point-removed")]
    pub fn connect_access_point_removed<F: Fn(&Self, &glib::Object) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn access_point_removed_trampoline<
            F: Fn(&DeviceWifi, &glib::Object) + 'static,
        >(
            this: *mut ffi::NMDeviceWifi,
            ap: *mut glib::gobject_ffi::GObject,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), &from_glib_borrow(ap))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"access-point-removed\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    access_point_removed_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "access-points")]
    pub fn connect_access_points_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_access_points_trampoline<F: Fn(&DeviceWifi) + 'static>(
            this: *mut ffi::NMDeviceWifi,
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
                b"notify::access-points\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_access_points_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "active-access-point")]
    pub fn connect_active_access_point_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_active_access_point_trampoline<F: Fn(&DeviceWifi) + 'static>(
            this: *mut ffi::NMDeviceWifi,
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
                b"notify::active-access-point\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_active_access_point_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "bitrate")]
    pub fn connect_bitrate_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_bitrate_trampoline<F: Fn(&DeviceWifi) + 'static>(
            this: *mut ffi::NMDeviceWifi,
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
                b"notify::bitrate\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_bitrate_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v1_12", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_12")))]
    #[doc(alias = "last-scan")]
    pub fn connect_last_scan_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_last_scan_trampoline<F: Fn(&DeviceWifi) + 'static>(
            this: *mut ffi::NMDeviceWifi,
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
                b"notify::last-scan\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_last_scan_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "mode")]
    pub fn connect_mode_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_mode_trampoline<F: Fn(&DeviceWifi) + 'static>(
            this: *mut ffi::NMDeviceWifi,
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
                b"notify::mode\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_mode_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "perm-hw-address")]
    pub fn connect_perm_hw_address_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_perm_hw_address_trampoline<F: Fn(&DeviceWifi) + 'static>(
            this: *mut ffi::NMDeviceWifi,
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
                b"notify::perm-hw-address\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_perm_hw_address_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "wireless-capabilities")]
    pub fn connect_wireless_capabilities_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_wireless_capabilities_trampoline<
            F: Fn(&DeviceWifi) + 'static,
        >(
            this: *mut ffi::NMDeviceWifi,
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
                b"notify::wireless-capabilities\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_wireless_capabilities_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for DeviceWifi {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("DeviceWifi")
    }
}
