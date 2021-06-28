// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files
// DO NOT EDIT

use crate::Device;
use crate::Object;
use crate::WifiP2PPeer;
use glib::object::IsA;
use glib::object::ObjectType as ObjectType_;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;
use std::pin::Pin;
use std::ptr;

glib::wrapper! {
    #[doc(alias = "NMDeviceWifiP2P")]
    pub struct DeviceWifiP2P(Object<ffi::NMDeviceWifiP2P, ffi::NMDeviceWifiP2PClass>) @extends Device, Object;

    match fn {
        type_ => || ffi::nm_device_wifi_p2p_get_type(),
    }
}

impl DeviceWifiP2P {
    /// Gets a [`WifiP2PPeer`][crate::WifiP2PPeer] by path.
    /// ## `path`
    /// the object path of the peer
    ///
    /// # Returns
    ///
    /// the peer or [`None`] if none is found.
    #[doc(alias = "nm_device_wifi_p2p_get_peer_by_path")]
    #[doc(alias = "get_peer_by_path")]
    pub fn peer_by_path(&self, path: &str) -> Option<WifiP2PPeer> {
        unsafe {
            from_glib_none(ffi::nm_device_wifi_p2p_get_peer_by_path(
                self.to_glib_none().0,
                path.to_glib_none().0,
            ))
        }
    }

    /// Gets all the found peers of the [`DeviceWifiP2P`][crate::DeviceWifiP2P].
    ///
    /// # Returns
    ///
    /// a [`glib::PtrArray`][crate::glib::PtrArray] containing all the
    ///  found `NMWifiP2PPeers`.
    /// The returned array is owned by the client and should not be modified.
    #[doc(alias = "nm_device_wifi_p2p_get_peers")]
    #[doc(alias = "get_peers")]
    pub fn peers(&self) -> Vec<WifiP2PPeer> {
        unsafe {
            FromGlibPtrContainer::from_glib_none(ffi::nm_device_wifi_p2p_get_peers(
                self.to_glib_none().0,
            ))
        }
    }

    /// Request NM to search for Wi-Fi P2P peers on `self`. Note that the call
    /// returns immediately after requesting the find, and it may take some time
    /// after that for peers to be found.
    ///
    /// The find operation will run for 30s by default. You can stop it earlier
    /// using `nm_device_p2p_wifi_stop_find()`.
    /// ## `options`
    /// optional options passed to StartFind.
    /// ## `cancellable`
    /// a [`gio::Cancellable`][crate::gio::Cancellable], or [`None`]
    /// ## `callback`
    /// a `GAsyncReadyCallback`, or [`None`]
    #[doc(alias = "nm_device_wifi_p2p_start_find")]
    pub fn start_find<
        P: IsA<gio::Cancellable>,
        Q: FnOnce(Result<(), glib::Error>) + Send + 'static,
    >(
        &self,
        options: Option<&glib::Variant>,
        cancellable: Option<&P>,
        callback: Q,
    ) {
        let user_data: Box_<Q> = Box_::new(callback);
        unsafe extern "C" fn start_find_trampoline<
            Q: FnOnce(Result<(), glib::Error>) + Send + 'static,
        >(
            _source_object: *mut glib::gobject_ffi::GObject,
            res: *mut gio::ffi::GAsyncResult,
            user_data: glib::ffi::gpointer,
        ) {
            let mut error = ptr::null_mut();
            let _ = ffi::nm_device_wifi_p2p_start_find_finish(
                _source_object as *mut _,
                res,
                &mut error,
            );
            let result = if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            };
            let callback: Box_<Q> = Box_::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = start_find_trampoline::<Q>;
        unsafe {
            ffi::nm_device_wifi_p2p_start_find(
                self.to_glib_none().0,
                options.to_glib_none().0,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                Some(callback),
                Box_::into_raw(user_data) as *mut _,
            );
        }
    }

    pub fn start_find_future(
        &self,
        options: Option<&glib::Variant>,
    ) -> Pin<Box_<dyn std::future::Future<Output = Result<(), glib::Error>> + 'static>> {
        let options = options.map(ToOwned::to_owned);
        Box_::pin(gio::GioFuture::new(self, move |obj, cancellable, send| {
            obj.start_find(
                options.as_ref().map(::std::borrow::Borrow::borrow),
                Some(cancellable),
                move |res| {
                    send.resolve(res);
                },
            );
        }))
    }

    /// Request NM to stop any ongoing find operation for Wi-Fi P2P peers on `self`.
    /// ## `cancellable`
    /// a [`gio::Cancellable`][crate::gio::Cancellable], or [`None`]
    /// ## `callback`
    /// a `GAsyncReadyCallback`, or [`None`]
    #[doc(alias = "nm_device_wifi_p2p_stop_find")]
    pub fn stop_find<
        P: IsA<gio::Cancellable>,
        Q: FnOnce(Result<(), glib::Error>) + Send + 'static,
    >(
        &self,
        cancellable: Option<&P>,
        callback: Q,
    ) {
        let user_data: Box_<Q> = Box_::new(callback);
        unsafe extern "C" fn stop_find_trampoline<
            Q: FnOnce(Result<(), glib::Error>) + Send + 'static,
        >(
            _source_object: *mut glib::gobject_ffi::GObject,
            res: *mut gio::ffi::GAsyncResult,
            user_data: glib::ffi::gpointer,
        ) {
            let mut error = ptr::null_mut();
            let _ =
                ffi::nm_device_wifi_p2p_stop_find_finish(_source_object as *mut _, res, &mut error);
            let result = if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            };
            let callback: Box_<Q> = Box_::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = stop_find_trampoline::<Q>;
        unsafe {
            ffi::nm_device_wifi_p2p_stop_find(
                self.to_glib_none().0,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                Some(callback),
                Box_::into_raw(user_data) as *mut _,
            );
        }
    }

    pub fn stop_find_future(
        &self,
    ) -> Pin<Box_<dyn std::future::Future<Output = Result<(), glib::Error>> + 'static>> {
        Box_::pin(gio::GioFuture::new(self, move |obj, cancellable, send| {
            obj.stop_find(Some(cancellable), move |res| {
                send.resolve(res);
            });
        }))
    }

    /// Notifies that a [`WifiP2PPeer`][crate::WifiP2PPeer] is added to the Wi-Fi P2P device.
    /// ## `peer`
    /// the new access point
    #[cfg(any(feature = "v1_16", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_16")))]
    #[doc(alias = "peer-added")]
    pub fn connect_peer_added<F: Fn(&Self, &glib::Object) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn peer_added_trampoline<
            F: Fn(&DeviceWifiP2P, &glib::Object) + 'static,
        >(
            this: *mut ffi::NMDeviceWifiP2P,
            peer: *mut glib::gobject_ffi::GObject,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), &from_glib_borrow(peer))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"peer-added\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    peer_added_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    /// Notifies that a [`WifiP2PPeer`][crate::WifiP2PPeer] is removed from the Wi-Fi P2P device.
    /// ## `peer`
    /// the removed access point
    #[cfg(any(feature = "v1_16", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_16")))]
    #[doc(alias = "peer-removed")]
    pub fn connect_peer_removed<F: Fn(&Self, &glib::Object) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn peer_removed_trampoline<
            F: Fn(&DeviceWifiP2P, &glib::Object) + 'static,
        >(
            this: *mut ffi::NMDeviceWifiP2P,
            peer: *mut glib::gobject_ffi::GObject,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), &from_glib_borrow(peer))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"peer-removed\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    peer_removed_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v1_16", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_16")))]
    #[doc(alias = "peers")]
    pub fn connect_peers_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_peers_trampoline<F: Fn(&DeviceWifiP2P) + 'static>(
            this: *mut ffi::NMDeviceWifiP2P,
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
                b"notify::peers\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_peers_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for DeviceWifiP2P {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("DeviceWifiP2P")
    }
}
